---
schema_version: 1
id: finding.sdk-media-accepted-tool-name-routing
kind: finding
type: implementation_bug
status: resolved
severity: high
primary_focus: contract_evidence
focus:
- trigger_input
- result_side_effect
- state_authority
boundary_ref: boundary.sdk-integrations
behavior_refs:
- behavior.sdk-family-media
rule_refs:
- rule.framework-runtime-authority
evidence_refs:
- evidence.sdk-tools-bridge-finite-observation
audit_refs: []
decision_refs: []
repair_evidence_refs:
- evidence.sdk-media-accepted-route-repair-verification
verification_evidence_refs:
- evidence.sdk-media-accepted-route-repair-verification
rereview_audit_refs:
- audit.sdk-tools-bridge-finite-contract-evidence
discovered_at: 208a027f587446badaaa2058e6af3240457431d2
---

# 两个accepted媒体操作未映射到原framework Tool名称

## 问题

accepted media的16个操作包含pdf_extract与excel_write，原Host却只按Tool::name索引extract_pdf与write_excel，合法accepted调用被当unknown拒绝。SDK拥有名字适配，framework parser/writer本身不是此缺陷修复owner。两处是同一根因，不为每个媒体操作建立重复Finding，也不扩大accepted合同。

## 触发条件与影响

旧不可变Host0bbc72ba实际构建源为公开208a（工具源码77c1d305a55e8320692929efca4f48a6d6dc7a4370cd33c9b403da996dcc15c3）；发现时driver工作树为公开17be，相关工具和语言生产源码与208a逐字相同，17be仅其已审材料后继。discovered_at保存真实旧Host代码身份，不把发现时间倒写为208a提交时间。

真实TypeScript public SDK调用中，同596B有效PDF的pdf_info返回Page count1，pdf_extract却返回invalid_value/Never/operation=pdf_extract、unknown media tool。独立有效1447B OOXML先由read_excel读出中文🙂/beta/3/7，再调用合法excel_write同样unknown、未到writer。两个命令分别自然退出1；不是编译、握手或decoder失败，也不冒称三语言before。原probe01相对state_root启动错误与02本地SDK操作名写法错误均未媒体RPC，另保原件，不当产品red。

## 证据

PDF原raw872B/SHA25601dc8f7a4b3532f01e0d596025a96848ad96658ff33e6d5ea3d1005bf49f382f，Excel875B/6d6baa8820c81d721c8a37f4d11aa36a96a10b5651b470b3413eb3bf8148ce88。源码链为accepted-facade-operation-catalog.json的media16ops→原_echo_agent/media/op→tools.rs::dispatch_tool按Tool::name查找；上游PdfExtractTool::name/ExcelWriteTool::name分别为extract_pdf/write_excel，其余14相同。额外注册的ExcelLoad不在此16项合同中，不补造第13个non-text操作。

原件、有效输入、完整typed结果与source/provenance只保ignored sdk-parallel-delivery-01a14385/media-accepted12。cleanup支持awaited Session/Agent/SDK close和fixture SIGTERM/exit，未设置独立HostPID生死oracle，不扩为全部物理无泄漏。

## 处理记录

当前薄修复只在原media lookup前保留两accepted拼写并映射到同一原Tool，operation/参数schema/校验/typed错误/Session context/execute/生命周期均不改。新源码8b892c9a4739277a0818caf740955cf480d36a561927e934429cd1986c887abf独立source审C0/I0/M0，无F/catalog/lock/generator/API或第二parser变化。

新Hosta17db78e5d3e73a5edf9ef529d387a218aa248d7042b08e12b88b843cd53baaf实际新编译退出0，保opt1/assertions/overflow/unwind；同原两probe只改expectedHostSHA后各actual0，PDF真实text与info控制通过，Excel真实writer文件产生；真正写→读同文件由三语原fixture媒体短链另证。原六族默认case及新增media sibling在TS/Python/Java各2pass/0fail/0skip，每个media故事12个成功操作/5typed RPC error/2structured tool failure，Word/PDF真实parser、Excel ZIP cells/精CSV bytes、图像bytes/model_content及错误不覆sentinel均已强断言。不是新全语言整套或新源码14资格。

### 04eb首源码检查点（历史）

本项在此focused检查点仍open、三链空：先保存修复源码/测试与真实历史，后继以公开修复Git绑定最小实际repair/verification Evidence及原Audit精challenge，并在新冻结源适用完整门禁与独审后再具体关闭。不能把dirty源假绑为17be已修、以旧208a全套借资格，或用本项关闭managed历史Linux/main/平台/nativeHook义务。


Excel before/after seed由同脚本生成、语义输入相同，五个XML entry原字节逐一相同；ZIP时间戳13:21:52与13:28:50不同，因此容器整体SHA不同，不能称seed ZIP byte-identical。PDF before/after输入SHA相同。此精度修正来自实际输入独核，不修改成功断言或重跑结果。


## 公开修复源的后继绑定（9ef前材料阶段）

修复源码/原shared及三测试已公开于04eb6ad50982656b5a8b8ea649619b6a383065ff；repair_evidence_refs已绑定联合Evidence，原ToolsBridge contract-evidence追加精challenge。Finding仍open，verification_evidence_refs与rereview_audit_refs继续留空，待最终新冻结源完整Rust14、唯一whole语言/3quickstart和精确后验后再填，不以已有repair提前closed。


## 2026-10-20 最终匹配源码资格后具体闭合

04eb修复公开源与9ef最终工程源码的production/tests/dependencies/contracts/CI逐字无差；9ef原14Rust全部actual0/零warning/1222tracked无漂移、最终fullfacade Host实际a17不变、唯一原whole入口TS358/PY442加18子测试/Java379及76freshXML/三quickstart全部0，同一Host SHA/mtime与1222输入不变。原独立读者对源码/actualbefore-after/三语故事/newbuild/final14/whole及原件均C0/I0/M0，原ToolsBridge Audit只按本缺陷精challenge复审。故本项现在resolved，repair与verification分别引用联合Evidence的相应实际事实，rereview引用原Audit精challenge；没有新API/第二parser或改source合同来绕过失败。

本地具体闭合不替SDK精main及实际9check、Windows/CI17、本机nativeHook/Stop或其它composite；managed历史Linux根因未知仍open/无repair，不能用本次PASS继承为它的因果修复。旧04eb初检查点open/空链及9ef前partialrepair的文字保各自历史时点。
