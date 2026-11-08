"""用本地命令替身检查正式脚本的准入顺序；不运行 Cargo、生成器或 Git 写操作。"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest


ROOT = Path(__file__).resolve().parents[2]
REVISION = "340993201cc937fc8e4783b66be399ef9f14e3d5"
SOURCE = f"git+https://github.com/EchoYue-lp/echo-agent.git?rev={REVISION}#{REVISION}"
PACKAGES = [
    ("calamine", "0.36.1"), ("docx-rs", "0.4.22"), ("echo_agent", "0.2.0"),
    ("echo_core", "0.2.0"), ("echo_execution", "0.2.0"), ("echo_integration", "0.2.0"),
    ("echo_macros", "0.2.0"), ("echo_orchestration", "0.2.0"), ("echo_state", "0.2.0"),
    ("echo_tools", "0.2.0"), ("polars-arrow", "0.55.2"), ("polars-parquet", "0.55.2"),
    ("rust_xlsxwriter", "0.82.0"),
]


class SourcePreflightTests(unittest.TestCase):
    def setUp(self):
        # 回执和临时输入仍留在本仓主根；不用系统外部 worktree 或真实 Git fixture。
        local = ROOT / ".echo-semantic/local/contract-preflight-tests"
        local.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(dir=local)
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.checkout = self.base / "framework"
        self.checkout.mkdir()
        self.scripts = self.base / "scripts"
        self.scripts.mkdir()
        self.bin = self.base / "bin"
        self.bin.mkdir()
        shutil.copyfile(ROOT / "scripts/check-sdk-contracts.sh", self.scripts / "check-sdk-contracts.sh")
        self.trace = self.base / "trace.jsonl"
        self.metadata = {"packages": []}
        for name, version in PACKAGES:
            directory = self.checkout / name
            directory.mkdir()
            manifest = directory / "Cargo.toml"
            manifest.write_text("# 本地静态夹具\n")
            self.metadata["packages"].append({"name": name, "version": version,
                "source": SOURCE, "manifest_path": str(manifest)})
        stub = textwrap.dedent("""\
            import json, os, sys
            from pathlib import Path
            name = Path(sys.argv[0]).name
            args = sys.argv[1:]
            with open(os.environ['PREFLIGHT_TRACE'], 'a') as output:
                output.write(json.dumps([name, *args]) + '\\n')
            if name == 'rustup':
                print('/fixture/rustdoc')
            elif name == 'python3':
                if args and args[0] == '-c': print('fixture-toolchain')
            elif name == 'cargo':
                if 'metadata' in args: print(Path(os.environ['PREFLIGHT_METADATA']).read_text())
                elif 'tree' in args: print('echo-sdk-protocol v0.2.0')
            elif name == 'git':
                if 'HEAD' in args and 'rev-parse' in args:
                    print(os.environ['PREFLIGHT_HEAD'])
                elif 'diff' in args:
                    sys.exit(int(os.environ.get('PREFLIGHT_DIRTY', '0')))
                elif '--show-toplevel' in args:
                    directory = args[args.index('-C') + 1]
                    print('/foreign/checkout' if os.environ.get('PREFLIGHT_FOREIGN') in directory
                          and os.environ.get('PREFLIGHT_FOREIGN') else os.environ['PREFLIGHT_CHECKOUT'])
                else:
                    sys.exit(90)
            else:
                sys.exit(91)
            """)
        for name in ["cargo", "git", "rustup", "python3"]:
            path = self.bin / name
            path.write_text(f"#!{sys.executable}\n" + stub)
            path.chmod(0o755)
        export = self.scripts / "export-language-sdk-catalog.sh"
        export.write_text("#!/bin/sh\nexit 0\n")
        export.chmod(0o755)
        self.env = {**os.environ, "PATH": str(self.bin) + os.pathsep + os.environ.get("PATH", ""),
            "PREFLIGHT_TRACE": str(self.trace), "PREFLIGHT_METADATA": str(self.base / "metadata.json"),
            "PREFLIGHT_CHECKOUT": str(self.checkout), "PREFLIGHT_HEAD": REVISION,
            "PREFLIGHT_DIRTY": "0", "PREFLIGHT_FOREIGN": ""}

    def run_script(self):
        Path(self.env["PREFLIGHT_METADATA"]).write_text(json.dumps(self.metadata))
        result = subprocess.run(["bash", str(self.scripts / "check-sdk-contracts.sh")],
                                env=self.env, text=True, capture_output=True)
        calls = [json.loads(line) for line in self.trace.read_text().splitlines()]
        generations = [call for call in calls if call[0] == "cargo" and "run" in call]
        return result, calls, generations

    def assert_rejected_before_generation(self):
        result, _, generations = self.run_script()
        self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(generations, [], "不可信 checkout 不得先参与生成")

    def test_valid_source_is_checked_before_generation_and_keeps_native_contract(self):
        result, calls, generations = self.run_script()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(len(generations), 1)
        generation = calls.index(generations[0])
        self.assertTrue(any(call[0] == "git" and "diff" in call for call in calls[:generation]))
        manifests = [call for call in calls[:generation] if call[0] == "git" and "--show-toplevel" in call]
        self.assertEqual(len(manifests), 14)  # 原 echo_agent root ＋全部十三 manifest。
        self.assertTrue(any("native_message_contract" in call for call in calls if call[0] == "cargo"))

    def test_wrong_checkout_head_is_rejected(self):
        self.env["PREFLIGHT_HEAD"] = "0" * 40
        self.assert_rejected_before_generation()

    def test_tracked_checkout_change_is_rejected(self):
        self.env["PREFLIGHT_DIRTY"] = "1"
        self.assert_rejected_before_generation()

    def test_vendor_manifest_in_another_checkout_is_rejected(self):
        self.env["PREFLIGHT_FOREIGN"] = "polars-parquet"
        self.assert_rejected_before_generation()

    def test_missing_vendor_manifest_is_rejected(self):
        Path(self.metadata["packages"][0]["manifest_path"]).unlink()
        self.assert_rejected_before_generation()

    def test_wrong_source_or_package_set_is_rejected(self):
        for change in ["source", "version", "duplicate", "missing", "extra", "manifest"]:
            with self.subTest(change=change):
                original = json.loads(json.dumps(self.metadata))
                if self.trace.exists(): self.trace.unlink()
                if change == "source": self.metadata["packages"][0]["source"] = "path+override"
                if change == "version": self.metadata["packages"][0]["version"] = "999.0.0"
                if change == "duplicate": self.metadata["packages"].append(self.metadata["packages"][0])
                if change == "missing": self.metadata["packages"].pop()
                if change == "extra": self.metadata["packages"].append({"name": "unknown", "version": "1", "source": SOURCE})
                if change == "manifest": self.metadata["packages"][0]["manifest_path"] = None
                self.assert_rejected_before_generation()
                self.metadata = original


if __name__ == "__main__":
    unittest.main()
