# 两个 TypeScript Host fixture 的独立文件状态目录

原官方完整语言脚本 Node 全集287项有285通过/2失败，不能由本包证明全套已通过；其中一项是两个独立Host并行复用同一配置、state_root/memory_path，框架 FileRuntimeStateStore 正确拒绝第二个文件authority owner（OS35）。另一项 Task pause resume 的 extension exclusive lease 风险由其它owner处理。

Supreme debug Phase1-3调查及同两文件默认并行原命令稳定复现3tests2pass1fail后，仅修改两个测试 caller：为每个Host mkdtemp复制原完整配置，改sdk_profile.state_root及原已存在memory_path到自己的目录；保原模型、Agent、limits与全部操作断言，close成功之后才删除自己目录。没有禁用框架文件锁、没有串行测试、没有跳过case、没有改生产/schema/scripts或新增helper。

原 `host.test.js` 33行 assert、`family-route-host.test.js` 3行 assert 与修改前完全一致（assertion-preservation.json）。相同默认并行命令、相同Host8d819以及相同外层配置，最终源码运行3passed0failure0skip；原before/finalafter/log/officialrun-command receipt/source/diff都逐字节归档，保留原时间/绝对路径有助恢复历史。

命令：`node --test sdks/typescript/test/host.test.js sdks/typescript/test/family-route-host.test.js`，外层ECHO_AGENT_SDK_CONFIG=shared-host.json（本包固定原数据）；原实际 Host路径/hash绑定manifest。before和after完整收据在before-receipt.json与after-cleanup-final-receipt.json；对应log、源码副本在before-source和after-source。debug-phases.md解释唯一区别与权威。after-parallel是前一次验证，最终以after-cleanup-final为准，不覆盖原失败。

独立只读审查 framework_final_review 对两文件、最终源码、原23文件包以及assert preservation确认 PASS/action0。归档为同字节搬运，仅此README与archive-manifest说明新增；没有再运行native，也未修改任何已review业务源。任务范围语义校验仍有全任务基线/依据/大文件等未完成项，semantic-task-scope.log保留真实305错误，最终由共同语义owner绑定；本包不宣称全局语义、完整语言脚本或SDKmain验收通过。
