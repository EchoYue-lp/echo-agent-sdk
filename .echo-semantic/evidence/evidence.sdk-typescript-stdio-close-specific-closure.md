---
schema_version: 1
id: evidence.sdk-typescript-stdio-close-specific-closure
kind: evidence
observed_at: 76601b242d7c354352f3b5c7d4bf099873746b14
source_refs:
- sdks/typescript/src/client.ts
supports:
- behavior.sdk-language-clients
limitations:
- Specific native stdio ownership repair and exact same-source verification only; not all SDK families or SDK main delivery.
- Original failed official language command and intermediate client epochs are preserved, not relabelled as green.
- Actual child exit is not a claim that every grandchild or generic five-second Task producer settled.
asset_refs:
- asset.sdk-historical-proof-location
---

# typescript stdio close 局部闭合综合证据

## 支持的结论

具体支持 finding.sdk-typescript-stdio-close-ownership：singlecloseowner synchronousadmission + originalstdinEOF fact + actualsamechildexit + lateIOerror/failedretry; no wholemain。修复、旧真实反例、当前验证在同一对象中分明保留，不制造平行状态权威或近重复repair/verification报告。该证据可以同时用于原Finding的repair_evidence_refs及verification_evidence_refs，因为两项事实均有实际来源。

## 来源与范围

修复源 SHA `dd822f73e5cfeb84a5d082923b255d18357b3480a03602ec6a79fcb45e9aeb3f` 与完整官方a634运行790 inputs before/after/current实际相同。before、源码delta、focused成功/失败及原nativeHost事实见精确source_refs。后来官方同一语言入口100567bytes/exit0，TS308、PY375+18、Java332/71XML及3quickstarts；真实Host193e49b8对应已发布frameworka634。原whole错误98d714/TaskHTTP回调连接夹具反例仍保独立scope，不转成客户端源码缺陷。

原Telemetry XML真实2tests：publicCloseWaitsForRealSlowTelemetryFlush与acceptedTelemetryExportsRealBusinessRecordsAndFlushes，零失败/错误/跳过；后者内部两个console配置，不虚记3JUnit tests。其current全语言执行体确为此最终客户端源，不借旧b89/3aa/4967统计替代。

## 已知缺口

独立具体源复审早已PASS0，本次8lensreview也未新增该slice高风险缺口；scopeAudit独立回收既有真正审查而非另造执行。12acceptedsource迁移、5秒Task物理政策、完整生成/平台/main仍独立，不因specificfinding本地resolved关闭全goal。observe版本现在绑定真实76601，运行仍保原790输入与原时刻，不伪提交时重新执行。


## 历史证明存储的有界表示迁移

原id/业务正文/status/supports/limitations/命令及原观察含义不改变；本冻结父 `7f14a1b9c751b0612ceb42224f83f29d09aea46c` 中此对象完整原header/source refs可由公开证据tag恢复。只将具体archive物理依赖交当前位置资产与真实same-role consumer，全部旧义务由显式Eq/CT承接，不把存储重观察冒原业务新审或旧失败变成功。
