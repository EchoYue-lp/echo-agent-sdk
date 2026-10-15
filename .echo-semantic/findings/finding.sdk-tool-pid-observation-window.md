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
