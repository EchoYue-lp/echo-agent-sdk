---
{
  "schema_version": 1,
  "id": "evidence.sdk-memory-permission-candidate",
  "kind": "evidence",
  "observed_at": "source:3ea544c47e331b5d29a6624f978f3595619bef39fa5985230351098d38930404",
  "source_refs": [
    "docs/sdk/governance/evidence/sdk-memory-permission-20261013/formal-manifest.json",
    "docs/sdk/governance/evidence/sdk-memory-permission-20261013/temporary-consumer/cross-language-comparison.json",
    "docs/sdk/governance/evidence/sdk-memory-permission-20261013/temporary-consumer/ACTUAL-RESULTS.md",
    "docs/sdk/governance/evidence/sdk-memory-permission-20261013/temporary-consumer/scope-disposition.json",
    "sdks/shared/testing/memory_permission_fixture.py",
    "sdks/python/tests/test_memory_permission_host.py",
    "sdks/typescript/test/memory-permission-host.test.js",
    "sdks/java/src/test/java/com/echoagent/sdk/MemoryPermissionHostTest.java",
    "docs/sdk/governance/evidence/sdk-memory-permission-20261013/formal/TEST-com.echoagent.sdk.MemoryPermissionHostTest.xml",
    "docs/sdk/governance/evidence/sdk-memory-permission-20261013/formal/cross-language-comparison.json",
    "docs/sdk/governance/evidence/sdk-memory-permission-20261013/formal/sdk-memory-permission-formal-java.log"
  ],
  "supports": [
    "behavior.sdk-family-memory",
    "behavior.sdk-family-permission"
  ],
  "limitations": [
    "actual TTL expiration not observed; accepted contract exposes no shortTTL setter",
    "approval_receipt is new framework field not in current acceptedfield inventory; no automatic API expansion",
    "generic numeric Store callback roundtrip must preserve WireValue type; current fixture does not claim arbitraryu64store-nativeprojection coverage",
    "59 evidence rows per language are not59testcases; formal TS/Python each1test retain same scene assertions.",
    "New SDK Tool validation and later source changes do not inherit old6711 whole-run pass."
  ]
}
---

# Memory、Permission 真实消费者候选

## 支持的结论

三语言临时公开 API consumer 在6711 Host上各记录59条证据，包括52个family operation结果、两个完整typed failure、11种reverse Store callback路由、真实model recall/journal、permission上下文与修改args，以及实际拒绝/批准的Tool文件副作用。正式TypeScript/Python各一次测试已执行同场景；测试计数不替代每项断言。

Memory沿same Session由Store A切到B，保实际semantic registration单一约束与旧A数据；pre-write callback失败不写入，随后真实成功重试。native Store的expiry/prune、dedup和owner错误有直接结果。新native Session recall只把带真实fixture证据的Active记录送入model，Draft/Legacy不进入实际user输入；journal memory_recalled=1、唯一event ID/连续sequence/唯一final_answer。

Permission跨两个Session验证exact args/session_tool cache、once不缓存、narrow revoke、关闭A保B；HumanLoop callback保留完整context与modified args。真实Run拒绝write时无callback/文件，approved_once时一次callback且文件UTF-8内容精确，终态由framework receipt决定。

## 来源与范围

原临时输入、raw输出、callback/model/journal/sideeffect与错误保存于归档，comparison声明各自normalization：独立路径、已逐个验证canonicalu64的时钟、equal-score无序、dedup原key/content的指定等价。confidence使用显式既有f64 wire，不改变native类型。原Host SHA6711dd17fe3be13671ee181fce5ee79946f40963e000af293083f114301a39a5；这些是原来源的具名消费者断言，不冒充新的Tool validator或全scope最新通过。

## 已知缺口

实际30分钟 permission TTL 过期未观察；已接受合同没有短 TTL setter，不能编造新 API 或默认闭合。framework 新增 approval_receipt 尚不属于当前 accepted 字段，不隐式扩范围。正式 Java consumer 已在 cb9 Host 执行一项 JUnit 测试、零 failure/error/skip，覆盖同一59行场景；来源与 XML 见下节。最终 source/pin、匹配的完整语言门禁与精确 main 仍待，所有 Behavior 保持 needs_review。

## 正式Java消费者执行

正式MemoryPermissionHostTest XML记录1test、零failure/error/skip，匹配59row场景而不是59testcase。原临时6711消费者与正式cb9测试的源/epoch保持分开；不将新增实际证据推广TTL过期或未接受approval_receipt。XML SHA 07df10f13df041c6d39ccae32718e4ddae8cf669856773a2d1be953c97d54bd5。

正式cb9语料比较：三语言原生门禁test复制后各1test、59row仍精确对齐，formal comparison/raw inputs/logs单独保留；其classification文字仍写temporary为原receipt字段，不将其伪改为全mainproof。旧6711临时语料/clock/ID原记录保留，新的正式运行只按自己的实际来源支持。
