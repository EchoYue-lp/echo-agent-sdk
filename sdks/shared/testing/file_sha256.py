"""固定分块计算测试证据摘要，不把大型 Host 二进制整体载入内存。"""

import hashlib
from pathlib import Path


def file_sha256(path: Path | str) -> str:
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        while chunk := stream.read(1024 * 1024):
            digest.update(chunk)
    return digest.hexdigest()
