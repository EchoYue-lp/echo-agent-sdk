import test from "node:test";
import assert from "node:assert/strict";
import { PromptCacheLayout, toWireValue } from "../dist/index.js";

const message = (role, text) => ({ role, content: toWireValue(text) });
const tail = (body) => "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\n" + body;

test("PromptCacheLayout projects Rust cache segments and ranges", () => {
  const layout = PromptCacheLayout.fromMessages([
    message("system", "You are Echo Agent"),
    message("system", "[Canonical context - restored]"),
    message("user", "hello"),
    message("assistant", "hi"),
    message("user", tail("turn 1")),
    message("user", tail("hook")),
  ], [{ name: "lookup", description: "", parameters: toWireValue({}), tool_type: "function" }]);

  assert.equal(layout.system.length, 1);
  assert.equal(layout.canonical.length, 1);
  assert.equal(layout.history.length, 2);
  assert.equal(layout.runtimeContext.length, 2);
  assert.equal(layout.tools.length, 1);
  const ranges = layout.segmentRanges();
  assert.deepEqual([
    [ranges.system.start, ranges.system.end],
    [ranges.canonical.start, ranges.canonical.end],
    [ranges.history.start, ranges.history.end],
    [ranges.runtimeContext.start, ranges.runtimeContext.end],
  ], [[0n, 1n], [1n, 2n], [2n, 4n], [4n, 6n]]);
});

test("PromptCacheLayout leaves canonical empty when no marker exists", () => {
  const layout = PromptCacheLayout.fromMessages([
    message("system", "S"),
    message("user", "hello"),
  ], []);
  assert.equal(layout.canonical.length, 0);
  assert.equal(layout.history.length, 1);
  assert.equal(layout.segmentRanges().canonical.isEmpty(), true);
});

// 期望值来自上游严格信封和尾段边界，不在测试中实现分段算法。
const headerCases = [
  {
    "name": "valid_tail",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\n事实🙂",
    "dynamic": true
  },
  {
    "name": "valid_empty_body",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\n",
    "dynamic": true
  },
  {
    "name": "valid_json_whitespace",
    "text": "<echo-agent-context-projection-v2>\nmarker= \" \" \nplacement=tail\nversion= \"v\" \n\nbody",
    "dynamic": true
  },
  {
    "name": "valid_json_unicode",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"\\u4e2d\\ud83d\\ude42\"\nplacement=tail\nversion=\"\\u7248\"\n\nbody",
    "dynamic": true
  },
  {
    "name": "valid_crlf_header_lines",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\r\nplacement=tail\r\nversion=\"v1\"\n\nbody",
    "dynamic": true
  },
  {
    "name": "bare_prefix",
    "text": "[runtime_context: user text]",
    "dynamic": false
  },
  {
    "name": "bare_prefix_after_spaces",
    "text": "  [runtime_context: user text]",
    "dynamic": false
  },
  {
    "name": "legacy_v1",
    "text": "<echo-agent-context-projection-v1>\nmarker=\"runtime\"\nversion=\"body\"",
    "dynamic": false
  },
  {
    "name": "boundary",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=boundary\n\nbody",
    "dynamic": false
  },
  {
    "name": "boundary_body_version",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=boundary\n\nversion=\"body\"",
    "dynamic": false
  },
  {
    "name": "unterminated_header",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"",
    "dynamic": false
  },
  {
    "name": "missing_version",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\n\nversion=\"body\"",
    "dynamic": false
  },
  {
    "name": "extra_header",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\nextra=\"x\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "empty_marker",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"\"\nplacement=tail\nversion=\"v1\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "empty_version",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "numeric_marker",
    "text": "<echo-agent-context-projection-v2>\nmarker=1\nplacement=tail\nversion=\"v1\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "numeric_version",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=1\n\nbody",
    "dynamic": false
  },
  {
    "name": "bad_json_escape",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"\\q\"\nplacement=tail\nversion=\"v1\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "trailing_marker_json",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\" true\nplacement=tail\nversion=\"v1\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "trailing_version_json",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\" \"v2\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "unpaired_marker_surrogate",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"\\ud800\"\nplacement=tail\nversion=\"v1\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "unpaired_version_surrogate",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"\\udc00\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "leading_whitespace",
    "text": " <echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=tail\nversion=\"v1\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "wrong_header_order",
    "text": "<echo-agent-context-projection-v2>\nplacement=tail\nmarker=\"runtime\"\nversion=\"v1\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "extra_boundary_version",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=boundary\nversion=\"v1\"\n\nbody",
    "dynamic": false
  },
  {
    "name": "non_tail_placement",
    "text": "<echo-agent-context-projection-v2>\nmarker=\"runtime\"\nplacement=Tail\nversion=\"v1\"\n\nbody",
    "dynamic": false
  }
];

