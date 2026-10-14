# Workflow候选执行证据

同一共享fixture在TypeScript、Python、Java原生SDK实际执行 **48条记录：47次Workflow调用+1次Session close**，覆盖24个accepted操作中的17项。三语言均exit0，零skip；每项结果和输入保存在green-evidence，不能把1个语言test数当作单一行为覆盖。

| 故障假设 | old6711真实反例 | newcb9b真实结果 |
|---|---|---|
| resume_exact接受Checkpoint数组 | 17字段sequence执行继续至end，父CPload=null | InvalidValue，父CP全字段仍等于输入前canonical对象 |
| resume_with_state接受Checkpoint数组 | 原CP消费、approved=false与unapproved_mutation写入并走no路径 | InvalidValue，原state/context/generation/identity全部不变 |

合法object输入使用既有公开WireValue.Map，明确保留domain continuation.kind=node为普通JSON属性；generation/step_count用u64tag，不修改UUID、time、revision、state。SDK允许unknown wire kind透传，因此普通native对象的kind碰撞不宣称自动roundtrip成功；没有修改codec或扩API。

## 已证明局部行为

- pureRouter直接yes/no routing；gate和end前interrupt，exact path/steps/state。
- load/restore完整17字段CP；tag Unicode label/tags+generation单调增加。
- defer不继续/不consume；canonical exact resume，重复exact与按id resume明确失败。
- branch保父CP、child workflow_run_id独立/graph_revision相同、消费fork后的lineage关系；updates与reject理由/state可见。
- SharedState get/set/keys/snapshot保Unicode/optional/context，跨Session graph/state隔离。
- Sessionclose后closed graph/state错误；其它Session继续，resource quota两个slot实际refill、第三次明确PayloadTooLarge。
- 11条具名RPC error均按预期code核对；三语言48row归一化全等。

## 比较规则

仅UUID/RFC3339值按首次出现稳定标签保同一性，SharedState.keys的HashMap无序集合排序，以及error details absent/null统一。path、tags、context lists和其它ordered值不排序、不丢数字或字段。原始48row全部保留，可以回溯任意归一化值。

## 来源

oldHost SHA `6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5`；greenHost SHA `cb9b7fcad3946d9428cdef5a7b7258a334ee387ed34ca0612b4f40b262ef20a2`。producer root局部shape guard改动，framework保有claim/CAS权威；新Host出处为sdk-workflow-shape-host-proof.json。本lane没有SDK业务源码写入，也没有Cargo执行；Java只在/tmp编译runner并读取SDK公共classes。

red→green三个runner变化仅保存成功观察目录的环境开关；corefixture与probe class源码byteexact unchanged，原frozenhash不被当前manifest覆盖。red源码为匹配既有frozenhash的inverse重建，明确不是原backup。失效数组预期一直是InvalidValue，不为绿色更改成允许数组。

## 保留项

MemoryStore不证明跨restart，未覆盖run_stream/stream next/cancel/close、graph.cancel与extension run/run_stream（7 accepted operations）。本组不证明所有checkpoint历史过滤、并发claim竞态或graphrev错误组合，也不证明正式publishedframeworkpin/独立build/最新wholegates/main。
