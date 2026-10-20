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

## 工程与精复审（9ef材料阶段历史）

fmt apply/check、严格production lib/bin Clippy（warnings及unwrap/expect/panic/unreachable）和原Rust六族case1pass/0fail/ignore/filter均actual0、零warning、1220guard0；那个Rust case未运行新增media19。focusedRust proof15372B/0a98c48dd0965d67151a59276c34546d142357db5a6637f345b464f16401f25b。原source/test/native/Java-build/formal/necessity读者均精scope C0/I0/M0。这一资产给出04eb公开源的实际repair与有限after，Finding已绑定本repair Evidence且仍open；verification/rereview引用及最终新冻结源完整门禁/精审继续待，不把focused after当整体验证。原ToolsBridge contract-evidence只追加这一精challenge，不提升整族或全部八风险组。

## 媒体闭合前的已知缺口（历史）

一个非加密单页PDF、DOCX、OOXML工作簿与PNG不证明任意codec/oversize/所有optional/外部viewer；没有独立Host PID生死oracle。新冻结源码最终完整Rust14、唯一官方全语言与3quickstart及精审尚待；不能借208a/0bbc旧整套。SDKmain/LinuxWindows/nativeHook和managed历史未知独立，后者仍open且没有虚构repair或人的风险接受。


## 2026-10-20 最终新源码完整工程验证

公开修复source_refs继续绑定04eb；本次实际运行Git为9ef071b7c8724ff2528d7ba06558f002cec02e28/tree736c6b0e481fcc1d5cfc75105e92002834b241f5。04eb→9ef仅正式材料变化，production/test/dependency/contracts/CI全部字节相同；不把旧208a资格借给新a17源码。

原14条适用Rust命令在9ef冻结1222输入完整自然退出0，零编译warning/资源bound/源码漂移；proof112197B/SHA25688fb4f1d874a314885d41b822b7e6b00abe20a03bd2e48c2dab165b467787956。workspace alltargets/allfeatures单一配置13个harness为521pass/0fail/0ignore/0filtered，其它配置不相加为unique。Protocol42/9/5/22/42/91、defaultHost31+4、core94+14+4、bridge29、facade33+12、无bridge准入1pass/25filtered分别保原profile；第7项两crate各0doctest、第11项no-run仅编译。原Java增量工程复审完整14为C0/I0/M0，不重跑已过matrix。

随后原explicit-v fullfacade Host build actual0/Fresh，产物159709568B仍SHA256a17db78e5d3e73a5edf9ef529d387a218aa248d7042b08e12b88b843cd53baaf，与本修复实际rustc产物一致；保opt1/assertions/overflow/unwind，EH15491112/unwind2979912。此Fresh不虚构本次rustc，raw19215B/028213c30b43cf36e2a9602abd396870b698bc24328d77108991995a145574a5；1222 guard0与63selected build inputs（非完整rustc inventory）分别记录。

唯一原scripts/check-language-sdks.sh在07:14:18Z→07:19:05Z自然actual0，无suite筛选或profile改动，原Host+collector/npmci/fullTS/PY/Java及三个quickstart均执行。TS358/0fail/skip；Python442加18子测试；Java379/0failure/error/skip、76freshXML，实际Oracle21.0.5不冒CI Temurin17。1222源码零漂移、同不可变Host SHA/mtime恒定、原运行guard未bound；#120原各12个具名场景均在此whole，不新增重跑。raw129497B/b1bb61c92da91f88ed4fb3d726d0de39e4011680c8e88dade5628223828a0ebd；wholeproof54903B/4fb72ff205762efe14e2d2a8032c494ba6ebf7b37ff16268d1e5ecfcec4c2062。原Java读者对完整入口、quickstarts、freshXML、guard与本次各12原件C0/I0/M0。

本次150目录/2418file原件source-to-copy核0mismatch，逻辑4347404314B，manifest1619023B/61d3dbba05dc755703eec04a627154d20ee18275482dd40dddb37212f0a801e0不变。后处理copy2把pytest父目录中旧+新两个稀疏2GiB文件展开，曾造成归档阶段free4917166080B低于5GiB；原whole运行最低free9681469440B且未bound，二者明确分开，不改旧日志或重跑suite。按原授权逐源/副本/manifest完整SHA、regular/nlink1/no-live/mode/mtime/并发未变检查后，两个新归档副本以zero-seek/truncate临时文件全SHA验证再atomic replace，逻辑bytes/path/mode/mtime不变、回收allocated4294705152B；原source、manifest、cache/target/tmp不删除。sparseproof8923B/6a433197640f5a4f2f52b536db765ea7ef2c13befece6251e530929c78f4a67d；最后独立遍历2418file全manifest SHA/size为0mismatch，收据943B/481c6f045ce6c98e41c88dbaa062c85ac8a0fe36702f541caffc1182e4f9e0d6。这个工程budget事件不能被抹成后处理全程floor合格，也不使原0终态成为新的测试失败。所有原件/proof只保ignored local，未进入Git。

SDK独立源码/媒体5源增量/typed native/newHost/14门禁/唯一whole/后处理字节保全及必要性读者各精scope C0/I0/M0。上述真实匹配后验支持两alias Finding的verification，与公开04eb实际repair及原ToolsBridge精challenge一起闭合本项；不据此把97Audit、maps或四个仍heldcomposite批量升格。

## 已知缺口

只结算原accepted两alias与十二non-text具名消费；任意codec/oversize/所有option/viewer或模型Run未外推。SDK精main实际9check、平台CI/Hook、原历史证据全必要ref冷strict仍独立。managed历史Linux单次passive-exit根因未知，保持open/无repair；诊断增强/同case回归通过不伪原因或人的riskaccept。