for (const { name, text, dynamic } of headerCases) {
  test(`PromptCacheLayout v2 header: ${name}`, () => {
    const messages = [message("system", "S"), message("user", "history"), message("user", text)];
    const tools = [{ name: "second" }, { name: "first" }];
    const before = structuredClone({ messages, tools });
    const layout = PromptCacheLayout.fromMessages(messages, tools);
    assert.deepEqual(layout.history, dynamic ? messages.slice(1, 2) : messages.slice(1));
    assert.deepEqual(layout.runtimeContext, dynamic ? messages.slice(2) : []);
    assert.deepEqual(layout.tools, tools);
    assert.equal(layout.segmentRanges().runtimeContext.start, dynamic ? 2n : 3n);
    assert.equal(layout.segmentRanges().runtimeContext.end, 3n);
    assert.deepEqual({ messages, tools }, before);
    assert.equal(Object.hasOwn(layout, "dynamic"), false);
  });
}

test("PromptCacheLayout stops at the last ordinary message or any system role", () => {
  const system = message("system", "S");
  const history = message("user", "history");
  const fact = message("user", tail("fact"));
  const lateSystem = message("system", tail("system fact"));
  for (const messages of [[system, fact, history], [system, history, fact, lateSystem]]) {
    const layout = PromptCacheLayout.fromMessages(messages, []);
    assert.deepEqual(layout.history, messages.slice(1));
    assert.deepEqual(layout.runtimeContext, []);
  }
  const messages = [system, history, fact, lateSystem, fact];
  const layout = PromptCacheLayout.fromMessages(messages, []);
  assert.deepEqual(layout.history, messages.slice(1, 4));
  assert.deepEqual(layout.runtimeContext, [fact]);
  assert.deepEqual([layout.segmentRanges().runtimeContext.start, layout.segmentRanges().runtimeContext.end], [4n, 5n]);
});

test("PromptCacheLayout accepts only the first text part as an envelope", () => {
  for (const [parts, expected] of [
    [[{ type: "text", text: tail("fact") }, { type: "text", text: "extra" }], 1],
    [[{ type: "image_url", image_url: { url: "data:image/png;base64,AA==" } }, { type: "text", text: tail("fact") }], 0],
    [[{ type: "text", text: "<echo-agent-context-projection-v2>\n" }, { type: "text", text: 'marker="runtime"\nplacement=tail\nversion="v1"\n\nbody' }], 0],
  ]) {
    const layout = PromptCacheLayout.fromMessages([message("system", "S"), { role: "user", content: toWireValue(parts) }], []);
    assert.equal(layout.runtimeContext.length, expected);
  }
});

test("PromptCacheLayout handles empty and entirely system inputs", () => {
  for (const messages of [[], [message("system", tail("fact")), message("system", "S")]]) {
    const layout = PromptCacheLayout.fromMessages(messages, []);
    assert.deepEqual(layout.system, messages);
    assert.deepEqual(layout.history, []);
    assert.deepEqual(layout.runtimeContext, []);
  }
});

