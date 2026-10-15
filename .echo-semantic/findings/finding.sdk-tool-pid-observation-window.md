---
schema_version: 1
id: finding.sdk-tool-pid-observation-window
kind: finding
type: implementation_bug
status: open
severity: medium
primary_focus: contract_evidence
focus:
- result_side_effect
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-shell
rule_refs:
- rule.framework-runtime-authority
evidence_refs: []
audit_refs: []
decision_refs: []
repair_evidence_refs: []
verification_evidence_refs: []
rereview_audit_refs: []
discovered_at: 722d09b1a0837f170d4af19cbc3e417d2131b5f3
---

# Tool进程采样断言在Linux得到空列表，唯一原因尚未证实

## 问题

同一Linux原完整语言另一个失败为 `test_tool_value_effect_native_host` line113期待单个自有进程，实际pids=[]。源码在create_task(session.invoke(...))后每10ms采样至多50次、或RPC task.done时停；然后terminate_observed要求len(pids)==1并重新核身份。原错误发生在测试观察边界，不是被证实的生产shell/Host漏启动、PID泄漏或错误回收。

目前唯一原CI因果未确定。禁止从[]推Linux ps截断或生产shell缺陷。Tool已完成Darwin同不可变Host的受控合法请求准入排队反例：原50轮每次sleep10ms只是名义总500ms，同步ps成本使实际观察耗时2.592508709秒，结束时task仍pending且请求尚未dispatch，pids一直为空，原len1断言真实失败。释放请求准入后，约112ms观察到相同自有tail，再严格复核PID/PPID/命令/root→SIGTERM→原完整ToolResult失败cause/metadata返回、保护文件SHA完全相同、afterPIDs和finalPIDs均空。这证明有限采样窗口对合法请求排队脆弱，不证明原Linux[]的唯一原因。

## 触发条件与影响

发现基准722/PR实际merge `5d407e160bb5c7ade01be1980ed9a4e5a6888d17`，不冒裸head执行。原本地第四whole800/TS336/PY41518/J35576XML真实0保macOS来源；新Linux反例不倒改旧执行/不批量失效8Host风险审。

## 证据

原Linux语言log Admin `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-722-ci-evidence/116172597308.log`，198565B，SHA256 `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`，内容只读。当前新证据尚未正式Git，不伪source:pending Evidence或futureGit。

- `sdks/python/tests/test_tool_value_effect_host.py`，原722 Git源码SHA256 `b7b59e497d324a2b914e8ab641cfb0b70f9e16523c3d04c7acca7ae211e942a7`。

## 处理记录

后验需保自有PID/PPID/命令与root匹配、信号前同身份复核、真正原进程消失/错误字段与文件effect强断言；先实际定位采样结束原因、请求是否到达/启动/已完成，再修最小观察协议。不得只拉长常数或放宽len断言把[]当成功。当前只open未知原因/缺证，不要求新产品决策、不推资源泄漏；修后actualsource/rawreview/有限native证明齐才关闭。

此open候选仅本测试support或CIbootstrap边界，不新增family/API/权限/RiskAccepted。原四已解决Mac具名缺陷保持闭合，56/4是原切点而非含本新缺陷的当前最终总数。全goal30d UTF8工具修复第三repo仍待授权，不能改工具或scope来关闭这些记录。


## 受控观察的独立边界

原sourceSHA仍722 b7b59，Host SHA0f1未改，受控脚本只在Admin真实public invocation准入前排队，不修改SDK/Host实现。原default ps与wide ps在未dispatch阶段都[]，release后都同一个原tail；没有证据把原Linux失败归ps宽度截断。2.5925秒是已测wall elapsed，不能写成实耗0.5秒；从实际admission到观察为112080292ns。后继条件式observer的界限来源方案尚在审，当前不宣它已修/afterCIgreen。

受控results SHA256 `aa80ee24e0af0e3e3863ce9f00e9c7785443164bf7362e19dc3b7249bfd760e9`；原工具运行收据 `/Users/ls/MyWork/code/ylp_agent_learn/lp-agent/.worktrees/admin/sdk-linux-tool-pid-investigation/controlled-dispatch-gate-run.json` SHA256 `be74a72f8d572f2a13c7b6ff4b999ea33bc9e6f021f60bebbb19380a9ef44226`。这些Admin原件不伪新Git正式Evidence。


## 正式CI保存材料的当期观察（f68）

本段当期观察Git `f68cb49bf421ecdb71dfa4775495c363e31b3324`仅定位正式保存的722运行材料，不把运行时间或源码测试执行标成f68。原发现点`722d09b1a0837f170d4af19cbc3e417d2131b5f3`、PR实际merge5d407、候选所有原Map观察/scenes/status/锚点与对应测试源码SHA保持。正式来源逐路径与Hash如下，可从该Git读取：

- `docs/sdk/governance/evidence/sdk-722-ci-results/README.md`，Git f68 SHA256 `dcd6a9e4922a05527b189a2c03108a3ac73936d5107d022600dada1e570f0b27`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/manifest.json`，Git f68 SHA256 `4eb02f38b55191bfa761cd892b667733dcb26d4462eb655cfd2a22cbd379443e`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/SHA256SUMS.json`，Git f68 SHA256 `c22f92784ad1874ffcb0bb70ad312965e50bda78dcd69871230b2073006f1e44`。
- `docs/sdk/governance/evidence/sdk-722-ci-results/jobs/116172597308.log`，Git f68 SHA256 `70548f9331858291e374f6e751581cfcf854d8767900d03738d5609041880547`。

