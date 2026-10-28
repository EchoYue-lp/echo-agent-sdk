# 三语言源码包的自包含合同

当前三语言产品仍为 source-only；TypeScript 的 `private=true` 和 Rust 的 `publish=false` 保持。构建本地 tgz、wheel、sdist、JAR 只是发行准备，不表示上传注册表或正式发布。

包必须携带默认运行需要的 `facade-operation-catalog.json` 和 `contract-digests.json`，不能依赖分发目录之外的 `sdks/shared`。这两份文件仍由唯一入口 `scripts/export-language-sdk-catalog.sh` 从 accepted catalog、extension schema 和 source-contract 生成；TypeScript 的 `contracts/`、Python 包内 `contracts/`、Java classpath `com/echoagent/sdk/contracts/` 都是受检查的分发副本，不拥有第二套 accepted 范围。

TypeScript、Python 的默认 FacadeCatalog 从包内加载；Java 在现有构造参数的 catalogPath／digestPath 为 null 时从 JAR resource 加载，显式路径仍可分别指定。不能找不到当前包资源后静默读取另一个工作树的 shared 文件。

`export-language-sdk-catalog.sh --check` 对三份副本逐字节核验。语言测试覆盖本地资源；TypeScript 测试还真实打包、解包并从仓库外加载默认 catalog。发行准备另检查 wheel／sdist 和 JAR 的实物内容、默认加载及文件 SHA，不能只以 pack 命令退出 0 作为包可用证据。

旧 Host 对新 schema digest 的不匹配必须保留。旧 pin 消费证据显式绑定保全的旧 catalog／digest；后继 Host、纯协议、三语言和示例须重新绑定相同生成切点，不把旧测试通过续到新来源。新 framework pin、source 402、安全、main CI 与发布资格仍分别验收。
