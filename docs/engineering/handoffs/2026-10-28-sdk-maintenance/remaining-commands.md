# Root可排的最小验证与尚未执行命令

当前人类维护优先，不在本线执行新Cargo、native/App、完整重型矩阵、模型或外部业务调用。
以下是恢复后的明确入口；占位处必须换成Root核准的真实来源/路径，不能用占位pin或假锁。

## 仅保存资料的提交准备

Root按六份工程资料stage后，正常运行所属repo的预检/Task门禁及staged artifact检查。
该Scope不宣称155路径产品代码通过。原业务Task仍是 `sdk-native-consumer-20261027`，owner SID
`01a17d6c-e195-7f90-a8a4-8da3dc80a009`；现存完整合同不能被手工覆盖或假清空。

```sh
node /Users/ls/.codex/plugins/cache/echo-semantic/echo-semantic/0.5.0/scripts/semantic-verify.mjs --root /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk --task-id sdk-native-consumer-20261027 --session-id 01a17d6c-e195-7f90-a8a4-8da3dc80a009
/opt/anaconda3/bin/python3.12 -B /Users/ls/.codex/plugins/cache/echo-semantic/echo-semantic/0.5.0/scripts/artifact_contract.py --root /Users/ls/MyWork/code/ylp_agent_learn/lp-agent/echo-agent-sdk --staged
```

原实际TaskSemantic exit1是退役等价Evidence的after_revision未绑定当前Task；原件已保，不能改matched刷绿。
这不是新SDK工程包可以自授的构建资格。最新正常诊断由维护handoff.json记录，不宣称已通过。

## 原正式f7/0dac下可独立排的focused检查

维护保存若需验证原纯Protocol切片，可由Root安排独立target/jobs≤5/debug=0：

```sh
cargo fmt --all -- --check
cargo test -p echo-sdk-protocol --locked --test a2a_lsp_retirement --test native_message_contract --test core_rpc_contract
/opt/anaconda3/bin/python3.12 -B scripts/check-a2a-lsp-retirement.py
scripts/export-language-sdk-catalog.sh --check
```

这不需要native/App或完整重型矩阵，也不证明当前Host原生正控在f7可用。
当前Native adapter tests已切为新源正控；正式f7不具全部新原生字段/事件，不能靠只跑纯Protocol掩盖该缺口。
已有命令失败仍保持失败，按正常授权处理，不使用no-verify或新增skip。

## rich候选真正采用前

先补完精patch独审；Root核准真实Frame发布/受控读窗后，逐baseSHA合流而非回灌旧文件。
原唯一schema/包资源生成入口、完整producer/accepted delta仍须实际执行，不能手改生成物。

```sh
cargo metadata --manifest-path <新的受控consumer/Cargo.toml> --format-version 1 --locked
cargo test --manifest-path <同一consumer/Cargo.toml> -p echo-sdk-protocol --test native_message_contract --test extension_contract --locked
cargo test --manifest-path <同一consumer/Cargo.toml> -p echo-sdk-host --lib --no-default-features --features sdk-facade-all --locked message_input_tests -- --test-threads=5
cargo test --manifest-path <同一consumer/Cargo.toml> -p echo-sdk-host --test core_profile_e2e --no-default-features --features sdk-facade-all --locked task_ -- --test-threads=5
```

源稳定后真实构建同源Host/collector，再运行原 `bash scripts/check-language-sdks.sh`，绑定该次实际binary SHA。
原4a99只资格化1490组合，新删后源必须正常生成/核对其真实受控锁，不假定旧锁适用。
正式新Git pin、依赖风险合同、source402/fresh、完整Rust/平台、mainCI和发布各自结算；本保存不触发它们。
