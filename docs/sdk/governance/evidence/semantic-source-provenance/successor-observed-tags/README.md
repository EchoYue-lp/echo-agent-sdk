# 后继 observed-source 证据 tag 远端恢复

原 [八个 tag fresh fetch 收据](../receipt.json) 已由独立审查核实：真正空 bare 仓库从远端获取，没有 alternates。此次先只读验证该八个 annotated tag 的 peeled commit/tree 仍匹配原收据、九个新 ref 全不存在，再用同一远端对象基线增量 fetch 九个新 tag。真实 fetch 退出0，进度记录接收1796objects并完成delta结算；九个实际 annotated tag 对象、peeled commit、tree 全与原预计清单匹配，原八个保持匹配，总17refs。没有从工作树复制object或添加alternates。

这是“原真实远端八个基线＋九个真实远端增量”的恢复证据，不是“另一个空仓库九个fetch成功”。此前新空 bare 的三次尝试分别为原SSH自有进程SIGINT -2、有界SSH90秒124、HTTPS120秒124。三份原stderr、receipt、expected与结果按原字节另存 history/empty-bare-attempts；没有将失败改成成功。无进度日志不能证明握手故障，后来的真实progress显示增量pack传输较慢；这里不据先前earlyEOF猜网络故障根因。

created-new-tags.json 记录Root实际已推送且无force的九个具名tag。incremental/baseline.json 记录原八个来源与九个预计object/commit/tree；fetch.log、fetch-receipt.json和result.json保留实际远端命令、完整进度和最终比对。archive-manifest固定每个Admin原路径/字节摘要。只复制这些小proof文件，没有复制bare数据库、node源码或既有原八个正式归档，也没有修改任何immutabletag。此包未再执行网络操作。

恢复范围仅为列表中的九个新增＋八个原tag。后继productionSource622d、之后模型observed快照以及未来最终current源码tag不属于九个列表，需要各自保留并验证；17refs恢复不能替代其证据，也不表示SDKmain、release、注册表发布或全能力完成。source_refs 当前相对链接持久化与这份对象恢复证明分别由主线程绑定。归档材料独审仍待主线程承接，本README不自行宣称PASS。
