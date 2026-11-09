# ADR 0045：SDK 当前值投影、命令资源与输入边界

## 状态

已批准

## 批准范围与资格

Root依据用户既有NoCompat目标和Framework ADR0105先批准I1三语输入同步，随后经e33158方案和7142设计独审，批准同一受控u03候选内的Sandbox六字段投影、原owner资源/取消接入及精确typed DELETE。P01/H01和typed DELETE均已取得真实有限Before并完成独立复核；当前允许最小实现、同源Rust/Host/三语言验证及原full/export后继。正式源码采用、pin/锁/风险、HTTPS来源、main/发布和完整原生验收仍独立，不从本候选继承。

当前副本继承u02真实1298文件（manifest `1a10b9787bc324258b59095d3113568589df825290d1732944524e56f114fe9c`），u02及其原full/export自然0保持不可变。SDK正式3f/f7/0dac未修改；本ADR只属于受控候选。

## 背景与权威

Framework `340993201cc937fc8e4783b66be399ef9f14e3d5` 的 `echo-core/src/tools/permission.rs` 和 `docs/adr/0105-current-config-and-plugin-inputs.md` 已规定精确八ID输入。SDK实际u02 retained四variant摘要变化删除旧Serde aliases，而三语本地parse仍做alias与trim/lower转换，可在向Host送canonical前恢复已取消的输入。

八个模式本体和权限决策继续归Framework；[ADR0034](./0034-sdk-governance-authority-and-lossless-adaptation.md) 的无损适配、[ADR0044](./0044-sdk-source-disposition-native-callback.md) 的source增退边界保持。不因四digest变化新增接受身份、schema或安全闸，不把所有enumparser一起重构。

## 候选方案与决策

保留alias/trim/lower会继续承担已取消的输入成功义务；只去10个variant别名但保ask/大小写/空白也不等于当前Framework输入。恢复Framework旧别名违背既有决定。因此三语PermissionMode.parse采用精确匹配八ID：

`default`、`plan`、`auto-edit`、`full-auto`、`auto`、`bubble`、`dont-ask`、`strict`。

旧 `ask`、`accept-edits`、`autoedit`、`acceptedits`、`fullauto`、`bypass`、`bypass-permissions`、`bypasspermissions`、`dontask`、`strict-confirm`、`strict-confirmation` 均拒绝。大小写变体、首尾空白、未知与非文本输入拒绝，不默认成某个模式。语种既有异常类型/文案保持各自习惯。

TS从既有八值常量精确找值，Python使用既有枚举canonical值，Java在现switch精确匹配；不新增另一份权限策略。write/interaction/classifier、Default枚举和值、序列化canonical输出均保持原样。其它RuleSource别名等独立合同不变。

## 测试责任与实际证据

原TS/Python alias成功断言属于已退出义务，在新候选替换为明确拒绝；原件保在u02和I1 original-before。新测试先冻结后在原生产parser取得RED，再只改三语解析入口：Node原61项为17通过/44失败，Python原61项为18通过/43失败；两者失败是非canonical输入被接纳，没有环境/构建前提错误。相同测试SHA在修改后各61通过、0失败、0跳过。

测试使用原已安装Node22.19.0的focused Node harness直接载入生产TS模块，以及原现有Python3.12.4/pytest9.1.1虚拟环境的原pytest入口；无install、无手写catalog/schema、无Host/Cargo/Maven。直接源模块加载的有限资格不代替完整TypeScript SDK build。所有命令、自然退出、raw、Before/After parser与测试SHA保存于本切片独立证据目录。

八ID roundtrip、wire字符串输出与八模式helper真值表作为正控，11旧alias、每个ID的大小写/首尾空白、unknown/空串/prototype名称和非文本作为负控。原parity映射完整性仍检查，不改其accepted种子来使测试通过。

Java的独立轻量驱动只调用现PermissionMode public API；同一驱动分别编译原u02 enum与u03 enum，原版本编译0/行为14通过43失败，修改版编译0/行为57通过0失败。原JDK21.0.5的java/javac/release均绑定SHA，两个独立输出目录总量不足20KiB，未修改系统缓存或下载依赖。这证明该enum的有限当前行为；新增JUnit测试源尚未通过Maven编译/执行，不冒完整Java SDK/三语Host验收。

