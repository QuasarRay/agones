use std::{collections::HashMap,num::NonZeroUsize,pin::Pin,time::Duration};
use agones_core::{Fleet,FleetAutoscaler as FA,GameServer as GS,GameServerSet as GSS,GameServerState as ST,IntOrPercent as IP};
use chrono::{DateTime,Utc};use futures::{Stream,StreamExt};use lru::LruCache;
use super::{controller_metrics::{Metric as M,StateMetrics as SM},GameServerCount as GC};

type R<T>=Result<T,String>;
const P:Duration=Duration::from_secs(15);
const CAP:usize=600_000;

fn st(g:&GS) -> &str {
    g.status.state.as_ref().map_or("",ST::as_str)
}

fn fl(g:&GS) -> &str {
    g.metadata.labels.get("agones.dev/fleet")
        .filter(|s| !s.is_empty())
        .map_or("none", String::as_str)
}

pub type Node=(String,Vec<String>);

pub enum Event{F(Fleet),FD,A(Option<FA>,FA),AD,G(GS,GS)}

#[ allow(async_fn_in_trait) ]
pub trait Src:Send+Sync+'static {

    async fn rd(&self)->R<()>{
        Ok(())
    }

    async fn fl(&self) -> R<Vec<Fleet>>;
    async fn fa(&self) -> R<Vec<FA>>;
    async fn gs(&self) -> R<Vec<GS>>;
    async fn nd(&self) -> R<Vec<Node>>;
    async fn gss(&self, f:&Fleet) -> R<Vec<GSS>>;

    fn ev(&self) -> Pin<Box<dyn Stream<Item=Event>+Send>>;
}

pub struct Controller<S> {
    s:S,
    m:SM,
    g:GC,
    t:LruCache<String,f64>,
    p:bool,
    n:fn()->DateTime<Utc>
}

impl<S:Src> Controller<S>{
    pub fn new(s:S,m:SM,g:GC,p:bool) -> Self {
        Self {
            s, m, g, t:LruCache::new(NonZeroUsize::new(CAP).unwrap()),p,n:Utc::now
        }
    }

    pub async fn run(&mut self)->R<()>{
    
        self.s.rd().await?;

        let(mut e,mut t) = (self.s.ev(),tokio::time::interval(P));
    
        loop {
            tokio::select!{
                _ = t.tick() => self.c()
                
                .await,
                
                x=e.next()
                
                => match x {
                    Some(x) => self.e(x).await,None=>return Ok(())
                }
            }
        }
    }

    async fn e(&mut self,e:Event){
        match e{
            Event::F(f) => self.f(&f).await,
            Event::FD => if let Err(e)=self.r(true).await{eprintln!("{e}")},
            Event::A(o,a) => {
                if o.as_ref().is_some_and(|o|o.spec.fleet_name!=a.spec.fleet_name){
                    if let Err(e) = self.r(false).await{ eprintln!("{e}") }
                }
                self.a(&a)
            }
            Event::AD => if let Err(e) = self.r(false).await{ eprintln!("{e}") },
            Event::G(a,b) => self.g(&a,&b),
        }
    }

    fn a(&self,a:&FA){
        if a.metadata.deletion_timestamp.is_some(){return}

        let l=[a.metadata.name.as_str(),a.spec.fleet_name.as_str(),a.metadata.namespace.as_str()];

        for(k,v)in[
            (M::AC,a.status.current_replicas as f64),
            (M::AD,a.status.desired_replicas as f64),
            (M::AA,a.status.able_to_scale as u8 as f64),
            (M::AL,a.status.scaling_limited as u8 as f64)
            ]
        {
            self.m.s(k,&l,v)
        }
        if let Some(p)=&a.spec.policy.buffer{

            for (t,v) in [ ("max",p.max_replicas),("min",p.min_replicas) ] {
                self.m.s(M::AB, &[l[0], t,l[1], l[2]], v as f64)
            }

            let z = match&p.buffer_size {
                IP::Int(v) => Some(("count",*v)),
                IP::String(s) => s.as_bytes()
                    .split_last()
                    .and_then(
                        |(_,x)| std::str::from_utf8(x)
                        .ok()?
                        .parse()
                        .ok()
                    )
                    .map( |v| ("percentage",v) )
            };

            if let Some((t,v))=z{self.m.s(M::AS,&[l[0],t,l[1],l[2]],v as f64)}
        }
    }