const canonicalPartCases = [
  {
    "name": "single_text",
    "parts": [
      {
        "type": "text",
        "text": "Canonical context"
      }
    ],
    "canonical": true
  },
  {
    "name": "split_text_no_separator",
    "parts": [
      {
        "type": "text",
        "text": "Canonical "
      },
      {
        "type": "text",
        "text": "context"
      }
    ],
    "canonical": true
  },
  {
    "name": "split_text_no_inserted_space",
    "parts": [
      {
        "type": "text",
        "text": "Canonical"
      },
      {
        "type": "text",
        "text": "context"
      }
    ],
    "canonical": false
  },
  {
    "name": "ignored_media_between_text",
    "parts": [
      {
        "type": "text",
        "text": "Canonical"
      },
      {
        "type": "file",
        "name": "ignored",
        "content": "AA=="
      },
      {
        "type": "text",
        "text": " context"
      }
    ],
    "canonical": true
  },
  {
    "name": "media_names_are_not_text",
    "parts": [
      {
        "type": "file",
        "name": "Canonical context",
        "content": "AA=="
      },
      {
        "type": "image_url",
        "image_url": {
          "url": "Canonical context"
        }
      }
    ],
    "canonical": false
  },
  {
    "name": "empty_parts",
    "parts": [],
    "canonical": false
  },
  {
    "name": "empty_text",
    "parts": [
      {
        "type": "text",
        "text": ""
      }
    ],
    "canonical": false
  },
  {
    "name": "resource_title_missing_optionals",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "title": "Canonical context"
        }
      }
    ],
    "canonical": true
  },
  {
    "name": "resource_name",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "Canonical context",
          "uri": "resource://report"
        }
      }
    ],
    "canonical": true
  },
  {
    "name": "resource_uri",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "Canonical context"
        }
      }
    ],
    "canonical": true
  },
  {
    "name": "resource_description",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "description": "Canonical context"
        }
      }
    ],
    "canonical": true
  },
  {
    "name": "resource_mime",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "mime_type": "Canonical context"
        }
      }
    ],
    "canonical": true
  },
  {
    "name": "resource_annotations_value",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "annotations": {
            "note": "Canonical context"
          }
        }
      }
    ],
    "canonical": true
  },
  {
    "name": "resource_annotations_key",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "annotations": {
            "Canonical context": true
          }
        }
      }
    ],
    "canonical": true
  },
  {
    "name": "resource_meta",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "meta": {
            "note": "Canonical context"
          }
        }
      }
    ],
    "canonical": true
  },
  {
    "name": "resource_minimal_no_marker",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report"
        }
      }
    ],
    "canonical": false
  },
  {
    "name": "resource_unknown_field_ignored",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "extra": "Canonical context"
        }
      }
    ],
    "canonical": false
  },
  {
    "name": "resource_prevents_text_join",
    "parts": [
      {
        "type": "text",
        "text": "Canonical"
      },
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report"
        }
      },
      {
        "type": "text",
        "text": " context"
      }
    ],
    "canonical": false
  },
  {
    "name": "resource_fields_do_not_join",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "Canonical",
          "uri": " context"
        }
      }
    ],
    "canonical": false
  },
  {
    "name": "resource_json_newline_not_space",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "title": "Canonical\ncontext"
        }
      }
    ],
    "canonical": false
  },
  {
    "name": "resource_empty_optionals",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report",
          "description": "",
          "mime_type": "",
          "title": "",
          "annotations": null,
          "meta": null,
          "size": null
        }
      }
    ],
    "canonical": false
  },
  {
    "name": "text_after_resource",
    "parts": [
      {
        "type": "resource_link",
        "resource": {
          "name": "report",
          "uri": "resource://report"
        }
      },
      {
        "type": "text",
        "text": "Canonical context"
      }
    ],
    "canonical": true
  }
];

for (const { name, parts, canonical } of canonicalPartCases) {
  test(`PromptCacheLayout canonical text view: ${name}`, () => {
    const system = { role: "system", content: toWireValue(parts) };
    const before = structuredClone(system);
    const layout = PromptCacheLayout.fromMessages([system], []);
    assert.deepEqual(layout.system, canonical ? [] : [system]);
    assert.deepEqual(layout.canonical, canonical ? [system] : []);
    assert.deepEqual(layout.history, []);
    assert.deepEqual(layout.runtimeContext, []);
    assert.equal(layout.segmentRanges().system.end, canonical ? 0n : 1n);
    assert.equal(layout.segmentRanges().canonical.end, 1n);
    assert.deepEqual(system, before);
  });
}

test("PromptCacheLayout multipart canonical keeps leading-system boundary and dynamic first-part rule", () => {
  const canonical = { role: "system", content: toWireValue([{ type: "text", text: "Canonical context" }]) };
  const system = message("system", "S");
  const history = message("user", "history");
  const fact = message("user", tail("fact"));
  const layout = PromptCacheLayout.fromMessages([system, canonical, system, history, fact], []);
  assert.deepEqual(layout.system, [system]);
  assert.deepEqual(layout.canonical, [canonical, system]);
  assert.deepEqual(layout.history, [history]);
  assert.deepEqual(layout.runtimeContext, [fact]);
  const late = PromptCacheLayout.fromMessages([system, history, canonical], []);
  assert.deepEqual(late.canonical, []);
  assert.deepEqual(late.history, [history, canonical]);
  assert.deepEqual(late.runtimeContext, []);
  const resourceThenText = { role: "user", content: toWireValue([
    { type: "resource_link", resource: { name: "report", uri: "resource://report", title: tail("fact") } },
    { type: "text", text: tail("fact") },
  ]) };
  assert.deepEqual(PromptCacheLayout.fromMessages([resourceThenText], []).runtimeContext, []);
});
