"""Deterministic computation inputs and independent oracle; never sends ACP requests.

Native language tests own all real Host calls. Only unordered table rows are sorted;
errors, wire variants, UTF-8 spans and exported bytes remain exact observations.
"""

from __future__ import annotations

import argparse
import hashlib
import io
import json
import math
import unittest
from pathlib import Path

from file_sha256 import file_sha256

SDK = Path(__file__).resolve().parents[3]
ROWS_TEMPLATE = [
    {
        "id": "stats-exact",
        "operation": "exploratory_statistics",
        "arguments": ["value", "$WORK/stats.csv"],
        "expected_error": None,
    },
    {
        "id": "stats-nonnumeric",
        "operation": "exploratory_statistics",
        "arguments": ["label", "$WORK/stats.csv"],
        "expected_error": None,
    },
    {
        "id": "stats-unknowncolumn",
        "operation": "exploratory_statistics",
        "arguments": ["unknown", "$WORK/stats.csv"],
        "expected_error": "framework_error",
    },
    {
        "id": "structured-valid",
        "operation": "structured_output.validate",
        "arguments": [
            {
                "type": "object",
                "properties": {
                    "count": {"type": "integer", "minimum": 1},
                    "name": {"type": "string"},
                },
                "required": ["count"],
                "additionalProperties": False,
            },
            {"count": 2, "name": "中文😀"},
        ],
        "expected_error": None,
    },
    {
        "id": "structured-invalid",
        "operation": "structured_output.validate",
        "arguments": [
            {
                "type": "object",
                "properties": {
                    "count": {"type": "integer", "minimum": 1},
                    "name": {"type": "string"},
                },
                "required": ["count"],
                "additionalProperties": False,
            },
            {"count": 0, "name": 3},
        ],
        "expected_error": None,
    },
    {
        "id": "structured-no-sample",
        "operation": "structured_output.validate",
        "arguments": [
            {
                "type": "object",
                "properties": {
                    "count": {"type": "integer", "minimum": 1},
                    "name": {"type": "string"},
                },
                "required": ["count"],
                "additionalProperties": False,
            }
        ],
        "expected_error": None,
    },
    {
        "id": "structured-null-sample",
        "operation": "structured_output.validate",
        "arguments": [
            {
                "type": "object",
                "properties": {
                    "count": {"type": "integer", "minimum": 1},
                    "name": {"type": "string"},
                },
                "required": ["count"],
                "additionalProperties": False,
            },
            None,
        ],
        "expected_error": None,
    },
    {
        "id": "structured-bad-schema",
        "operation": "structured_output.validate",
        "arguments": ["not-a-schema"],
        "expected_error": "invalid_value",
    },
    {
        "id": "structured-too-large",
        "operation": "structured_output.validate",
        "arguments": [{"description": "$OVERSIZE"}],
        "expected_error": "payload_too_large",
    },
    {
        "id": "structured-error-bound",
        "operation": "structured_output.validate",
        "arguments": [
            {
                "type": "object",
                "properties": {
                    "p0": {"type": "integer"},
                    "p1": {"type": "integer"},
                    "p2": {"type": "integer"},
                    "p3": {"type": "integer"},
                    "p4": {"type": "integer"},
                    "p5": {"type": "integer"},
                    "p6": {"type": "integer"},
                    "p7": {"type": "integer"},
                    "p8": {"type": "integer"},
                    "p9": {"type": "integer"},
                    "p10": {"type": "integer"},
                    "p11": {"type": "integer"},
                    "p12": {"type": "integer"},
                    "p13": {"type": "integer"},
                    "p14": {"type": "integer"},
                    "p15": {"type": "integer"},
                    "p16": {"type": "integer"},
                    "p17": {"type": "integer"},
                    "p18": {"type": "integer"},
                    "p19": {"type": "integer"},
                },
                "required": [
                    "p0",
                    "p1",
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7",
                    "p8",
                    "p9",
                    "p10",
                    "p11",
                    "p12",
                    "p13",
                    "p14",
                    "p15",
                    "p16",
                    "p17",
                    "p18",
                    "p19",
                ],
            },
            {},
        ],
        "expected_error": None,
    },
    {
        "id": "guard-utf8-detect",
        "operation": "content-guard.detect",
        "arguments": ["中😀 alice@example.com 尾"],
        "expected_error": None,
    },
    {
        "id": "guard-utf8-redact",
        "operation": "content-guard.redact",
        "arguments": ["中😀 alice@example.com 尾"],
        "expected_error": None,
    },
    {
        "id": "guard-clean",
        "operation": "content-guard.is_clean",
        "arguments": ["中😀清洁尾"],
        "expected_error": None,
    },
    {
        "id": "guard-detect",
        "operation": "content-guard.check",
        "arguments": ["detect", "中😀 alice@example.com 尾"],
        "expected_error": None,
    },
    {
        "id": "guard-reject",
        "operation": "content-guard.check",
        "arguments": ["reject", "中😀 alice@example.com 尾"],
        "expected_error": None,
    },
    {
        "id": "guard-redact",
        "operation": "content-guard.check",
        "arguments": ["redact", "中😀 alice@example.com 尾"],
        "expected_error": None,
    },
    {
        "id": "guard-illegalmode",
        "operation": "content-guard.check",
        "arguments": ["illegal", "中😀 alice@example.com 尾"],
        "expected_error": "invalid_value",
    },
    {
        "id": "chart-line",
        "operation": "generate_chart",
        "arguments": [
            "line",
            "中😀标题",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
        ],
        "expected_error": None,
    },
    {
        "id": "chart-bar",
        "operation": "generate_chart",
        "arguments": [
            "bar",
            "中😀标题",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
        ],
        "expected_error": None,
    },
    {
        "id": "chart-pie",
        "operation": "generate_chart",
        "arguments": [
            "pie",
            "中😀标题",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
        ],
        "expected_error": None,
    },
    {
        "id": "chart-scatter",
        "operation": "generate_chart",
        "arguments": [
            "scatter",
            "中😀标题",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
        ],
        "expected_error": None,
    },
    {
        "id": "chart-area",
        "operation": "generate_chart",
        "arguments": [
            "area",
            "中😀标题",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
        ],
        "expected_error": None,
    },
    {
        "id": "chart-heatmap",
        "operation": "generate_chart",
        "arguments": [
            "heatmap",
            "中😀标题",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
        ],
        "expected_error": None,
    },
    {
        "id": "chart-boxplot",
        "operation": "generate_chart",
        "arguments": [
            "boxplot",
            "中😀标题",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
        ],
        "expected_error": None,
    },
    {
        "id": "chart-unknown",
        "operation": "generate_chart",
        "arguments": [
            "illegal",
            "bad",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
        ],
        "expected_error": "framework_error",
    },
    {
        "id": "chart-wrongformat",
        "operation": "generate_chart",
        "arguments": [
            "bar",
            "bad",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "png",
        ],
        "expected_error": "framework_error",
    },
    {
        "id": "chart-trailingnullarity",
        "operation": "generate_chart",
        "arguments": [
            "bar",
            "bad",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            321,
            123,
            "json",
            None,
        ],
        "expected_error": "invalid_value",
    },
    {
        "id": "data-aggregate",
        "operation": "aggregate_data",
        "arguments": ["$WORK/input.csv", "group", "value:sum,value:mean,value:count"],
        "expected_error": None,
    },
    {
        "id": "data-join",
        "operation": "join_data",
        "arguments": [
            "$WORK/input.csv",
            "$WORK/lookup.csv",
            ["id"],
            "left",
            None,
            None,
            None,
            10,
        ],
        "expected_error": None,
    },
    {
        "id": "data-pivot",
        "operation": "pivot_data",
        "arguments": ["$WORK/input.csv", "group", "quarter", "value", "sum"],
        "expected_error": None,
    },
    {
        "id": "data-missing",
        "operation": "missing_value_analysis",
        "arguments": ["$WORK/quality.csv"],
        "expected_error": None,
    },
    {
        "id": "data-consistency",
        "operation": "consistency_check",
        "arguments": [
            "$WORK/quality.csv",
            '[{"column": "value", "type": "range", "min": 0, "max": 50}]',
        ],
        "expected_error": None,
    },
    {
        "id": "data-join-keyfail",
        "operation": "join_data",
        "arguments": ["$WORK/input.csv", "$WORK/lookup.csv", ["unknown"], "inner"],
        "expected_error": "framework_error",
    },
    {
        "id": "data-pivot-colfail",
        "operation": "pivot_data",
        "arguments": ["$WORK/input.csv", "group", "quarter", "unknown", "sum"],
        "expected_error": "framework_error",
    },
    {
        "id": "data-export-preserve-sentinel",
        "operation": "export_data",
        "arguments": ["$WORK/input.csv", "$WORK/sentinel.txt", "illegal"],
        "expected_error": "framework_error",
    },
    {
        "id": "guard-overlap-detect",
        "operation": "content-guard.detect",
        "arguments": ["4513812345678111"],
        "expected_error": None,
    },
    {
        "id": "guard-overlap-redact",
        "operation": "content-guard.redact",
        "arguments": ["4513812345678111"],
        "expected_error": None,
    },
    {
        "id": "media-export-valid",
        "operation": "export_text",
        "arguments": ["$WORK/input.txt", "$WORK/output.txt", "unique"],
        "expected_error": None,
    },
    {
        "id": "media-export-missing-required",
        "operation": "export_text",
        "arguments": ["$WORK/input.txt"],
        "expected_error": "framework_error",
    },
    {
        "id": "media-export-wrongoptional-type",
        "operation": "export_text",
        "arguments": ["$WORK/input.txt", "$WORK/sentinel.txt", 3],
        "expected_error": "framework_error",
    },
    {
        "id": "media-export-extra-null",
        "operation": "export_text",
        "arguments": ["$WORK/input.txt", "$WORK/sentinel.txt", "unique", None],
        "expected_error": "invalid_value",
    },
    {
        "id": "data-export-valid",
        "operation": "export_data",
        "arguments": ["$WORK/input.csv", "$WORK/output.json", "json", None, "id,value"],
        "expected_error": None,
    },
    {
        "id": "chart-wrong-width-type",
        "operation": "generate_chart",
        "arguments": [
            "bar",
            "bad",
            "label",
            "value",
            None,
            [{"label": "中文😀", "value": 1}],
            "wide",
            123,
            "json",
        ],
        "expected_error": "framework_error",
    },
    {
        "id": "chart-required-missing",
        "operation": "generate_chart",
        "arguments": ["bar", "bad", "label", "value"],
        "expected_error": "framework_error",
    },
    {
        "id": "stats-required-missing",
        "operation": "exploratory_statistics",
        "arguments": [],
        "expected_error": "framework_error",
    },
    {
        "id": "data-pivot-null-index",
        "operation": "pivot_data",
        "arguments": ["$WORK/null-index.csv", "group", "quarter", "value", "sum"],
        "expected_error": None,
    },
    {
        "id": "data-pivot-label-collision",
        "operation": "pivot_data",
        "arguments": ["$WORK/collision.csv", "group", "quarter", "value", "sum"],
        "expected_error": "framework_error",
    },
    {
        "id": "data-pivot-unknown-aggregation",
        "operation": "pivot_data",
        "arguments": ["$WORK/input.csv", "group", "quarter", "value", "unknown"],
        "expected_error": "framework_error",
    },
    {
        "id": "data-pivot-wrong-aggregation-type",
        "operation": "pivot_data",
        "arguments": ["$WORK/input.csv", "group", "quarter", "value", 3],
        "expected_error": "framework_error",
    },
    {
        "id": "data-pivot-default-aggregation",
        "operation": "pivot_data",
        "arguments": ["$WORK/input.csv", "group", "quarter", "value"],
        "expected_error": None,
    },
]
# One shared numerical boundary corpus feeds all three real public consumers.
# Optional null slots retain the accepted None/missing binder semantics.
CHART_DIMENSION_CASES = [
    ("width-overflow", 4294967296, 123, "framework_error"),
    ("height-overflow", 321, 4294967297, "framework_error"),
    ("width-negative", -1, 123, "framework_error"),
    ("height-negative", 321, -1, "framework_error"),
    ("width-u32-max", 4294967295, 123, None),
    ("height-u32-max", 321, 4294967295, None),
    ("zero", 0, 0, None),
    ("null", None, None, None),
    ("wrong-width-type", "wide", 123, "framework_error"),
    ("wrong-height-type", 321, "tall", "framework_error"),
    ("omitted-dimensions", None, None, None),
]
for identifier, width, height, expected_error in CHART_DIMENSION_CASES:
    arguments = [
        "bar",
        "中😀边界",
        "label",
        "value",
        None,
        [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
    ]
    if identifier != "omitted-dimensions":
        arguments.extend([width, height, "json"])
    ROWS_TEMPLATE.append(
        {
            "id": "chart-dimension-" + identifier,
            "operation": "generate_chart",
            "arguments": arguments,
            "expected_error": expected_error,
            "native_dimension_indices": [6, 7],
        }
    )

FILES = {
    "stats.csv": "value,label\n1,一\n2,二\n3,三\n4,四\n",
    "input.csv": "id,group,quarter,value\n1,a,q1,1\n2,a,q2,2\n3,b,q1,3\n4,b,q2,4\n",
    "lookup.csv": "id,label\n1,中文\n3,emoji😀\n",
    "quality.csv": "id,value,label\n1,10,中文\n2,,emoji😀\n3,-1,alpha\n4,100,beta\n",
    "input.txt": "中文😀\nalpha\n中文😀\n",
    "sentinel.txt": "unchanged\n",
    "null-index.csv": "group,quarter,value\n,q1,2\n,q2,3\na,q1,4\n",
    "collision.csv": "group,quarter,value\na,null,1\na,,2\n",
}


def prepare(work):
    work.mkdir(parents=True, exist_ok=True)
    for name, value in FILES.items():
        (work / name).write_text(value, encoding="utf-8")
    config = json.loads((SDK / "echo-sdk-host/config.sdk.example.json").read_text())
    config["sdk_profile"]["state_root"] = str(work / "state")
    config["default_agent"]["agent"]["memory_path"] = str(work / "memory.jsonl")
    (work / "host.json").write_text(json.dumps(config), encoding="utf-8")

    def rebase(value):
        if isinstance(value, str):
            return (
                "a" * 262145
                if value == "$OVERSIZE"
                else value.replace("$WORK", str(work))
            )
        if isinstance(value, list):
            return [rebase(item) for item in value]
        if isinstance(value, dict):
            return {key: rebase(item) for key, item in value.items()}
        return value

    (work / "scenario.json").write_text(
        json.dumps({"rows": rebase(ROWS_TEMPLATE)}, ensure_ascii=False),
        encoding="utf-8",
    )


def snapshot(work):
    return {
        name: hashlib.sha256((work / name).read_bytes()).hexdigest() for name in FILES
    }


def wire(value):
    kind = value["kind"]
    if kind == "null":
        return None
    if kind in {"string", "bool", "f64"}:
        return value["value"]
    if kind in {"u64", "i64"}:
        return int(value["value"])
    if kind == "list":
        return [wire(item) for item in value["value"]]
    if kind == "map":
        return {wire(item["key"]): wire(item["value"]) for item in value["value"]}
    if kind == "variant":
        return value
    raise AssertionError(kind)


def data(identifier):
    result = ROWS[identifier]["result"]
    output = json.loads(result["output"])
    assert result["success"] is True
    assert wire(result["data"]) == output, (
        identifier + " ToolResult output/data mismatch"
    )
    return output


class ExactGreenObservations(unittest.TestCase):
    def test_statistics_known_sample_and_no_inference(self):
        value = data("stats-exact")
        column = value["columns"][0]
        self.assertFalse(value["inference"])
        self.assertEqual(value["analysis_kind"], "exploratory_descriptive")
        self.assertEqual(
            (column["mean"], column["p25"], column["median"], column["p75"]),
            (2.5, 1.75, 2.5, 3.25),
        )
        self.assertTrue(
            math.isclose(
                column["sample_std_dev"], math.sqrt(5 / 3), rel_tol=0.0, abs_tol=1e-12
            )
        )
        self.assertEqual(
            (
                column["total_count"],
                column["valid_count"],
                column["missing_or_non_finite_count"],
            ),
            (4, 4, 0),
        )
        self.assertEqual(
            data("stats-nonnumeric")["columns"],
            [{"name": "label", "error": "Non-numeric column"}],
        )

    def test_structured_errors_paths_missing_null_and_limit(self):
        self.assertEqual(
            ROWS["structured-valid"]["result"], {"valid": True, "errors": []}
        )
        invalid = ROWS["structured-invalid"]["result"]
        self.assertFalse(invalid["valid"])
        self.assertEqual(
            {error["instance_path"] for error in invalid["errors"]}, {"/count", "/name"}
        )
        self.assertEqual(
            ROWS["structured-no-sample"]["result"], {"valid": True, "errors": []}
        )
        self.assertFalse(ROWS["structured-null-sample"]["result"]["valid"])
        self.assertEqual(
            ROWS["structured-null-sample"]["result"]["errors"][0]["instance_path"], ""
        )
        self.assertEqual(len(ROWS["structured-error-bound"]["result"]["errors"]), 16)
        self.assertEqual(
            ROWS["structured-bad-schema"]["error"]["code"], "invalid_value"
        )
        self.assertEqual(
            ROWS["structured-too-large"]["error"]["code"], "payload_too_large"
        )

    def test_guard_exact_unicode_bytes_and_modes(self):
        match = ROWS["guard-utf8-detect"]["result"]["matches"][0]
        self.assertEqual(
            (int(match["start"]), int(match["end"]), match["matched"]),
            (8, 25, "alice@example.com"),
        )
        self.assertEqual(
            ROWS["guard-utf8-redact"]["result"],
            {"redacted": "中😀 [REDACTED:EMAIL] 尾"},
        )
        for mode, variant in [
            ("detect", "detected"),
            ("reject", "rejected"),
            ("redact", "redacted"),
        ]:
            self.assertEqual(
                ROWS["guard-" + mode]["result"]["value"]["variant"], variant
            )

    def test_chart_all_seven_mark_mappings(self):
        for kind, mark in [
            ("line", "line"),
            ("bar", "bar"),
            ("pie", "arc"),
            ("scatter", "point"),
            ("area", "area"),
            ("heatmap", "rect"),
            ("boxplot", "boxplot"),
        ]:
            spec = json.loads(ROWS["chart-" + kind]["result"]["output"])
            self.assertEqual(spec["mark"], mark)
            self.assertEqual(
                (spec["width"], spec["height"], spec["title"]), (321, 123, "中😀标题")
            )
            self.assertEqual(
                spec["data"]["values"],
                [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
            )

    def test_data_aggregate_join_and_quality_exact(self):
        aggregate = data("data-aggregate")
        self.assertEqual(aggregate["rows"], 2)
        self.assertEqual(aggregate["columns"], 4)
        self.assertFalse(aggregate["truncated"])
        self.assertEqual(
            sorted(aggregate["data"], key=lambda row: row["group"]),
            [
                {"group": "a", "value_sum": 3, "value_mean": 1.5, "value_count": 2},
                {"group": "b", "value_sum": 7, "value_mean": 3.5, "value_count": 2},
            ],
        )
        joined = data("data-join")
        self.assertEqual(joined["total_rows"], 4)
        self.assertEqual(
            sorted(joined["preview"], key=lambda row: row["id"]),
            [
                {"id": 1, "group": "a", "quarter": "q1", "value": 1, "label": "中文"},
                {"id": 2, "group": "a", "quarter": "q2", "value": 2, "label": None},
                {
                    "id": 3,
                    "group": "b",
                    "quarter": "q1",
                    "value": 3,
                    "label": "emoji😀",
                },
                {"id": 4, "group": "b", "quarter": "q2", "value": 4, "label": None},
            ],
        )
        missing = data("data-missing")
        self.assertEqual(missing["total_missing_cells"], 1)
        self.assertEqual(missing["overall_missing_pct"], 8.33)
        column = next(row for row in missing["per_column"] if row["name"] == "value")
        self.assertEqual(
            (
                column["total"],
                column["non_null"],
                column["missing_count"],
                column["missing_pct"],
            ),
            (4, 3, 1, 25.0),
        )
        consistency = data("data-consistency")
        self.assertEqual(consistency["total_issues"], 1)
        self.assertEqual(consistency["issues"][0]["detail"], "2 values outside range")
        self.assertEqual(consistency["issues"][0]["type"], "range_violation")

    def test_error_fidelity_and_unchanged_failure_sentinel(self):
        for row in EVIDENCE["rows"]:
            if "error" in row:
                self.assertEqual(row["error"]["retryable"], "never")
                self.assertTrue(row["error"]["operation"])
                self.assertTrue(row["error"]["message"])
        for name in [
            "stats-unknowncolumn",
            "chart-unknown",
            "chart-wrongformat",
            "data-join-keyfail",
            "data-pivot-colfail",
            "data-export-preserve-sentinel",
        ]:
            error = ROWS[name]["error"]
            self.assertEqual(error["code"], "framework_error")
            self.assertIn(
                {"key": "framework_terminal_kind", "value": "failed"},
                error["details"]["fields"],
            )
        self.assertEqual(
            ROWS["chart-trailingnullarity"]["error"]["code"], "invalid_value"
        )
        self.assertEqual(
            EVIDENCE["actual_side_effects"]["sentinel_contents"], "unchanged\n"
        )
        self.assertEqual(
            EVIDENCE["actual_side_effects"]["sentinel_sha256"],
            hashlib.sha256(b"unchanged\n").hexdigest(),
        )

    def test_generic_validation_preserves_outputs_before_failed_effects(self):
        self.assertEqual(EVIDENCE["actual_exports"]["output_text"], "中文😀\nalpha")
        self.assertEqual(
            EVIDENCE["actual_exports"]["output_json"],
            [
                {"id": 1, "value": 1},
                {"id": 2, "value": 2},
                {"id": 3, "value": 3},
                {"id": 4, "value": 4},
            ],
        )
        for name in [
            "media-export-missing-required",
            "media-export-wrongoptional-type",
            "chart-wrong-width-type",
            "chart-required-missing",
            "stats-required-missing",
        ]:
            error = ROWS[name]["error"]
            self.assertEqual(error["code"], "framework_error")
            self.assertEqual(error["retryable"], "never")
            self.assertIn(
                {"key": "framework_failure_category", "value": "tool"},
                error["details"]["fields"],
            )
        self.assertEqual(
            ROWS["media-export-extra-null"]["error"]["code"], "invalid_value"
        )
        self.assertEqual(
            EVIDENCE["actual_side_effects"]["sentinel_sha256"],
            hashlib.sha256(b"unchanged\n").hexdigest(),
        )


class ChartDimensionBoundaries(unittest.TestCase):
    def assert_original_dimension_wire(self, identifier):
        row = ROWS["chart-dimension-" + identifier]
        self.assertEqual(row["operation"], "generate_chart")
        wire_arguments = row["arguments_wire"]
        self.assertEqual(len(wire_arguments), len(row["arguments"]))
        for index in row["native_dimension_indices"]:
            if index >= len(row["arguments"]):
                continue
            value = row["arguments"][index]
            if value is None:
                expected = {"kind": "null"}
            elif isinstance(value, int):
                expected = {
                    "kind": "i64" if value < 0 else "u64",
                    "value": str(value),
                }
            else:
                expected = {"kind": "string", "value": value}
            self.assertEqual(wire_arguments[index], expected)

    def test_supplied_invalid_dimensions_preserve_typed_failure_and_exact_wire(self):
        messages = {
            "width-overflow": "4294967296 is greater than the maximum of 4294967295",
            "height-overflow": "4294967297 is greater than the maximum of 4294967295",
            "width-negative": "-1 is less than the minimum of 0",
            "height-negative": "-1 is less than the minimum of 0",
            "wrong-width-type": '"wide" is not of type "integer"',
            "wrong-height-type": '"tall" is not of type "integer"',
        }
        for identifier, message in messages.items():
            with self.subTest(identifier=identifier):
                self.assert_original_dimension_wire(identifier)
                row = ROWS["chart-dimension-" + identifier]
                self.assertNotIn("result", row, "invalid dimension returned a chart")
                self.assertIn("error", row)
                error = row["error"]
                self.assertEqual(error["code"], "framework_error")
                self.assertEqual(error["operation"], "generate_chart")
                self.assertEqual(error["retryable"], "never")
                self.assertIsNone(error["handle"])
                self.assertEqual(
                    error["message"],
                    "Tool Error: Invalid parameter 'generate_chart': " + message,
                )
                self.assertEqual(
                    error["details"],
                    {
                        "fields": [
                            {"key": "framework_failure_category", "value": "tool"},
                            {"key": "framework_terminal_kind", "value": "failed"},
                            {"key": "framework_failure_code", "value": "tool"},
                            {"key": "framework_retryable", "value": "false"},
                        ]
                    },
                )

    def test_valid_dimensions_defaults_and_unicode_preserve_original_data(self):
        controls = {
            "width-u32-max": (4294967295, 123),
            "height-u32-max": (321, 4294967295),
            "zero": (0, 0),
            "null": (600, 400),
            "omitted-dimensions": (600, 400),
        }
        for identifier, dimensions in controls.items():
            with self.subTest(identifier=identifier):
                self.assert_original_dimension_wire(identifier)
                result = ROWS["chart-dimension-" + identifier]["result"]
                self.assertIs(result["success"], True)
                spec = json.loads(result["output"])
                self.assertEqual((spec["width"], spec["height"]), dimensions)
                self.assertEqual(spec["title"], "中😀边界")
                self.assertEqual(spec["mark"], "bar")
                self.assertEqual(
                    spec["data"]["values"],
                    [{"label": "中文😀", "value": 1}, {"label": "尾", "value": 4}],
                )


class OriginalSemanticCounterexamples(unittest.TestCase):
    def test_overlap_should_redact_complete_card_once(self):
        self.assertEqual(
            ROWS["guard-overlap-redact"]["result"], {"redacted": "[REDACTED:CARD]"}
        )

    def test_pivot_should_preserve_every_input_cell(self):
        value = data("data-pivot")["pivot_result"]
        self.assertEqual(set(value["pivot_values"]), {"q1", "q2"})
        self.assertEqual(
            sorted(value["data"], key=lambda row: row["group"]),
            [{"group": "a", "q1": 1, "q2": 2}, {"group": "b", "q1": 3, "q2": 4}],
        )


def verify(work, evidence):
    global EVIDENCE, ROWS
    EVIDENCE = evidence
    ROWS = {row["id"]: row for row in evidence["rows"]}
    assert len(ROWS) == len(ROWS_TEMPLATE) == len(evidence["rows"])
    assert evidence["host_sha256"] == file_sha256(evidence["host"])
    suite = unittest.TestSuite(
        [
            unittest.defaultTestLoader.loadTestsFromTestCase(ExactGreenObservations),
            unittest.defaultTestLoader.loadTestsFromTestCase(ChartDimensionBoundaries),
            unittest.defaultTestLoader.loadTestsFromTestCase(
                OriginalSemanticCounterexamples
            ),
        ]
    )
    stream = io.StringIO()
    result = unittest.TextTestRunner(stream=stream).run(suite)
    assert result.wasSuccessful(), stream.getvalue()
    for row in evidence["rows"]:
        outcome = row.get("result")
        if isinstance(outcome, dict) and "data" in outcome:
            assert wire(outcome["data"]) == json.loads(outcome["output"]), (
                row["id"] + " raw data/output mismatch"
            )
        assert row["protected_before"] == row["protected_after"] == snapshot(work), (
            row["id"] + " changed protected input"
        )
        if row["expected_error"] is None:
            assert "error" not in row, row["id"]
        else:
            error = row["error"]
            assert error["code"] == row["expected_error"]
            expected_operation = (
                "_echo_agent/structured_output/validate"
                if row["operation"] == "structured_output.validate"
                else row["operation"]
            )
            assert error["operation"] == expected_operation
            assert error["handle"] is None
            assert (
                error["message"]
                and error["operation"]
                and error["retryable"] == "never"
            )
            if error["code"] == "framework_error":
                fields = {
                    entry["key"]: entry["value"] for entry in error["details"]["fields"]
                }
                assert fields["framework_terminal_kind"] == "failed"
                assert fields["framework_failure_category"] == "tool"
                assert fields["framework_failure_code"] == "tool"
                assert fields["framework_retryable"] == "false"
                assert (
                    error["handle"] is None
                )  # The accepted framework envelope has no attached handle.
    null_rows = data("data-pivot-null-index")["pivot_result"]["data"]
    assert {
        key: value
        for key, value in next(row for row in null_rows if row["group"] is None).items()
    } == {"group": None, "q1": 2, "q2": 3}
    assert "null" in ROWS["data-pivot-label-collision"]["error"]["message"]
    assert "not one of" in ROWS["data-pivot-unknown-aggregation"]["error"]["message"]
    default = data("data-pivot-default-aggregation")["pivot_result"]
    assert default["agg_function"] == "sum"
    assert sorted(default["data"], key=lambda row: row["group"]) == [
        {"group": "a", "q1": 1, "q2": 2},
        {"group": "b", "q1": 3, "q2": 4},
    ]
    return {
        "native_calls": len(ROWS),
        "typed_errors": sum("error" in row for row in evidence["rows"]),
        "oracle_groups": result.testsRun,
        "final_pivot_boundaries": 5,
        "protected_files": len(FILES),
        "host_sha256": evidence["host_sha256"],
    }


def compare_evidence(paths):
    """Compare native outcomes after only documented nonsemantic order/path normalization."""

    def canonical(value, work):
        if isinstance(value, str):
            return value.replace(work, "$WORK")
        if isinstance(value, list):
            return [canonical(item, work) for item in value]
        if not isinstance(value, dict):
            return value
        result = {key: canonical(item, work) for key, item in value.items()}
        if value.get("kind") == "map":
            result["value"].sort(
                key=lambda entry: json.dumps(entry["key"], sort_keys=True)
            )
        return result

    normalized = []
    summaries = []
    for path in paths:
        evidence = json.loads(path.read_text())
        summaries.append(verify(Path(evidence["work"]), evidence))
        rows = []
        for original in evidence["rows"]:
            row = canonical(
                {
                    key: value
                    for key, value in original.items()
                    if key not in {"protected_before", "protected_after"}
                },
                evidence["work"],
            )
            value = row.get("result")
            if isinstance(value, dict) and "output" in value:
                try:
                    output = json.loads(value["output"])
                except json.JSONDecodeError:
                    output = None
                if output is not None:
                    operation = row["operation"]
                    if operation == "aggregate_data":
                        output["data"].sort(key=lambda item: item["group"])
                    if operation == "join_data":
                        output["preview"].sort(key=lambda item: item["id"])
                    if operation == "pivot_data":
                        output["pivot_result"]["data"].sort(
                            key=lambda item: json.dumps(item["group"])
                        )
                        output["pivot_result"]["pivot_values"].sort()
                    value["parsed_output"] = output
                    value.pop("output")
                    # Every raw data value was independently proven equal to parsed output.
                    value.pop("data", None)
            rows.append(row)
        normalized.append(rows)
    for other in normalized[1:]:
        for expected, actual in zip(normalized[0], other):
            assert expected == actual, (expected["id"], expected, actual)
    return {
        "native_comparison": "equal",
        "consumers": summaries,
        "normalization": [
            "isolated working root",
            "WireValue map association order",
            "unordered Polars aggregate/join/pivot row and pivot-label order",
        ],
        "raw_data": "retained; checked against parsed output before ordering normalization",
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--work", type=Path)
    parser.add_argument("--compare", type=Path, nargs=3)
    parser.add_argument("--verify", type=Path)
    args = parser.parse_args()
    if args.compare:
        print(json.dumps(compare_evidence(args.compare)))
    elif args.verify:
        print(json.dumps(verify(args.work, json.loads(args.verify.read_text()))))
    else:
        prepare(args.work)
