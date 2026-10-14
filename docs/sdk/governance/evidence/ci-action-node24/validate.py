import copy,json,hashlib,shutil,subprocess
from pathlib import Path
import yaml
P=Path(__file__).resolve().parent
a=yaml.load((P/"workflow.before.yml").read_text(),Loader=yaml.BaseLoader)
b=yaml.load((P/"workflow.candidate.yml").read_text(),Loader=yaml.BaseLoader)
changes=[]
for i,(old,new) in enumerate(zip(a["jobs"]["language-sdk"]["steps"],b["jobs"]["language-sdk"]["steps"])):
 if old!=new:
  assert old.keys()==new.keys() and "uses" in old
  remainder=copy.deepcopy(new);remainder["uses"]=old["uses"];assert remainder==old
  changes.append({"index":i,"before":old["uses"],"after":new["uses"]});new["uses"]=old["uses"]
assert len(changes)==3 and a==b
provenance=json.loads((P/"official-provenance.json").read_text())
for item in provenance["actions"]:
 action=yaml.load((P/item["repository"].replace("/","-")/"action.yml").read_text(),Loader=yaml.BaseLoader)
 assert action["runs"]["using"]=="node24"
 with_inputs=a["jobs"]["language-sdk"]["steps"][changes[[c["after"].startswith(item["repository"]+"@") for c in changes].index(True)]["index"]].get("with",{})
 assert set(with_inputs)<=set(action.get("inputs",{}))
result={"exit_code":0,"yaml_parser":"PyYAML BaseLoader (preserves on key)","exact_ast_delta":changes,"all_other_job_steps_commands_env_inputs_permissions_concurrency_timeout_unchanged":True,"test_node":"20","java_distribution":"temurin","test_java":"17","python_input":"absent, existing environment retained","action_runtimes":"all three pinned action.yml use node24","actionlint":"not installed; no execution claimed", "warning_status":"unresolved until actual remote CI"}
(P/"ast-validation.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps(result,indent=2))
