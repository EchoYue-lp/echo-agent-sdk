# 已发布 framework b909 与自然 EOF 修复后的有限完整消费者证据

首次三个完整原生命令均通过，9 个测试、0 failure/error/skip。每语言的普通工具共同语料49行/48public调用；文件、artifact、shell、web、数据库、研究实际字段及副作用由同一正确值 oracle 检查。工具跨语言比较45个确定性行与4个固定官方服务时点观察；SearchProvider为真实Agent两个Run、1次成功+3次原框架失败重试，Sandbox为3真实producer（正常、pending cancel、pending close）/language。

正常 Sandbox 输出、完整结果 item、最终 EOF 是 sequence1/2/3；9个 producer 都 physically exited，finally/cleanup各一次；关闭与正常 EOF 返回时原producer已经结算。cancel RPC只表示请求，可先于原cleanup；Python/Node -15、Java143含义同SIGTERM且原值保留。完整错误、路径、U64、cursor/null、artifact原文件压缩与SHA、真实model/replay/IO/SQL文件不变哈希均在各语言raw中。

前置反例链保留于上级 before-0435、context-0a0f、settlement-f852、published-framework-b909：DB动态列全NULL、Web artifact上下文丢失、Sandbox canonical入口漏匹配、pending callback取消未达、正常EOF重复轮询panic、自然EOF取消cleanup导致随机跳过已发布result。此前错误与重复观察未被覆盖；本after消费者4源在修复前后未放宽断言。

manifest.json绑定实际Host SHA8d819004...、published Git framework b909与16份实际运行source/catalog/client（含既有 tagged ToolResult decoder）、Java编译类摘要；完整日志和JUnit XML在上级。current source contract仍是受控候选、完整accepted identity生成/SDKmain验收未完成，本包不能证明该范围。

外部搜索结果可变：Semantic Scholar当次实际429按明确错误合同验证，未skip亦未造固定端点返回。只证明default ShellTool实际tail超时/SIGTERM退出，不声称后代树；Sandbox本包只证明pending callback取消和关闭，post-answer cancellation不扩写。