    async fn f(&self,f:&Fleet){
        if f.metadata.deletion_timestamp.is_some(){return}let(n,ns)=(&*f.metadata.name,&*f.metadata.namespace);
        for(t,v)in[("total",f.status.replicas),("allocated",f.status.allocated_replicas),("ready",f.status.ready_replicas),("desired",f.spec.replicas),("reserved",f.status.reserved_replicas)]{self.m.s(M::R,&[n,t,ns],v as f64)}
        match self.s.gss(f).await {
            Ok(s) => {
                let c = s.iter()
                .filter(|s|s.spec.template==f.spec.template)
                .last()
                .map_or(0,|s|s.status.replicas);
            
                for (t,v) in [
                        ("current_replicas", c),
                        ("desired_replicas", f.spec.replicas)
                        ]
                {
                    self.m.s(M::O,&[n,t,ns],v as f64)
                }
            }

            Err(e) => eprintln!("{e}"),
        }

        for (k,s) in &f.status.counters{
            self.x(M::C, n, ns, k, [
                s.allocated_count,
                s.allocated_capacity,
                s.count,
                s.capacity
                ]
            )
        }

        for (k,s) in &f.status.lists{
            self.x(M::L, n, ns ,k , [
                s.allocated_count,
                s.allocated_capacity,
                s.count,
                s.capacity
                ]
            )
        }

    }
    fn x(&self,m:M,n:&str,ns:&str,k:&str,v:[i64;4]) {
        
        for(t,v) in [
            "allocated_count",
            "allocated_capacity",
            "total_count",
            "total_capacity"].into_iter().zip(v)
        {
            self.m.s(m,&[n,ns,t,k],v as f64)
        }
    }

    fn g( &mut self, a:&GS, b:&GS) {
        let (f,ns) = (fl(b), b.metadata.namespace.as_str());
        if self.p {
            if let(Some(x),Some(y)) = (&a.status.players,&b.status.players) {
                let l=[f,b.metadata.name.as_str(),ns];
        
                if x.count!=y.count {
                    self.m.s(M::P,&l,y.count as f64)
                }
        
                if x.capacity-x.count != y.capacity-y.count {
                    self.m.s(M::Q,&l,(y.capacity-y.count)as f64)
                }
            }
        }
        
        if a.status.state==b.status.state { return }
        
        self.m.i(M::T, &[st(b), f, ns]);
        
        if matches!(b.status.state.as_ref(), Some(ST::Allocated)) {
            self.m.i(M::X,&[f,ns])
        }

        match self.d(a,b) {
            Ok(d) => self.m.o(M::D, &[st(a), f, ns], d),
            Err(e) => eprintln!("{e}")
        }
    }

    fn d(&mut self, a:&GS, b:&GS)->R<f64> {
        
        let c = b.metadata.extensions
            .get("creationTimestamp")
            .and_then(serde_json::Value::as_str)
            .ok_or("GS creationTimestamp missing")?;
        
        let c = DateTime::parse_from_rfc3339(c)
            .map_err(|e|e.to_string())?
            .with_timezone(&Utc);
        
        let n = ((self.n)()-c)
            .num_nanoseconds()
            .ok_or("duration overflow")? as f64/1e9;
        
        let f=fl(b);
        
        let k = |g:&GS| format!(
            "{}/{}/{}/{}",
            g.metadata.namespace,
            f,
            g.metadata.name,st(g)
        );
        
        let d = if matches!( b.status.state.as_ref(),
            Some(ST::Creating|ST::PortAllocation)
            ) { n } else {
                n-self.t.pop(&k(a))
                .ok_or_else( || format! (
                    "unable to calculate '{}' state duration of '{}' GS",
                    st(a),a.metadata.name
                   )
                )?
        };

        if !matches!(b.status.state.as_ref(), Some(ST::Shutdown)) {
            self.t.put(k(b),n)
        }
        
        if d<0.{
            Err(format!("negative duration for '{}' state of '{}' GS",st(a),a.metadata.name))
        } else {
            Ok(d)
        }
    }
    
    async fn c(&mut self) {
        
        match self.s.gs().await {
            Ok(s) => self.g.record(&s),
            Err(e)=>eprintln!("{e}")
        }

        let s = match self.s.gs().await {
            Ok(s)=>s,
            Err(e) => {
                eprintln!("{e}");
                return
            }
        };

        let n= match self.s.nd().await {
            Ok(n) => n,
            Err(e) => {
            eprintln!("{e}");
            return
            }
        };

        let mut h = HashMap::<&str,i32>::new();

        for g in s.iter().filter(|g|!g.status.node_name.is_empty()) {
            *h.entry(&g.status.node_name).or_default() += 1
        }

        let mut z =0;
        
        for n in n.iter().filter(|n|!n.1.iter().any(|t|t.starts_with("agones.dev/"))) {
            z += 1;
            self.m.o(M::NG,&[],*h.get(n.0.as_str()).unwrap_or(&0) as f64)
        }

        self.m.s(M::N, &["true"], (z as isize-h.len() as isize) as f64);

        self.m.s(M::N, &["false"], h.len() as f64)
    }

    async fn r(&mut self,all:bool)->R<()>{
        if all{
            let(f,a) = (self.s.fl().await?,self.s.fa().await?);
            self.m.rf();
            self.g.reset();
            
            for x in &f {
                self.f(x).await
            }
            
            for x in &a {
                self.a(x)
            }
            
            match self.s.gs().await {
                Ok(s) => self.g.record(&s),
                Err(e) => eprintln!("{e}")
            }
        } else {
            let a=self.s.fa().await?;
            self.m.ra();
            for x in &a { self.a(x) }
        }
            Ok(())
    }
}
