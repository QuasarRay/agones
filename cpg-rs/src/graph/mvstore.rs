//! Read-only H2 MVStore 1.4.200 decoding for OverflowDB's three native maps.
//! Format algorithms follow the retained Chunk, DataUtils, Page and ObjectDataType
//! source. Copyright 2004-2019 H2 Group. Initial Developer: H2 Group.
//! SPDX-License-Identifier: MPL-2.0 OR EPL-1.0
use crate::{Error,Result};
use std::{collections::{BTreeMap,BTreeSet},io::Read,path::Path};
const BLOCK:usize=4096;
fn invalid(message:impl Into<String>)->Error {Error::LegacyStorage(message.into())}
#[derive(Clone,Debug)]
pub(super) enum Object {Null,Number(i64),String(String),Bytes(Vec<u8>)}
impl Object {
    pub fn string(self)->Result<String>{match self {Self::String(v)=>Ok(v),_=>Err(Error::ClassCast("expected H2 string".into()))}}
    pub fn number(self)->Result<i64>{match self {Self::Number(v)=>Ok(v),_=>Err(Error::ClassCast("expected H2 integer/long".into()))}}
    pub fn bytes(self)->Result<Vec<u8>>{match self {Self::Bytes(v)=>Ok(v),_=>Err(Error::ClassCast("expected H2 byte array".into()))}}
}
struct Cursor<'a>{bytes:&'a [u8],offset:usize}
impl<'a> Cursor<'a> {
    fn new(bytes:&'a [u8])->Self {Self {bytes,offset:0}}
    fn take(&mut self,len:usize)->Result<&'a [u8]> {
        let end=self.offset.checked_add(len).filter(|end|*end<=self.bytes.len()).ok_or_else(||invalid("truncated H2 page"))?;
        let bytes=&self.bytes[self.offset..end];self.offset=end;Ok(bytes)
    }
    fn byte(&mut self)->Result<u8>{Ok(self.take(1)?[0])}
    fn u32(&mut self)->Result<u32>{Ok(u32::from_be_bytes(self.take(4)?.try_into().expect("four bytes")))}
    fn u64(&mut self)->Result<u64>{Ok(u64::from_be_bytes(self.take(8)?.try_into().expect("eight bytes")))}
    fn var(&mut self)->Result<u64>{
        let mut value=0u64;
        for shift in (0..70).step_by(7) {let byte=self.byte()?;
            if shift==63 && byte>1 {return Err(invalid("H2 varlong overflow"));}
            value|=u64::from(byte&127)<<shift;if byte<128 {return Ok(value);}}
        Err(invalid("invalid H2 varlong"))
    }
    fn len(&mut self)->Result<usize>{usize::try_from(self.var()?).map_err(|_|invalid("H2 length overflow"))}
    fn string(&mut self,len:usize)->Result<String>{
        if len>self.bytes.len()-self.offset {return Err(invalid("H2 string length exceeds page"));}
        let mut units=Vec::with_capacity(len);
        for _ in 0..len {let first=self.byte()?;let unit=if first<128 {u16::from(first)}else if first>=0xe0 {
            (u16::from(first&15)<<12)+(u16::from(self.byte()?&63)<<6)+u16::from(self.byte()?&63)
        }else{(u16::from(first&31)<<6)+u16::from(self.byte()?&63)};units.push(unit);}
        Ok(String::from_utf16_lossy(&units))
    }
    fn object(&mut self,string_type:bool)->Result<Object>{
        if string_type {let len=self.len()?;return Ok(Object::String(self.string(len)?));}
        let tag=self.byte()?;
        Ok(match tag {
            0=>Object::Null,2=>Object::Number(i64::from(self.byte()? as i8)),
            3=>Object::Number(i64::from(i16::from_be_bytes(self.take(2)?.try_into().expect("short")))),
            4=>Object::Number(i64::from(self.var()? as u32 as i32)),5=>Object::Number(self.var()? as i64),
            33=>Object::Number(i64::from((self.var()? as u32 as i32).wrapping_neg())),
            34=>Object::Number(i64::from(self.u32()? as i32)),35=>Object::Number((self.var()? as i64).wrapping_neg()),
            36=>Object::Number(self.u64()? as i64),64..=79=>Object::Number(i64::from(tag-64)),80..=87=>Object::Number(i64::from(tag-80)),
            11=>{let len=self.len()?;Object::String(self.string(len)?)},88..=103=>Object::String(self.string(usize::from(tag-88))?),
            104..=119=>Object::Bytes(self.take(usize::from(tag-104))?.to_vec()),
            14=>{if self.byte()?!=1 {return Err(Error::ClassCast("OverflowDB requires byte[] H2 node values".into()));}
                let len=self.len()?;Object::Bytes(self.take(len)?.to_vec())},
            _=>return Err(Error::ClassCast(format!("unexpected H2 ObjectDataType tag {tag} in OverflowDB maps"))),
        })
    }
}
fn fletcher(bytes:&[u8])->u32 {
    let mut a=0xffffu32;let mut b=0xffffu32;
    for block in bytes.chunks(720) {
        for pair in block.chunks(2) {let value=(u32::from(pair[0])<<8)|u32::from(pair.get(1).copied().unwrap_or(0));
            a=a.wrapping_add(value);b=b.wrapping_add(a);}
        a=(a&0xffff)+(a>>16);b=(b&0xffff)+(b>>16);
    }
    a=(a&0xffff)+(a>>16);b=(b&0xffff)+(b>>16);(b<<16)|a
}
fn parse_map(bytes:&[u8])->Result<BTreeMap<String,String>> {
    let text=std::str::from_utf8(bytes).map_err(|_|invalid("invalid H2 map header"))?.trim();
    let mut map=BTreeMap::new();let mut chars=text.chars().peekable();
    while chars.peek().is_some() {
        let mut key=String::new();let mut colon=false;
        for c in chars.by_ref() {if c==':' {colon=true;break;}key.push(c);}
        if !colon {return Err(invalid("missing H2 map separator"));}
        let mut value=String::new();let mut quoted=false;
        while let Some(c)=chars.next() {match c {
            '"'=>quoted=!quoted,'\\' if quoted=>value.push(chars.next().ok_or_else(||invalid("truncated H2 map escape"))?),
            ',' if !quoted=>break,_=>value.push(c),
        }}
        if quoted {return Err(invalid("unterminated H2 map string"));}map.insert(key,value);
    }
    Ok(map)
}
fn checked_map(bytes:&[u8])->Result<BTreeMap<String,String>> {
    let end=bytes.iter().position(|b|*b==b'\n'||*b==0).unwrap_or(bytes.len());let line=&bytes[..end];
    let marker=b",fletcher:";let start=line.windows(marker.len()).rposition(|w|w==marker).ok_or_else(||invalid("missing H2 checksum"))?;
    let map=parse_map(line)?;let expected=hex(&map,"fletcher",0)? as u32;
    if fletcher(&line[..start])!=expected {return Err(invalid("H2 header/footer checksum mismatch"));}Ok(map)
}
fn hex(map:&BTreeMap<String,String>,name:&str,default:u64)->Result<u64> {
    match map.get(name) {Some(v)=>u64::from_str_radix(v.trim(),16).map_err(|_|invalid(format!("invalid H2 {name}"))),None=>Ok(default)}
}
#[derive(Clone)]
struct Chunk {id:u32,block:usize,len:usize,root:u64,version:u64}
fn lzf(bytes:&[u8],length:usize)->Result<Vec<u8>> {
    let mut input=Cursor::new(bytes);let mut output=Vec::with_capacity(length);
    while input.offset<bytes.len() {let control=usize::from(input.byte()?);
        if control<32 {output.extend_from_slice(input.take(control+1)?);}else{
            let mut len=control>>5;let mut offset=(control&31)<<8;
            if len==7 {len+=usize::from(input.byte()?);}offset+=usize::from(input.byte()?)+1;len+=2;
            let source=output.len().checked_sub(offset).ok_or_else(||invalid("invalid LZF back-reference"))?;
            for i in 0..len {let value=*output.get(source+i).ok_or_else(||invalid("invalid LZF copy"))?;output.push(value);}
        }
        if output.len()>length {return Err(invalid("LZF page exceeds declared length"));}
    }
    if output.len()!=length {return Err(invalid("LZF page length differs from declaration"));}Ok(output)
}
pub(super) struct Store {bytes:Vec<u8>,chunks:BTreeMap<u32,Chunk>,meta:BTreeMap<String,String>}
impl Store {
    pub fn open(path:&Path)->Result<Self> {
        let bytes=std::fs::read(path)?;
        if bytes.len()<2*BLOCK {return Err(invalid("H2 store is shorter than its two headers"));}
        let headers:Vec<_>=bytes[..2*BLOCK].chunks_exact(BLOCK).filter_map(|b|checked_map(b).ok()).collect();
        if headers.is_empty() {return Err(invalid("both H2 headers are corrupt"));}
        for header in &headers {if hex(header,"blockSize",BLOCK as u64)?!=BLOCK as u64 || hex(header,"formatRead",hex(header,"format",1)?)?>1 {
            return Err(invalid("unsupported H2 MVStore file format"));}}
        let mut chunks=BTreeMap::new();
        for offset in (2*BLOCK..bytes.len()).step_by(BLOCK) {
            if !bytes[offset..].starts_with(b"chunk:") {continue;}
            let available=&bytes[offset..bytes.len().min(offset+1024)];
            let Some(end)=available.iter().position(|b|*b==b'\n') else {continue;};
            let Ok(map)=parse_map(&available[..end]) else {continue;};
            let chunk=Chunk {id:hex(&map,"chunk",0)? as u32,block:offset/BLOCK,len:hex(&map,"len",0)? as usize,
                root:hex(&map,"root",0)?,version:hex(&map,"version",0)?};
            let Some(end)=chunk.len.checked_mul(BLOCK).and_then(|n|offset.checked_add(n)).filter(|n|*n<=bytes.len()&&*n>=offset+128) else {continue;};
            let Ok(footer)=checked_map(&bytes[end-128..end]) else {continue;};
            if hex(&footer,"chunk",0)?!=u64::from(chunk.id)||hex(&footer,"block",0)?!=chunk.block as u64||hex(&footer,"version",0)?!=chunk.version {continue;}
            chunks.insert(chunk.id,chunk);
        }
        let mut candidates:Vec<_>=chunks.values().cloned().collect();candidates.sort_by_key(|c|std::cmp::Reverse(c.version));
        if candidates.is_empty() {
            if headers.iter().all(|h|hex(h,"chunk",0).ok()==Some(0)) {return Ok(Self {bytes,chunks,meta:BTreeMap::new()});}
            return Err(invalid("H2 store has no intact committed chunk"));
        }
        let mut store=Self {bytes,chunks,meta:BTreeMap::new()};let mut last_error=None;
        for candidate in candidates {
            match store.read_tree(candidate.root,0,true) {
                Ok(entries)=>{let mut meta=BTreeMap::new();for (key,value) in entries {meta.insert(key.string()?,value.string()?);}store.meta=meta;return Ok(store);},
                Err(error)=>last_error=Some(error),
            }
        }
        Err(last_error.unwrap_or_else(||invalid("H2 metadata root is missing")))
    }
    fn read_tree(&self,root:u64,map_id:u32,string_type:bool)->Result<Vec<(Object,Object)>> {
        fn walk(store:&Store,pos:u64,map_id:u32,string_type:bool,seen:&mut BTreeSet<u64>,depth:usize,out:&mut Vec<(Object,Object)>)->Result<()> {
            if pos==0 {return Ok(());}if depth>128 || !seen.insert(pos) {return Err(invalid("cyclic or excessive-depth H2 page tree"));}
            let chunk_id=(pos>>38) as u32;let chunk=store.chunks.get(&chunk_id).ok_or_else(||invalid("H2 page references an absent chunk"))?;
            let offset=(pos>>6) as u32 as usize;let start=chunk.block.checked_mul(BLOCK).and_then(|n|n.checked_add(offset)).ok_or_else(||invalid("H2 page position overflow"))?;
            let mut header=Cursor::new(store.bytes.get(start..).ok_or_else(||invalid("H2 page position exceeds file"))?);
            let len=header.u32()? as usize;
            let end=start.checked_add(len).filter(|end|*end<=store.bytes.len()&&*end<=(chunk.block+chunk.len)*BLOCK).ok_or_else(||invalid("H2 page extends beyond chunk"))?;
            if len<8 {return Err(invalid("H2 page too short"));}
            let mut page=Cursor::new(&store.bytes[start+4..end]);let check=u16::from_be_bytes(page.take(2)?.try_into().expect("check word"));
            let reduce=|v:u32|((v>>16)^v) as u16;
            if check!=(reduce(chunk_id)^reduce(offset as u32)^reduce(len as u32)) {return Err(invalid("H2 page checksum mismatch"));}
            if page.var()?!=u64::from(map_id) {return Err(invalid("H2 page belongs to a different map"));}
            let count=page.len()?;
            let typ=page.byte()?;if (typ&1) as u64!=(pos&1) {return Err(invalid("H2 page kind disagrees with position"));}
            let mut children=Vec::new();
            if typ&1!=0 {for _ in 0..=count {children.push(page.u64()?);}for _ in 0..=count {page.var()?;}}
            let decompressed;
            let mut values=if typ&2!=0 {
                let extra=page.len()?;let compressed=page.take(page.bytes.len()-page.offset)?;
                let size=compressed.len().checked_add(extra).filter(|v|*v<=i32::MAX as usize).ok_or_else(||invalid("H2 page expansion overflow"))?;
                if typ&6==6 {let mut decoded=Vec::new();flate2::read::ZlibDecoder::new(compressed).take(size as u64+1).read_to_end(&mut decoded)
                    .map_err(|error|invalid(format!("invalid deflated H2 page: {error}")))?;
                    if decoded.len()!=size {return Err(invalid("deflated H2 page length differs"));}decompressed=decoded;
                }else{decompressed=lzf(compressed,size)?;}Cursor::new(&decompressed)
            }else{page};
            if count>values.bytes.len()-values.offset {return Err(invalid("H2 page key count exceeds decoded payload"));}
            let keys=(0..count).map(|_|values.object(string_type)).collect::<Result<Vec<_>>>()?;
            if typ&1==0 {for key in keys {out.push((key,values.object(string_type)?));}}
            else {for child in children {walk(store,child,map_id,string_type,seen,depth+1,out)?;}}
            Ok(())
        }
        let mut output=Vec::new();walk(self,root,map_id,string_type,&mut BTreeSet::new(),0,&mut output)?;Ok(output)
    }
    pub fn map(&self,name:&str)->Result<Vec<(Object,Object)>> {
        let Some(id)=self.meta.get(&format!("name.{name}")) else {return Ok(Vec::new());};
        let id=u32::from_str_radix(id,16).map_err(|_|invalid("invalid H2 map identity"))?;
        let root=self.meta.get(&format!("root.{id:x}")).map(|v|u64::from_str_radix(v,16)).transpose().map_err(|_|invalid("invalid H2 map root"))?.unwrap_or(0);
        self.read_tree(root,id,false)
    }
}
