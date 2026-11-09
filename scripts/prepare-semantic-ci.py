#!/usr/bin/env python3
"""在精确候选源码视图上调用原预检 producer；不替代 canonical strict。"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys


ARGS_PATH = '.github/semantic-preflight-args.json'
PRODUCER_PATH = 'skills/semantic-preflight/scripts/preflight.py'


class CommandError(RuntimeError):
    def __init__(self, command: list[str], code: int):
        super().__init__(f'命令失败，原出口 {code}：{command!r}')
        self.code = code if code > 0 else 128 - code


class Commands:
    def __init__(self, root: Path, output: Path):
        self.root = root
        self.output = output
        self.records: list[dict] = []

    def run(self, command: list[str], *, check: bool = True) -> subprocess.CompletedProcess:
        result = subprocess.run(command, cwd=self.root, capture_output=True,
                                env={**os.environ, 'GIT_OPTIONAL_LOCKS': '0'})
        prefix = self.output / f'{len(self.records):03d}'
        prefix.with_suffix('.stdout').write_bytes(result.stdout)
        prefix.with_suffix('.stderr').write_bytes(result.stderr)
        self.records.append({'argv': command, 'cwd': str(self.root),
                             'returnCode': result.returncode,
                             'stdout': str(prefix.with_suffix('.stdout')),
                             'stderr': str(prefix.with_suffix('.stderr'))})
        (self.output / 'commands.json').write_text(
            json.dumps(self.records, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
        if check and result.returncode:
            raise CommandError(command, result.returncode)
        return result

    def git(self, *args: str, check: bool = True) -> subprocess.CompletedProcess:
        return self.run(['git', '-c', 'core.quotePath=false', *args], check=check)

    def text(self, *args: str) -> str:
        return self.git(*args).stdout.decode('utf-8').strip()

    def clean(self) -> None:
        # 不排除业务路径；工具 checkout 只能由 workflow 提前放入精确 runtime 排除范围。
        if self.git('status', '--porcelain=v1', '--untracked-files=all', '-z').stdout:
            raise RuntimeError('受检工作区存在 tracked 或非忽略 untracked 改动')

    def view(self, head: str, tree: str) -> None:
        if self.text('rev-parse', 'HEAD') != head:
            raise RuntimeError('受检 HEAD 与指定版本不一致')
        if self.text('write-tree') != tree:
            raise RuntimeError('index tree 与 candidate tree 不一致')
        self.git('diff', '--exit-code', '--')
        # Git diff可能按CRLF/clean filter归一化；受检源码还须逐文件匹配index原blob。
        algorithm = self.text('rev-parse', '--show-object-format')
        for entry in self.git('ls-files', '--stage', '-z').stdout.split(b'\0'):
            if not entry:
                continue
            metadata, raw_path = entry.split(b'\t', 1)
            mode, expected, stage = metadata.decode('ascii').split()
            path = self.root / os.fsdecode(raw_path)
            if stage != '0' or mode not in {'100644', '100755', '120000'}:
                raise RuntimeError('index包含未合并项或不支持的gitlink')
            if mode == '120000' and path.is_symlink():
                content = os.fsencode(os.readlink(path))
            elif mode != '120000' and path.is_file() and not path.is_symlink():
                content = path.read_bytes()
                if bool(path.stat().st_mode & 0o111) != (mode == '100755'):
                    raise RuntimeError(f'工作文件mode与index不同：{path}')
            else:
                raise RuntimeError(f'工作文件类型与index不同：{path}')
            digest = hashlib.new(algorithm)
            digest.update(b'blob ' + str(len(content)).encode('ascii') + b'\0')
            digest.update(content)
            if digest.hexdigest() != expected:
                raise RuntimeError(f'工作文件原字节与candidate不同：{path}')
        if self.git('ls-files', '--others', '--exclude-standard', '-z').stdout:
            raise RuntimeError('受检视图含额外非忽略输入')


def prepare(root: Path, base: str, canonical: Path, output: Path) -> int:
    root = root.resolve()
    output = output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    if (output / 'result.json').exists() or (output / 'commands.json').exists():
        raise RuntimeError('证据目录已使用，请保留原件并指定新目录')
    commands = Commands(root, output)
    report: dict = {'root': str(root), 'base': base, 'strictExecuted': False,
                    'producerInvoked': False, 'restorationAttempted': False}
    candidate: str | None = None
    tree: str | None = None
    changed_view = False
    status = 0
    try:
        if Path(commands.text('rev-parse', '--show-toplevel')).resolve() != root:
            raise RuntimeError('--root 必须是实际 Git 根')
        commands.clean()
        candidate = commands.text('rev-parse', 'HEAD')
        tree = commands.text('rev-parse', f'{candidate}^{{tree}}')
        report.update(candidate=candidate, candidateTree=tree)
        commands.view(candidate, tree)
        if not re.fullmatch(r'[0-9a-f]{40}', base) or base == '0' * 40:
            raise RuntimeError('必须提供事件实际的完整非零 base revision')
        commands.git('cat-file', '-e', f'{base}^{{commit}}')
        commands.git('merge-base', '--is-ancestor', base, candidate)
        deleted = commands.git('diff', '--no-renames', '--name-only', '--diff-filter=D',
                               '-z', base, candidate, '--').stdout.decode('utf-8').split('\0')
        deleted = [path for path in deleted if path and path != '.echo-semantic'
                   and not path.startswith('.echo-semantic/')]
        report['deletedCodePaths'] = deleted
        if not deleted:
            report['mode'] = 'no_deletions_original_action_only'
            commands.view(candidate, tree)
            commands.clean()
        else:
            # 只读取精确候选的已版本化原 CLI 参数；不从旧 runtime receipt 生成 payload。
            raw_args = commands.git('show', f'{candidate}:{ARGS_PATH}').stdout
            args = json.loads(raw_args)
            if not isinstance(args, list) or not all(isinstance(item, str) for item in args):
                raise RuntimeError('预检声明必须是原 CLI argv 字符串数组')
            if any(item == '--root' or item.startswith('--root=') or item == 'record'
                   for item in args):
                raise RuntimeError('版本化参数不能指定 root 或 producer 命令')
            report.update(mode='replay_original_producer',
                          argsPath=ARGS_PATH,
                          argsSHA256=hashlib.sha256(raw_args).hexdigest())
            producer = canonical.resolve() / PRODUCER_PATH
            if not producer.is_file():
                raise RuntimeError('缺少同 c0bc 完整工具树的原 producer')
            report['producerPath'] = str(producer)
            report['producerSHA256'] = hashlib.sha256(producer.read_bytes()).hexdigest()
            changed_view = True
            commands.git('switch', '--detach', base)
            commands.git('merge', '--squash', '--no-commit', candidate)
            commands.view(base, tree)
            report['baseHeadCandidateTreeVerified'] = True
            report['producerInvoked'] = True
            result = commands.run([sys.executable, '-B', str(producer), 'record',
                                   '--root', str(root), *args], check=False)
            report['producerReturnCode'] = result.returncode
            if result.returncode:
                raise CommandError([str(producer), 'record'], result.returncode)
            receipt = root / '.echo-semantic/preflight.json'
            if not receipt.is_file() or receipt.is_symlink():
                raise RuntimeError('原 producer 未生成普通预检文件')
            raw_receipt = receipt.read_bytes()
            artifact = output / 'preflight.json'
            with artifact.open('xb') as stream:
                stream.write(raw_receipt)
            if artifact.read_bytes() != raw_receipt:
                raise RuntimeError('预检原件的artifact回读不一致')
            report['preflightArtifact'] = str(artifact)
            report['preflightSHA256'] = hashlib.sha256(raw_receipt).hexdigest()
    except CommandError as error:
        status = error.code
        report['error'] = str(error)
    except (OSError, ValueError, RuntimeError) as error:
        status = 1
        report['error'] = str(error)
    except KeyboardInterrupt:
        status = 130
        report['error'] = '编排被中断'
    finally:
        if changed_view and candidate and tree:
            report['restorationAttempted'] = True
            try:
                # 不用 reset/clean 或 checkout action：保留原 producer 的 ignored 新记录。
                commands.git('switch', '--detach', candidate)
                commands.view(candidate, tree)
                commands.clean()
                if report.get('preflightSHA256'):
                    receipt = root / '.echo-semantic/preflight.json'
                    if hashlib.sha256(receipt.read_bytes()).hexdigest() != report['preflightSHA256']:
                        raise RuntimeError('切回candidate后原预检记录已改变')
                report['candidateRestoredAndClean'] = True
            except (OSError, ValueError, RuntimeError) as error:
                report['candidateRestoredAndClean'] = False
                report['restorationError'] = str(error)
                if not status:
                    status = error.code if isinstance(error, CommandError) else 1
        report['exitCode'] = status
        (output / 'result.json').write_text(
            json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(report, ensure_ascii=False))
    return status


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, required=True)
    parser.add_argument('--base', required=True)
    parser.add_argument('--canonical-root', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    try:
        return prepare(args.root, args.base, args.canonical_root, args.output)
    except (OSError, ValueError, RuntimeError) as error:
        print(str(error), file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