## I1阶段的影响与非目标

只改三个PermissionMode parser、直接focused测试、三语说明和本SDK说明/ADR。生成物、source接受集合、工具/parser/guard/deny、其它enumparser、权限默认/决策/模式行为与Framework均保持原件。现八ID例子无需改写；后续完整SDK examples/官网正式交付仍由原owner按实际范围核对。本候选不触发网站部署。

## 回退

必要时只在本新副本将这组parser/tests/docs整体回到同一u02端点，保真实RED/GREEN原件，不把旧alias成功改称当前匹配。不可覆盖原u02或把旧schema、权限策略、Framework别名补回以求兼容。正式采用、Git动作与后续运行窗口仍须Root明确安排。

## 后继受控批准：Sandbox 值合同、原owner与精确拒绝

Root在设计独审 `sdk-u03-sandbox-value-owner-proposal-independent-review-r01.md`（SHA256 `7142a119da2a9ae24e780663a239e8de6dec9569b6a639bd65d4c152927256cf`）PASS后明确批准本唯一u03副本的有限路线。真实Before已完成：P01 2通过/2失败，H01 0通过/12失败；typed DELETE经TS/PY两行既有关闭码期望纠正后，两语各1通过/4失败，Java 2通过/3失败/2错误，均零跳过。原编译与Maven前提失败单独保留，不冒行为RED。reader0与Perf独占窗口释放后，Root已允许直接实施I2/I3/DELETE并以同测试GREEN。此批准不覆盖正式source/pin/锁/风险、旧guard、原u01/u02、Framework或发布。

I1原批准文稿及11个after文件保于独立sealed-I1-after，parser三源与其同测试不再变化。原ADR0044精确11retire/3with_id accept/native0及14机器处置字段保持；本后继不重放首次transition flags、不接受额外Rust成员。

## 真实问题与候选方案

实际u02 full/export均自然0，但有三项消费缺口（实际独审a4392d0b…）：PermissionMode旧alias（本候选focused已闭合）；SandboxCommand raw handle类型自动广告新sourceinvoke而Host无authority；Sandbox reverse proxy按值serde提前丢弃command-only guard/cancel。生成成功不改称生成失败，也不替这些缺口签运行票。

SandboxCommand在f7的被接受SDK义务是六字段value，340新增serde(skip) resource_guards/cancel属于本机运行时持有；它的真实Rust类型可含进程内资源，而SDK仍可投影原可序列化值。删合理Framework字段/accepted、全局忽略serde(skip)、改分类器优先或新造opaque command owner都不是最小修复，不采用。

采用现SDK唯一facade route/manifest机制的显式投影扩展：只匹配 `echo_core::sandbox::SandboxCommand` / struct，在facade.rs一个descriptor列原serialized六字段kind、minimum_isolation、working_dir、env、timeout、stdin和两runtime-only字段resource_guards/cancel。inventory仅消费同descriptor的SDK route/mapping/language投影，不改rawclass=handle、contains-process-local-state、原字段/impl/derived遥测或normalized父digest；其它kind/source/子成员不继承。

route使用既有Value和原WireValue/serde字段转换，语言target表达真实六字段值，不广告不存在的opaque构造/返回/close API。后继正常生成后bare SandboxCommand不应在可执行catalog；精确source invoke沿现非canonical operation的InvalidValue/Never拒绝，原policy/extension值输入仍可用，不补Null或虚假成功handle。

## 单一命令wire边界

Protocol/Host共用同一descriptor/闭集字段定义；原policy value与SandboxExecutor command入口都消费它，不给三语言各造validator。仅原六字段允许；原Option缺省/Null、env/timeout/CommandKind serde表示保持。resource_guards/cancel或未知顶层字段输入明确拒绝，不靠serde(skip)吞掉后声称恢复本机资源；坏类型拒绝。这个约束防框架和合同失真，不新增本地权限门控。

现有generic component_value按值消费command是生命周期缺口；不得把“正常wire不会有guards”当作本机调用者非空guards已被保的证据。投影与下面真实owner修复必须一起验，不只改catalog。

