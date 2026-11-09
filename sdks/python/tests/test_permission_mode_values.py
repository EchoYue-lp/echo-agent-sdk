import json
from pathlib import Path

import pytest

from echo_agent_sdk import PermissionMode

MODES = [
    ("default", False, True, False),
    ("plan", False, True, False),
    ("auto-edit", True, False, False),
    ("full-auto", True, False, False),
    ("auto", False, False, True),
    ("bubble", False, True, False),
    ("dont-ask", False, False, False),
    ("strict", False, True, False),
]
RETIRED_ALIASES = [
    "ask",
    "accept-edits",
    "autoedit",
    "acceptedits",
    "fullauto",
    "bypass",
    "bypass-permissions",
    "bypasspermissions",
    "dontask",
    "strict-confirm",
    "strict-confirmation",
]
REJECTED = (
    RETIRED_ALIASES
    + [
        variant
        for mode, *_ in MODES
        for variant in [mode.upper(), " " + mode, mode + " ", "\t" + mode + "\n"]
    ]
    + ["unknown", "", "__proto__", "toString"]
)


@pytest.mark.parametrize("identifier,write,interaction,classifier", MODES)
def test_exact_canonical_wire_and_policy(identifier, write, interaction, classifier):
    mode = PermissionMode.parse(identifier)
    assert str(mode) == identifier
    assert json.loads(json.dumps(mode)) == identifier
    assert mode.allows_write() is write
    assert mode.requires_interaction() is interaction
    assert mode.uses_classifier() is classifier


@pytest.mark.parametrize("value", REJECTED)
def test_rejects_noncanonical(value):
    with pytest.raises(ValueError, match="invalid permission mode"):
        PermissionMode.parse(value)


@pytest.mark.parametrize("value", [None, 1, {}, []])
def test_rejects_nontext(value):
    with pytest.raises(TypeError, match="permission mode must be text"):
        PermissionMode.parse(value)


def test_all_eight_modes_and_default_are_unchanged():
    assert PermissionMode.DEFAULT.value == "default"
    assert {mode.value for mode in PermissionMode} == {row[0] for row in MODES}


def test_permission_mode_helper_mappings_are_complete() -> None:
    root = Path(__file__).resolve().parents[3]
    manifest = json.loads((root / "contracts/sdk/parity-manifest.json").read_text())
    entries = [
        entry
        for entry in manifest["entries"]
        if entry["canonical"]
        and entry["languages"]["python"]["contract_test"].endswith(
            "/permission_mode_values"
        )
    ]
    assert len(entries) == 6
    assert all(entry["languages"]["python"]["status"] == "done" for entry in entries)
