---
schema_version: 1
id: evidence.sdk-mcp-channel-owned-candidate
kind: evidence
observed_at: source:pending
source_refs:
- docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/manifest.json
- docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/successor-native/typescript/run.log
- docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/successor-native/python/run.log
- docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/successor-native/java/run.log
- docs/sdk/governance/evidence/sdk-mcp-channel-20261013/owned-channel/successor-native/TEST-com.echoagent.sdk.McpChannelHostTest.xml
- sdks/typescript/test/mcp-channel-host.test.js
- sdks/python/tests/test_mcp_channel_host.py
- sdks/java/src/test/java/com/echoagent/sdk/McpChannelHostTest.java
- echo-sdk-host/src/core_profile/facade/integrations.rs
- echo-sdk-host/src/core_profile/facade/mod.rs
supports:
- behavior.sdk-family-mcp
- behavior.sdk-family-channels
- behavior.sdk-integrations
limitations:
- Exact five actual native scenarios each on immutable7bbd1569 Host, not all accepted
  operations or final publishedpin.
- Original cb9 four-scenario Java main receipt is historical and distinct from current
  five JUnit tests.
- Accepted MessageHandler inbound consumer and close reverse wire are not implemented;
  no inbound success is inferred.
- Independent SDK current-source scope rereview/full final gates are separate from
  focused runtime pass.
---

# MCP与owned Channel后继行为证据

## 支持的结论

原MCP真stdio exact metadata/resources/templates/prompts/Unicode与binary bytes、同名跨Manager/Session隔离和PID退出；注册transport首次close失败保同owner及quota、重试成功才失效；Channel四类attachment/send原字节、exact stop sibling隔离与失败重试。第五场景通过start/send/health/stop回调重入同manager metadata、A停驻时B与另一Session可操作，resource close首次失败保持同owner，成功B不重复stop、三个slot复用；Session close回调重入得到closing admission明确closed_handle，停止次数a4/b3/other1。

## 来源与范围

不可变Host7bbd1569367b0b1004fbe1cbc537b6869a2bc685bb1ca14e63228b1746aee3b2。正式Node、Pytest、JUnit各五项、零失败/错误/skip；共同ownedreentry投影匹配。归档保全部随机PID/handle原始值，仅跨进程不要求这些标识相等，callback全局顺序不属于合同。建本对象前已逐项核manifest六个source SHA与全部raw_files SHA，均匹配实际当前文件/归档。原两个真实callback红反例与首次后继fixture签名错误亦保留，不删除或伪改原日志。

## 已知缺口

独立后继源码复审、完整门禁和正式pin/main另验。MessageHandler已接受handle/handle_stream/reply尚无forward consumer；没有fabricated inbound callback，也没有新的close reverse wire。MCP所有请求/transport组合不从这五场景推断。family仍needs_review，模型mapped与focused runtime pass分开。
