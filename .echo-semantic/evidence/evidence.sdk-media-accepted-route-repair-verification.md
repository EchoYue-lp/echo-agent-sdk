---
schema_version: 1
id: evidence.sdk-media-accepted-route-repair-verification
kind: evidence
observed_at: 04eb6ad50982656b5a8b8ea649619b6a383065ff
source_refs:
- contracts/sdk/accepted-facade-operation-catalog.json
- echo-sdk-host/src/core_profile/facade/tools.rs#dispatch_tool
- sdks/shared/testing/tool_behavior_fixture.py#prepare_media
- sdks/shared/testing/tool_behavior_fixture.py#verify_media
- sdks/typescript/test/tool-behaviors.test.js
- sdks/python/tests/test_tool_behaviors.py
- sdks/java/src/test/java/com/echoagent/sdk/ToolBehaviorsHostTest.java
supports:
- behavior.sdk-family-media
limitations:
- Actual before was TypeScript public SDK on original Host 0bbc; not three-language before.
- Excel seed XML parts match but ZIP timestamps and container SHA differ; PDF input SHA matches.
- ImageUrl model_content wire projection does not prove a model Run or viewer rendering.
- Awaited Session/Agent/SDK cleanup and fixture exit are not an independent Host PID death oracle.
- Focused source/build/after evidence only; final frozen Rust14, whole language entry and three quickstarts, main/platform and native Hook remain separate.
---

# 两个accepted媒体名字的原修复与有限验证

## 来源与范围

唯一修复Git为公开04eb6ad50982656b5a8b8ea649619b6a383065ff；其tools.rs源码SHA2568b892c9a4739277a0818caf740955cf480d36a561927e934429cd1986c887abf只在原media lookup映射pdf_extract→extract_pdf与excel_write→write_excel。沿同一framework Tool/schema/validate/execute/typederror/owner/context，不新parser/Backend/API/状态或catalog/pin/lock/generator变化。真实旧Host源码208a的相应文件77c1d305与发现driver17be逐字相同；不是把dirty修复假绑17be。原source增量独审C0/I0/M0，注释没有“predates”等历史推测。

## 支持的结论

before是旧0bbc TypeScript public SDK：有效596B PDF先pdf_info成功再pdf_extract unknown；有效1447B OOXML先read_excel成功再excel_write unknown，都是invalid_value/Never/原operation。raw分别872B/01dc8f7a4b3532f01e0d596025a96848ad96658ff33e6d5ea3d1005bf49f382f及875B/6d6baa8820c81d721c8a37f4d11aa36a96a10b5651b470b3413eb3bf8148ce88。更早启动目录和SDK操作名写法错误均未媒体RPC，保留但不作产品red。

新Hosta17db78e5d3e73a5edf9ef529d387a218aa248d7042b08e12b88b843cd53baaf实际新lib/bin构建0、零warning，保存opt1/assertions/overflow/unwind，159709568B，EH15491112/unwind2979912；1220 tracked build guard0、63 selected build inputs0（非完整rustc输入集）。原build raw38950B/4864e90e6d5ac3e267c44005564df4340520fc74fcd56d3183baf2feab38ee06。两原probe只改expectedHostSHA后各0：PDF原text及info，Excel原read控制与writer真实文件，raw220B/5d34fa8e2931c53a0f96d09567d6566edd78cd54e70f95e7ab7d28ecc1590155及224B/6dfc06dfbd9ebc24df3043d76dc1abb68bbc1aa1d51febf5117bf6912e768f12。PDF输入同SHA；Excel同脚本/同语义输入、五XML entry逐字同，但ZIP时间戳13:21:52和13:28:50不同、整体SHA不同，不能称ZIP byte-identical。

## 三语言真实媒体短链

同原ToolBehaviors test文件保原六族default并新增可选media sibling，旧fixture五函数/HTTP Handler/default scenarios/verifier/text oracles逐字保，非另造请求后台。TS/Python/Java各2pass/0fail/0skip，media每个19native call=12accepted成功操作+5typed RPC error+2structured ToolFailure。两aliases实际可消费，Excel由原writer生成→同文件read/info/profile→CSV，ZIP单元格、Unicode/3/7/均值5.00和UTF8 BOM CSV原bytes精等；DOCX2段落1表的text/info/structure、PDF单页metadata/text/pages、PNG MIME/metadata/bytes与低detail ImageUrl model_content精等；image_fetch检查原文本data URI和唯一GET，不捏模型content或Run。坏PDF/XLSX/DOCX、unsupported image/URL、invalid write/export均检查原typed语义及inputs/sentinels不变；empty effects只为已报告事实，真实文件另检。

三语raw依次17112B/6d196810a98e12fc0322be2dd2a5be39aacbd840a528e3f861a9f00a0bafead7、1597B/5af7308e629456378aa4e61cebeddfc53fe53962c67b1f4a6c80d91401fbbb30、1425B/4d6d1049440f4110d4541777b318c110747486db476933468711839e59c0254d；Java freshXML10478B/975740e91d8e35a61bbb74dee01c4cb139dd1b8ff0134e70393be858497eab43为2/0/0/0，实际Oracle21.0.5不冒Temurin17。每语言1220 source guard0、不可变Host SHA/mtime恒定，63file/74396B源副本核0mismatch；focused native proof70203B/289a131364886ba65de370d7e31e0276f24725da23d69c99c39d8672ebb458f7，完整原件只保ignored本任务local。

## 工程与精复审

fmt apply/check、严格production lib/bin Clippy（warnings及unwrap/expect/panic/unreachable）和原Rust六族case1pass/0fail/ignore/filter均actual0、零warning、1220guard0；那个Rust case未运行新增media19。focusedRust proof15372B/0a98c48dd0965d67151a59276c34546d142357db5a6637f345b464f16401f25b。原source/test/native/Java-build/formal/necessity读者均精scope C0/I0/M0。这一资产给出04eb公开源的实际repair与有限after，Finding已绑定本repair Evidence且仍open；verification/rereview引用及最终新冻结源完整门禁/精审继续待，不把focused after当整体验证。原ToolsBridge contract-evidence只追加这一精challenge，不提升整族或全部八风险组。

## 已知缺口

一个非加密单页PDF、DOCX、OOXML工作簿与PNG不证明任意codec/oversize/所有optional/外部viewer；没有独立Host PID生死oracle。新冻结源码最终完整Rust14、唯一官方全语言与3quickstart及精审尚待；不能借208a/0bbc旧整套。SDKmain/LinuxWindows/nativeHook和managed历史未知独立，后者仍open且没有虚构repair或人的风险接受。
