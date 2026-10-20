"""摘要 helper 的真实文件、跨分块、超大稀疏输入及 IO 错误反例。"""

import hashlib

import pytest
from file_sha256 import file_sha256


@pytest.mark.parametrize(
    "data", [b"", "中文😀\n".encode(), b"a" * (1024 * 1024) + "中文🙂".encode()]
)
def test_file_sha256_exact_bytes(tmp_path, data):
    path = tmp_path / "input.bin"
    path.write_bytes(data)
    assert file_sha256(path) == hashlib.sha256(data).hexdigest()


def test_file_sha256_sparse_above_whole_file_limit(tmp_path):
    path = tmp_path / "large.bin"
    size = 2**31 + 1
    with path.open("wb") as stream:
        stream.truncate(size)
        stream.seek(0)
        stream.write("start 中文😀\n".encode())
        stream.seek(size - 3)
        stream.write(b"END")
    assert (
        file_sha256(path)
        == "05aa6dd2c62008228e2cb1ab20553b85be8565f3ddf82f0527661905c4015750"
    )


def test_file_sha256_missing_path_error(tmp_path):
    with pytest.raises(FileNotFoundError):
        file_sha256(tmp_path / "missing")


def test_file_sha256_directory_error(tmp_path):
    with pytest.raises(IsADirectoryError):
        file_sha256(tmp_path)