## 命令资源与取消的唯一owner

复用原CallbackInvocationOwner/ExtensionInvocationLease，不新增公共opaquewire或第二执行器。借用序列化先避免消费command；同exact command-only guards在任何可见reverse提交前移交原CallbackInvocationOwner私有保留槽，不能伪造ToolContext或TaskRun。非流与stream（含不支持流的降级）都走原owner保留/释放；setup future返回、observerdrop、ACK、局部terminal、超时或unknown不能释放仍在运行/清理的producer资源。

真实physical closure在原state gate内一次take资源，gate外drop并沿原finished通知顺序结算；registration原unknown Arc链继续保原资源。实际producer已停后不要求下游额外poll(None)或新wire ACK才释放；必要真实response、EOF/cleanup的先因不能从字段名或假terminal推断。提交前未产生执行的失败按原规则释放。

取消按Framework原 `explicit.or(command.cancel)`，两者皆无才原run/connection fallback；已选中token不再被另一Session active-run覆盖。接原lease child及connection.cancel_all/close-admission链，关停取消不能丢；不另起OR-token task/owner，也不把取消信号当作物理停止证明。

## 旧DELETE的精确typed拒绝

旧source operation `echo_agent::state::delete_persisted_conversation`不广告为成功route，只保FeatureUnavailable/Never/原operation/retired文案和零mutation。TS/PY在原catalog集中入口精确拒绝；Java在EchoAgentClient.invoke/call及SessionHandle.invoke查找前用既有failedFuture交付。TS/PY原closed Session优先closed_handle，Java优先stale_handle；精确DELETE遇closed Java client保host_exited，普通未知操作仍原同步catalog错误。Java FacadeCatalog实现不改，不广catch其它异常。

Host在原结构/预算校验后、catalog/Session/Store前拒绝同一完整字符串；未知/相近/大小写与坏结构仍各自原错误，删除不可达重复delete handler分支保clear协调器。不得合成有效signature、managed请求identity或调用managed owner；raw CRUD、原clear/managed稳定请求职责保持。

## Before测试、验收与来源

新增Protocol测试只用现manifest_entries/现reverse-call.validate，来源为u02真实public-api快照，断言raw handle与SDK value并列、exact kind/source负控、三CommandKind/Option缺省/Null原值和runtime字段注入拒绝，不引用after-onlyhelper或手写catalog。新增Host测试只通过现SandboxExecutor proxy/trait和原ACP回调；command-only DropProof没有其它ToolContext克隆，覆盖execute/limits/cancel、非流降级、stream setup/早ACK/晚原response、observerdrop、预取消/中途取消/显式优先和真实cleanup前后drop。

原typedDELETE三语Before测试及managed-retirement durable/calls-after控制在同一后继Host批次执行；真实Before运行失败必须是行为反例，编译/环境失败不冒RED。完整命令/原环境/唯一target及实际raw/源SHA由具名运行票绑定，不以本ADR批准或设计PASS代运行。

实现GREEN后若投影影响生成政策，必须在新资源窗用原完整update/export（不带14flags）产生新actualdiff再独审，不手改seed/生成物或把原26profiles/136票改签新epoch。正式HTTPS pin、锁/风险、历史guard精artifact绑定仍另论。

SDK说明、examples与官网按实际涉及项核对：现sandbox示例仍构造原六字段值，不加guard/tokenwire；Permission例子用canonical8。没有产品/wire呈现变化的网页不为形式部署；文档同步与不适用原因随最终源码交付记录。整套回退保持u02与sealed-I1原件及真实RED/GREEN，不能在新Framework恢复旧别名/字段兼容或新造请求owner来求绿。

## 当前实现与待验证边界

原facade模块的单个descriptor只匹配精确source与struct kind，持有六字段闭集及两个runtime-only字段。manifest保raw分类和摘要，SDK route与语言target/contract suffix消费同一投影。Protocol reverse-call与Host原SandboxPolicy入口共用闭集/形状检查，不新造三语validator或命令handle。

