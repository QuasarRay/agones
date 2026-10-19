import sys,json,pathlib,re,hashlib,subprocess
R=pathlib.Path(__file__).resolve().parents[2];sys.path.insert(0,str(R/'build/scripts/agones-rs'));from kani_results import read_results,require_claims_and_covers
load=lambda p:json.loads((R/p).read_text());sha=lambda p:hashlib.sha256((R/p).read_bytes()).hexdigest()
reuse=load('verification/agones/gameserver-case-014-proof-reuse.json');run=load(reuse['proof_run']);gate=load(reuse['production_gate_unchanged']);current=load('verification/agones/gameserver-defaulting-codegen.json');frozen=load(reuse['frozen_metadata']);selected=current['harnesses'][14]
assert selected==frozen['harnesses'][14]
for p,h in gate['production_files'].items():
 assert sha(p)==h,p
 if p in run['source_files']:assert run['source_files'][p]==h,p
 else:
  assert p.startswith(('pkg/','verification/agones/')),p
  commit=gate['baseline'] if p.startswith('pkg/') else run['head']
  assert hashlib.sha256(subprocess.check_output(['git','show',commit+':'+p],cwd=R)).hexdigest()==h,p
for p in ['Cargo.lock','Cargo.toml','crates/agones-core/Cargo.toml','rust-toolchain.toml']:assert sha(p)==run['source_files'][p],p
assert current['ir_sha256']==run['source_files'][current['ir']]
assert sha(current['output'])==reuse['current_output_sha256']
assert sha(reuse['frozen_output'])==run['source_files'][current['output']]
assert sha(reuse['frozen_generator'])==run['source_files'][current['generator_path']]
old=(R/reuse['frozen_output']).read_text();new=(R/current['output']).read_text();bodies={}
for name in reuse['unchanged_selected_functions_sha256']:
 pat=r'(?ms)^fn '+re.escape(name)+r'\b.*?^\}'
 a=re.search(pat,old);b=re.search(pat,new);assert a and b and a.group()==b.group(),name;bodies[name]=hashlib.sha256(b.group().encode()).hexdigest()
selected_fns=set(selected['assertion_functions']);byfn={a['function']:a for a in current['assertions'] if a['function'] in selected_fns};oldbyfn={a['function']:a for a in frozen['assertions'] if a['function'] in selected_fns};assert byfn==oldbyfn and len(byfn)==18
rows=read_results(pathlib.Path(reuse['proof_run']).stem);fn=lambda r:r['symbol'].split('.assertion.')[0].split('::')[-1];claims=[r for r in rows if fn(r) in byfn];assert len(claims)==18;covers=require_claims_and_covers(rows,claims,selected['required_covers']);assert len(covers)==2 and len(rows)==6436
report={'schema_version':1,'classification':'REPO_SAFE','status':'VALIDATED_CURRENT_PRODUCTION_AND_SELECTED_PROOF_REUSE','feature_id':'GS-DEFAULTS-001','production_gate':reuse['production_gate_unchanged'],'proof_run':reuse['proof_run'],'summary':reuse['summary'],'current_ir_sha256':current['ir_sha256'],'current_output_sha256':sha(current['output']),'original_complete_case':selected['case_id'],'checks':{'all_executable_production_gate_paths_match_proof_source_receipt':True,'other_gate_paths_match_original_baseline_or_proof_commit':[p for p in gate['production_files'] if p not in run['source_files']],'lock_toolchain_and_core_resolution_match':True,'complete_source_ir_matches_proof_receipt':True,'selected_fixture_harness_18_assertion_bodies_byte_identical':bodies,'selected_harness_domain_bound_and_covers_metadata_identical':True,'selected_assertion_mapping_identical':True,'strict_complete_results_reparsed':len(rows),'source_claims_success':len(claims),'required_covers_satisfied':len(covers)},'candidate_decision':'Optional static-name and read-only pair iteration experiment reverted byte-exactly to established production after 6/8 GiB solver failures. Candidate/patch/diagnostics retained; no proof of the abandoned candidate is claimed.','candidate':'verification/agones/gameserver-borrowed-feature-candidate.rs','candidate_sha256':sha('verification/agones/gameserver-borrowed-feature-candidate.rs'),'ordinary_validation':'verification/agones/runs/gs-restored-workspace-1.json','scope':'Only complete original case 014. All 18 state observations occur before ownership retention; no formal post-observation cleanup/lifecycle or other case/domain credit. All eight required _2 cells remain open. CI will re-execute current complete harness at the published head with wall-clock limit 900s.'}
p=R/'verification/agones/gameserver-restored-production-review.json';p.write_text(json.dumps(report,indent=2,sort_keys=True)+'\n');print(json.dumps({'results':len(rows),'source_claims':len(claims),'covers':len(covers),'status':report['status']}))
