# Python 测试入口跟随 UV 选定解释器

第三官方完整语言脚本在Node290/quickstart及Ruff两个命令通过后，Python测试collection88errors（缺acp）停止。相同UV参数/同cwd的python实际能import acp/pytest9.1.1；pytest launcher shebang指向SDK worktree搬迁前已不存在的解释器，直接launchFileNotFound，而python -m pytest可用。interpreter-diagnosis.log保精确解释器与模块来源，不把88collectionerror当被测行为失败。

唯一脚本修改：保留uv --no-project --with pytest --with pytest-asyncio --with agent-client-protocol==0.12.1和全部tests ../shared/testing路径，仅pytest -q改python -m pytest -q。没有更新环境/依赖、过滤case、skip或改变断言。bash -n检查通过。

修后相同UV入口实际运行全部Python测试357passed+18subtestspassed，quickstart echo-agent成功，官方run-command receipt exit0。Host为不可变b909-natural-eof8d819；此分段进展不是整个check-language-sdks.sh通过，也不能代替最终合同/main门禁。随后Java全suite311tests有1failure8errors0skip，原始日志XML由主线程分配默认路径修复；此包不称Java绿。

原完整脚本collection日志保在前次sdk-language-after-import-format收据；本包固定before/after脚本源码、单行diff、实际解释器诊断、Python全集日志/receipt/config。