Host借用序列化命令，将原guard Vec移交既有CallbackInvocationOwner；原物理闭合点在gate内take、gate外drop，unknown注册持有链继续保留。显式token优先于command.cancel，选中token不被active-run覆盖；两者皆无才原run/connection fallback，连接cancel_all仍取消原lease。预取消命令在提交前拒绝。stream复用原producer与ACK/真实terminal闭合，不依赖下游额外poll(None)，不增加wire ACK。

精确DELETE拒绝不提交RPC或生成managed身份。Host在结构与预算检查后、catalog/Session/Store前拒绝，原不可达删除分支移除；clear仍调用同一Framework协调器。Java只有指定异步入口交付failed stage，元数据查询和普通未知操作保持同步查找错误。

以上为实现说明，尚不宣称GREEN。After必须跑完原四Protocol/十二Host控制、三语全部入口、Host unknown/malformed及managed clear/replay/durable后段；Before首反例停止的限制继续保留。之后原完整producer/export不带旧14flags生成实际后继并独审delta，不手改catalog/seed或重签旧26profile票。

## R02：原入站值与本次提交事实

R01的P01真实四项通过，H01真实六项通过、六项失败；旧driver按阶段条数误写的汇总状态不是通过权威，以原native101为准。六项取消首错共同来自测试mock在ACP顺序请求回调内等待清理，阻塞后续通知。按官方ordering/既有工作夹具，将同一mock producer交给原connection.spawn持有；原末poll完成事实、response/cleanup、观察句柄和两端connect等待保持。修正后六项均通过，不以增加等待替代修复。

新增预取消控制首先观察到远端mock入口（包括cleanup）被进入，不冒称该计数就是Host发送或wire ID。另在已有framework-shell最小profile上，真实协商/Agent/Session/Extension/public facade合法六字段调用通过，三个注入调用被R01错误接纳并在真实producer/stream清理后报告反例。原Cargo行为因空间门槛未启动，该组行为由其实际已编译测试二进制的同filter/参数取得，不能重标Cargo命令已运行。

R02在原public shell入口的serde消费前复用唯一descriptor校验原JSON。清理条件复用ReverseInvocationScope的原观察器，在越过send_request同步发送尝试后置位；它不是成功入队、物理执行或停止receipt，关闭/发送失败仍可能没有入队。前置取消未越过尝试时不触碰共享executor；之后不能仅凭取消标志认定未尝试，不另造执行owner。外层cleanup与原lease使用同一实际选择token；普通execute/limits与带cancel入口保各自原错误映射。

预取消回归另记录原Host writer真正写出的JSON-RPC字节，并以既有SandboxIsAvailable调用作为发送顺序观察屏障，检查只有该显式屏障的实际请求ID、没有执行或cleanup请求。它不参与生产物理结算，不增加协议或ACK；远端入口/cleanup计数继续各自断言，不与Host提交事实混用。

8087冻结上原Protocol四项、Base十二项和正确Shell四项均已真实通过；Shell行为用实际已编译二进制运行，资源门槛下未启动原Cargo行为命令，原件分列。后继发现响应hook注册错误的强持链缺口，此前二十项不覆盖它，不能据此宣整体生命周期完成。

注册失败故障控制仅在测试配置下暂停同一独立调用者于真实发送尝试后/consume_with前，故障闸使用原runtime的block_in_place避免阻塞transport actors。待原peer任务真实进入后，通过既有connection.spawn的错误任务触发库自身task_actor失败，并以实际任务注册API拒绝接纳核实task_rx已关闭；join外层driver只用于测试收尾，不能冒该故障事实。原physical任务由独立原JoinHandle保留、统一release后真正join；测试只观测Weak原owner，不自己强持guard。若原响应hook已丢失，即使测试知道外部任务清理结束，也不得替Host补造原物理证明；缺少原proof时保持unknown，正常原回复的释放仍由既有正控验证。

该控制已取得真实NoRun0/零warning/list1及行为101：原peer进入、真实task故障已安排、任务注册API拒绝均成立，随后guard在原工作仍held时提前释放。原始失败与必要二进制已独立保全。最小修复只在on_receiving_result注册Err处，将同一个CallbackInvocationOwner接入原retain_unknown强持链，并返回原transport_error；不新增错误身份、owner、执行器或wire。修复后的新增控制及原正控尚待运行，不把源码修改称GREEN。
