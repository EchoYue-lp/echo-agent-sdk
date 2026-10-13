# 官方 Ruff 上下文导入格式修复

官方完整第二轮语言门禁在Node290项/quickstart通过后，Python官方Ruff0.16.6发现7个I001。只修paths.json所列6个Python消费者测试与Channel结果比较夹具的导入分组；compare文件按同官方formatter必要折行。没有改生产clients、代码逻辑、断言、skip、UV依赖或Ruff配置豁免。

修复命令在sdks/python目录沿用原uv --no-project --with ruff环境，仅ruff check --select I --fix指定7file与ruff format相同文件。原完整Ruff check src tests examples ../shared/testing及format --check都最终0，175文件已格式正确。first formatter发现compare文件未formatted的日志保留，不抹失败。

ast-preservation.json固定非import语法树逐字段相同及所有import语句multiset相同；managed_retirement把test_managed_host import在SDK模块之前是导入分组排序，不能宣称整个AST字节顺序完全不变。前后源码与每filepatch保留。

第三官方完整脚本随后Node290/quickstart和Ruff2命令再次通过，却在Python旧pytest shebang使用搬迁前解释器时collection88errors停止；这是另一入口缺陷，未以本包声明整个languagegate通过，原上级admin官方runcommand日志仍保。本包只支持格式范围。
