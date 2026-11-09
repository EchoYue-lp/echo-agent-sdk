"""分发合同随语言包携带，不依赖仓库外的 shared 相邻目录。"""

import json
import subprocess
import sys
import tarfile
import zipfile
from pathlib import Path

from echo_agent_sdk import FacadeCatalog


def test_package_catalog_and_digests_match_the_only_producer():
    root = Path(__file__).resolve().parents[3]
    packaged = root / "sdks/python/src/echo_agent_sdk/contracts"
    for name in ("facade-operation-catalog.json", "contract-digests.json"):
        assert (packaged / name).read_bytes() == (
            root / "sdks/shared" / name
        ).read_bytes()
    catalog = FacadeCatalog()
    assert catalog.resolve("memory.store.put").method == "_echo_agent/memory/op"
    assert catalog.operations()


def test_wheel_and_sdist_load_without_the_repository_shared_directory(tmp_path):
    project = Path(__file__).resolve().parents[1]
    result = subprocess.run(
        ["uv", "build", "--wheel", "--sdist", "--out-dir", str(tmp_path / "dist")],
        cwd=project,
        capture_output=True,
        text=True,
        check=False,
    )
    (tmp_path / "build.log").write_text(result.stdout + result.stderr, encoding="utf-8")
    assert result.returncode == 0, result.stdout + result.stderr
    (wheel,) = (tmp_path / "dist").glob("*.whl")
    (sdist,) = (tmp_path / "dist").glob("*.tar.gz")
    with zipfile.ZipFile(wheel) as archive, tarfile.open(sdist) as source:
        source_members = {member.name: member for member in source.getmembers()}
        for name in ("facade-operation-catalog.json", "contract-digests.json"):
            expected = (project.parent / "shared" / name).read_bytes()
            assert archive.read("echo_agent_sdk/contracts/" + name) == expected
            matches = [
                member
                for path, member in source_members.items()
                if path.endswith("/src/echo_agent_sdk/contracts/" + name)
            ]
            assert len(matches) == 1
            stream = source.extractfile(matches[0])
            assert stream is not None
            with stream:
                assert stream.read() == expected
        isolated = tmp_path / "installed-wheel"
        archive.extractall(isolated)
    # 新进程从解包后的wheel导入，不能通过editable安装回退原仓库代码。
    program = """
import json, pathlib, sys
sys.path.insert(0, sys.argv[1])
import echo_agent_sdk
from echo_agent_sdk import FacadeCatalog
assert pathlib.Path(echo_agent_sdk.__file__).is_relative_to(pathlib.Path(sys.argv[1]))
catalog = FacadeCatalog()
assert catalog.resolve('memory.store.put').method == '_echo_agent/memory/op'
print(json.dumps({'contract_digest':catalog.contract_digest,'source_contract_digest':catalog.source_contract_digest}))
"""
    probe = subprocess.run(
        [sys.executable, "-I", "-c", program, str(isolated)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        check=False,
    )
    (tmp_path / "load.log").write_text(probe.stdout + probe.stderr, encoding="utf-8")
    assert probe.returncode == 0, probe.stdout + probe.stderr
    assert (
        json.loads(probe.stdout)["contract_digest"] == FacadeCatalog().contract_digest
    )
