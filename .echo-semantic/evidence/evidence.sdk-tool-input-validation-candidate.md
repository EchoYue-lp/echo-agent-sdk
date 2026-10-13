---
{
  "schema_version": 1,
  "id": "evidence.sdk-tool-input-validation-candidate",
  "kind": "evidence",
  "observed_at": "source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404",
  "source_refs": [
    "echo-sdk-host/src/core_profile/facade/tools.rs#validate_tool_input",
    "docs/sdk/governance/evidence/sdk-tool-validator-20261013/sdk-tool-validation-host-proof.json",
    "docs/sdk/governance/evidence/sdk-tool-validator-20261013/sdk-chart-input-probe.log",
    "docs/sdk/governance/evidence/sdk-tool-validator-20261013/sdk-chart-input-probe-green.log",
    "docs/sdk/governance/evidence/sdk-tool-validator-20261013/archive-index.json"
  ],
  "supports": [
    "behavior.sdk-family-chart",
    "behavior.sdk-integrations"
  ],
  "limitations": [
    "Actual prototype invokes existing normal API, not yet formal malformed-input regression suite.",
    "Current fullRust418 proves newcanonical source only at recorded command; latestclippy/full3language/publishedframeworkhelpergate pending.",
    "Framework helper remains Rust/Host-only; no crosslanguagepublichelperacceptance implied."
  ]
}
---

# Direct Tool 输入校验候选反例

## 支持的结论

旧 SDK direct Tool adapter 绑定参数后直接execute，绕过ToolManager使用的framework schema/custom validator。真实chart普通API传入not_a_chart_type得到success=true及line图，掩盖非法enum。修复后同输入返回framework_error及canonical Tool InvalidParameter；合法bar仍精确生成bar。这个前后反例为真实小prototype调用，不伪称已加入正式回归套件。

## 来源与范围

新Host SHA c52da2d346d8848074b646e5442ecc83890cd68991f08287780280f871b59f89。SDK tools.rs SHA e3224c4ed670a639585a99af09c1f1902c61431a832507a4b835d4dac0b2f70a；复用framework既有validate_tool_input，仅提升Rust可调用visibility及doc，SDK不复制validator。参数绑定拒额外arity，包括额外null。新相同workspacealltargets418测试零failure/ignored；第一次错误re-exportmanager path编译失败和正确tools::execution结果均原字节归档。

## 已知缺口

非法参数须补正式三语言测试及其错误/副作用断言；最新完整Clippy、全语言和framework新publichelper完整门禁仍待自己的来源。旧6711完整结果不能覆盖新validator；不接受该helper成为三语言远程API，不关闭Finding或能力状态。
