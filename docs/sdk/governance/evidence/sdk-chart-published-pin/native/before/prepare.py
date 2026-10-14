from pathlib import Path
import importlib.util,json,sys
S=Path('/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/echo-agent-sdk-worktrees/sdk-semantic-governance')
spec=importlib.util.spec_from_file_location('computation_fixture',S/'sdks/shared/testing/computation_fixture.py'); f=importlib.util.module_from_spec(spec);spec.loader.exec_module(f)
work=Path(sys.argv[1]);f.prepare(work)
base=['bar','中😀边界','label','value',None,[{'label':'中文😀','value':1},{'label':'尾','value':4}]]
rows=[]
for ident,w,h in [('width-overflow',4294967296,123),('height-overflow',321,4294967297),('width-negative',-1,123),('height-negative',321,-1),('width-u32-max',4294967295,123),('height-u32-max',321,4294967295),('zero',0,0),('null',None,None),('wrong-width-type','wide',123),('wrong-height-type',321,'tall')]:rows.append({'id':ident,'operation':'generate_chart','arguments':base+[w,h,'json']})
rows.append({'id':'omitted-dimensions','operation':'generate_chart','arguments':base})
(work/'bounds.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2)+'\n')
