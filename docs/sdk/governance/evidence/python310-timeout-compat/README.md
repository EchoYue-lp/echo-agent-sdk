# Python3.10超时测试夹具兼容证据

声明Python>=3.10，六份老测试中9处asyncio.timeout使用3.11新增API。实际CPython3.10.19原barrier体复现AttributeError，原完整SDKimport还先暴露typing.NotRequired缺口；后者由[独立生产导入修复](../python310-import-compat/receipt.json)负责，不把未导入SDK的函数体检查当完整消费者证明。

本切片仅把9个原轮询等待体提取为相邻localasync函数，用已有asyncio.wait_for保相同2/5/10/15/35秒deadline。SDK/callback/cleanup owner任务没有迁入新poll任务，调用方取消只取消并等待自己的poll；所有原assertAST保持，不增睡眠、skip、兼容模块或依赖。Rules新测试超时由独立owner修，不在这六份源码改动中。

真实3.12旧六份focused18项通过；最终生产import源码cd07640f...和Rules测试528cce...冻结后，实际CPython3.10.19同七份文件focused19项全部通过47.23秒、0skip。前面的相邻源码18pass和独函数ready/cancel结果仍是历史阶段，最终以final-source-focused310.log及final-production-and-rules-source.json为准。19项是有限定向验证，不是全SDK或完整官方语言suite通过。

原Admin32条清单及其自身共33文件逐字节复制在raw/，仅新增本README和archive-manifest；失败、源码前后、逐filediff、AST判据、实际版本、真实日志和摘要原时间/路径保留。不复制CPython运行态或SDKtarget目录，未触其它原始证据。主线程传达独立review已对六文件9poll、最终19case与生产import兼容闭包PASS/action0，所有Minor清；此次只供归档增量复核，无源码改动或测试重跑。

生产导入仍由独立包负责，Rules三语言真实模型输入证据由其owner归档；本包不扩写三语言公共API、SDKmain交付或源码最终全局语义闭合。归档改变治理文档树，不能将历史runtime重标为后续新快照执行。
