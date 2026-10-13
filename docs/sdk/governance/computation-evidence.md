# Accepted computation capability evidence

The formal shared corpus contains 50 native calls per language: the historical 45 calls plus five final Pivot boundaries. TypeScript, Python and Java each execute one normally discovered test against a real Host. The shared Python fixture prepares deterministic files and checks recorded outcomes; it never sends ACP requests or owns SDK/runtime state.

The focused receipt is [sdk-computation-20261013/receipt.json](evidence/sdk-computation-20261013/receipt.json). All three cases passed with zero failures, errors and skips against immutable Host SHA `7bbd1569367b0b1004fbe1cbc537b6869a2bc685bb1ca14e63228b1746aee3b2`. This proves the accepted routes below on that executable. Whole SDK gates, published framework provenance and exact main acceptance remain separate evidence.

| Capability / authority | Concrete behavior and failure evidence |
| --- | --- |
| Statistics / framework StatisticsTool | Sample 1,2,3,4 gives mean 2.5, sample standard deviation sqrt(5/3), quantiles 1.75/2.5/3.25, exact counts and exploratory-only metadata. Explicit nonnumeric column returns its documented column error; unknown column and missing required input return typed framework errors. The float oracle uses absolute tolerance `1e-12`, relative tolerance zero. |
| Structured validation / Host accepted schema-validation route | Valid and invalid samples; two exact error paths; missing sample differs from explicit null; invalid schema is `invalid_value`; schema beyond 262144 bytes is `payload_too_large`; twenty missing properties produce the sixteen-error cap. |
| Content guard / framework ContentGuard | `中😀 alice@example.com 尾` reports byte span 8..25 and exact UTF-8 redaction. Clean/detect/reject/redact and invalid mode are checked. Card/phone overlap in `4513812345678111` produces one complete `[REDACTED:CARD]` replacement. |
| Chart / framework GenerateChartTool | All seven chart types preserve data, Unicode title and 321×123 dimensions with marks line/bar/arc/point/area/rect/boxplot. Unsupported type/format, wrong numeric width, missing data and excessive trailing-null arity return explicit typed errors. This checks JSON chart specifications, not graphical rendering. |
| Data / framework Polars tools | Exact aggregate sums/means/counts, left-join values/nulls, missing-value percentages and range-rule violations; ordinary Pivot retains all four cells. Null index retains q1=2/q2=3. Literal string `null` versus actual null produces a clear ambiguous-label error without changing the input. Unknown and numeric aggregation fail; only absent aggregation defaults to sum. |
| File exports / framework tools and accepted Host binder | Valid JSON projection exports four exact id/value records; text export preserves `中文😀\nalpha` bytes. Unsupported data format, required-input omission, wrong optional numeric input and excessive trailing null cannot change an existing sentinel. |

Each language retains 19 complete typed errors (14 `framework_error`, four `invalid_value`, one `payload_too_large`), including message, canonical operation, retryability, optional handle and details. Framework errors retain `framework_failure_category=tool`, `framework_terminal_kind=failed`, `framework_failure_code=tool`, `framework_retryable=false`, and retry contract `never`. Their accepted envelope deliberately has no attached handle: `EchoSdkError::new` initializes `handle=None`, and the framework adapter does not invent one.

SDK aggregation failures are rejected by the framework parameter-schema validator before the native Pivot leaf executes. Their message identifies `pivot_data` and the enum/type violation; the framework's direct native-leaf tests separately prove `agg_function` validation. Explicit trailing null is governed by the accepted positional binder; this corpus does not claim that it is equivalent to the native leaf receiving a JSON null property.

Every call records before/after SHA-256 for all eight protected inputs, including the collision CSV and output sentinel. Both valid export bytes and the unchanged sentinel are checked independently. ToolResult raw `data` must equal the JSON parsed from `output` before comparison. [comparison.json](evidence/sdk-computation-20261013/comparison.json) compares all 50 ordered calls, arguments, result flags and complete errors across languages. It normalizes only isolated work-directory prefixes, WireValue map association order, and unordered Polars table rows/pivot labels. Original values, raw IDs and bytes remain in each language's `*-results.json`.

## Formal test entry points

- [Shared input/oracle](../../../sdks/shared/testing/computation_fixture.py)
- [Node native consumer](../../../sdks/typescript/test/computation-host.test.js)
- [Python native consumer](../../../sdks/python/tests/test_computation_host.py)
- [JUnit native consumer](../../../sdks/java/src/test/java/com/echoagent/sdk/ComputationHostTest.java)

Use a supplied Host with the required contract/features and existing language dependencies. From the SDK root, set `ECHO_AGENT_SDK_HOST` to its absolute executable path and `ECHO_AGENT_SDK_REQUIRE_HOST_E2E=1`. Set `TMPDIR` to an existing writable working directory; Java explicitly uses it as the parent of its native fixture. Run:

```sh
(cd sdks/typescript && node --test test/computation-host.test.js)
(cd sdks/python && PYTHONPATH=src .venv/bin/python -m pytest tests/test_computation_host.py -q -s)
(cd sdks/java && mvn -q -Dtest=ComputationHostTest test)
```

With required Host mode, missing or whitespace-only `ECHO_AGENT_SDK_HOST` fails before optional skipping. Python/Node missing-and-blank rejections are recorded in [required-host-negative.json](evidence/sdk-computation-20261013/required-host-negative.json); JUnit checks `isBlank()` before `assumeTrue`. The three positive cases actually ran without skipping. The scoped Java XML has exactly one case, not 50 JUnit tests; the 50 figures count native scenario rows. Existing ACP dependency SLF4J no-provider notices are retained in the raw Maven log.

## Evidence and historical association

[The evidence directory](evidence/sdk-computation-20261013/) retains final logs, raw results, full scenario inputs, the immediate one-class Surefire XML, compile-input hashes, frozen test sources and the comparison receipt. `source-freeze-before.json` and `source-freeze-after.json` match for all twelve recorded inputs, including client/codec implementations and accepted catalog/digests.

`historical/` contains byte-preserved temporary inputs, consumers and observations. The earlier `c52` Host exposed incomplete overlap redaction and quoted/null Pivot cells. The `64c7` temporary Host proved the first two repairs but did not contain the later null-index/collision/aggregation fixes; its receipts must not replace the final executable's evidence. Earlier statistics argument-order and public content-guard route failures remain identifiable as their original records.

The first formal oracle additionally made an incorrect assumption that every framework error carried a Session handle. Its Python/Node failure logs are archived separately; the correction follows the existing optional-handle contract and does not modify runtime behavior. A comparison-script compatibility correction removed Python-3.10-only `zip(strict=True)` because the shared subprocess can run the system Python 3.9; the already checked fifty-row cardinality remains mandatory.

No public API, accepted membership, production adapter, generated contract, Rust implementation, example or website behavior changes in this test-only slice. Builtin validation/effects are proven here; custom Tool validation override behavior is covered by the separately owned Host unit evidence. The runtime tests and this formal SDK document are the relevant executable/documentation surfaces, so no example or website synchronization is needed. npm/PyPI/Maven publication remains outside this work.