正式完整八job范围为5success/3failure；TS336及quickstart通过，Python413通过/2失败/18subtests，Java未达。两个合同job原13连续性阻断与旧证明分别保留，不从CI fail泛化未知生产缺陷。PID原Linux[]唯一cause仍unknown，Darwin2.5925s排队原反例不是该CI唯一cause。bootstrap单input实施只原配置/顺序依据，新remote error结算尚未证明，不提前resolved。原四已闭Mac缺陷不重开、B/12未决保持。

原上文“第三repo工具交付权限待批准”属于候选准备时历史；Root后续已获得第三repo工具修复交付授权，实际工具修补/源码SHA/发布main与新CIjob结果尚未回报，本候选不冒已交付或官方原30d门禁已0。


## 已提交修复与有限后验进展（仍open）

Python唯一watch分支修复已806真提交，原固定50轮改发现自有PID或原invocationtask settled条件、10秒复用既有fixture watchdog不是productdeadline。len1/同PID复核/SIGTERM/await同result/保护文件原oracle不放宽；源码8cbc只参数绑定/格式化收RuffB023、无noqa。实际Darwin3.12完整Tool49rows/48calls及三pure诊断0；控after5.342s旧采样仍pending而新observerrelease后samePID完成。其controlled调度事实并非原Linux唯一cause；即使Mac0当前Finding仍open等待新Linux原case与完整三语/适用CI，不能RiskAccepted。

当前材料观察Git `ff3de936d22c6d5f4e68ece646f0b4cfd77e7c56`仅定位已提交源码和正式包；原722/merge5d407失败运行/发现时间不改，新remote原case仍待。以下实际Git来源可读，不把本地focused当Linux完整验收：

- `sdks/python/tests/test_tool_value_effect_host.py`，SHA256 `8cbcdd12277393999231afafa48d91684232f0ef9408fa5c1b430a74e8b0df57`。
- `docs/sdk/governance/evidence/tool-pid-observation/README.md`，SHA256 `190c6496e7c1afef69e3c3f776cda18e1cd00a9a07b1af63fd37a41919933b63`。
- `docs/sdk/governance/evidence/tool-pid-observation/after-native-source-and-results.json`，SHA256 `0f27d379d3c5e95215254a4c6534d6a6152aee71f1d76936b4a89517178a00fa`。
- `docs/sdk/governance/evidence/tool-pid-observation/controlled-dispatch-gate-results.json`，SHA256 `aa80ee24e0af0e3e3863ce9f00e9c7785443164bf7362e19dc3b7249bfd760e9`。
- `docs/sdk/governance/evidence/tool-pid-observation/controlled-candidate-dispatch-results.json`，SHA256 `28f641a191443a5073eb5aaa40aedf339fb7183340f4cd9e11b4baa05c97faf1`。
- `docs/sdk/governance/evidence/tool-pid-observation/phase4-source-freeze.json`，SHA256 `8e52a6eff9362585f126bd3f2c0bee6337969a9164e94d60c9785e58d4ad2655`。
- `docs/sdk/governance/evidence/tool-pid-observation/SHA256SUMS`，SHA256 `4bf67fd29f509cd433805ebab60369ee9515769c8e4e20817cb713ca8f0363fd`。

源4tests已变化，先前第四whole800完整0继续保它自己的执行cut而非声称当前all800exact。生产61Rust/Cargo输入与锁未因此变化；本次不会重复原14Rust或本地大target完整门禁。新CI按focused开发门禁后实跑方案另执行，暂不关闭本Finding或提升Behavior/库存/全goal完成；B3/12、第三工具交付和官方原30d门禁各按真实当前进展处理。


## 0ebe原Linux直接因果与-ww有限后继（仍open）

此前unknown/排队反例段落保留为当时观察；新0ebe原CI完整JSON给同一Host18445、自有tail18521在0.03058024秒的两行：default `tail -f /home/runner/work/echo-agent-sdk/echo-agent-sdk/target/t`，wide保完整`.../tool-value-effects/python-piqww4ly/tail.txt`。严格路径matcher因此漏掉已真实spawn的tail；原tool随后5003ms正确返回timeout，不能说从未dispatch。原0ebe语言job116226422997日志200288B/SHA0fe6bf746d27ca6aaa4c7b741f1cbfdd22c056c60b66cdfdb7550fc59522b88f，公开运行38674982406，不是原722唯一cause的新推断。

Admin `sdk-linux-ps-width-fix/causal-facts.json`（2249B，SHA256 `fbee6c2216c49320595c94498209b8daf8d0726255f7121edc08c9b537504538`）；Admin `sdk-linux-ps-width-fix/after-native-summary.json`（3004B，SHA256 `e2a8f356aa103bca8f0ba676a0d5930dda5e440a6002f40bf83cdeaa961b2030`）。真实修复Git `c5c977ef862acbe99684294cca966c559859b6be`只原ps argv加`-ww`，不改matcher/超时/identity复核/任何生产源。修后SHA62bc5fb0a8f5512060ce031303a80cbe1d0b1d1a1fad79c317aa53ac866d06ee；Darwin3.12本地49rows/48calls与protectedrows、timeout/signal物理after[]均0，SDKcap有限独审PASS。原执行HEAD185559时测试候选尚未提交，c5只是保存同字节，不能把185559 Git源码当62bc。

原722 unknown和Darwin2.5925秒合法排队反例不删改；本段新增0ebe直接事实取代当前原因未知描述。Finding status仍open：新exact-source Linux原case与CI后验未得，不把Mac通过当Linux验证，不做RiskAccepted或泛资源cleanup结论。
