"""只验证CI Git编排；mock producer哨兵不是有效预检，更不是strict资格。"""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import textwrap
import unittest


ROOT = Path(__file__).resolve().parents[2]
HELPER = ROOT / 'scripts/prepare-semantic-ci.py'


class SemanticCIViews(unittest.TestCase):
    def setUp(self):
        local = ROOT / '.echo-semantic/local/semantic-ci-tests'
        local.mkdir(parents=True, exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(dir=local)
        self.addCleanup(self.temporary.cleanup)
        self.folder = Path(self.temporary.name)
        self.repo = self.folder / 'repo'
        self.repo.mkdir()
        self.git('init', '-q')
        self.git('config', 'user.name', 'Synthetic CI Test')
        self.git('config', 'user.email', 'ci-fixture@example.invalid')
        self.git('config', 'commit.gpgsign', 'false')
        self.git('config', 'core.autocrlf', 'false')
        (self.repo / '.gitignore').write_text('/.echo-semantic/local/\n/.echo-semantic/preflight.json\n')
        (self.repo / 'keep.txt').write_text('base\n')
        (self.repo / 'old.txt').write_text('retired\n')
        (self.repo / 'future.txt').write_text('not authorized for deletion\n')
        self.commit('base')
        self.base = self.git('rev-parse', 'HEAD').stdout.strip()
        self.tool = self.repo / '.echo-semantic/local/canonical'
        producer = self.tool / 'skills/semantic-preflight/scripts/preflight.py'
        producer.parent.mkdir(parents=True)
        producer.write_text(textwrap.dedent('''\
            import json, os, subprocess, sys
            from pathlib import Path
            root = Path(sys.argv[sys.argv.index('--root') + 1])
            trace = {'mockProducer': True, 'argv': sys.argv[1:],
                     'observedHead': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip(),
                     'candidateMarker': (root / 'keep.txt').read_text()}
            (root / '.echo-semantic/local/mock-call.json').write_text(json.dumps(trace))
            if os.environ.get('MOCK_PRODUCER_FAIL'):
                print('controlled producer failure', file=sys.stderr)
                raise SystemExit(23)
            # 只证明ignored文件经过switch仍保留，不伪造root/base/time或正式payload。
            (root / '.echo-semantic/preflight.json').write_text('MOCK SENTINEL - NOT A VALID PREFLIGHT')
            if os.environ.get('MOCK_PRODUCER_MUTATE'):
                (root / 'keep.txt').write_text('unexpected mutation')
            '''), encoding='utf-8')

    def git(self, *args):
        return subprocess.run(['git', *args], cwd=self.repo, text=True,
                              capture_output=True, check=True)

    def commit(self, message):
        self.git('add', '.')
        self.git('-c', 'commit.gpgsign=false', 'commit', '-qm', message)

    def candidate(self, *, delete=True, extra_delete=False, args=None):
        (self.repo / 'keep.txt').write_text('candidate\n')
        if delete:
            (self.repo / 'old.txt').unlink()
        if extra_delete:
            (self.repo / 'future.txt').unlink()
        config = self.repo / '.github/semantic-preflight-args.json'
        config.parent.mkdir(exist_ok=True)
        self.args = args if args is not None else [
            '--kind', 'contract', '--risk', 'high', '--task-id', 'synthetic-ci',
            '--reuse-existing-boundary', 'existing fixture boundary',
            '--reuse', 'existing fixture', '--verify', 'orchestration test only',
            '--semantic-ref', 'finding.fixture', '--repair-ref', 'finding.fixture',
            '--allow', 'keep.txt', '--allow', 'old.txt', '--delete-path', 'old.txt']
        config.write_text(json.dumps(self.args))
        self.commit('candidate')
        self.tip = self.git('rev-parse', 'HEAD').stdout.strip()
        self.tree = self.git('rev-parse', 'HEAD^{tree}').stdout.strip()

    def run_helper(self, *, base=None, env=None):
        self.output = self.repo / '.echo-semantic/local/proof'
        result = subprocess.run([
            sys.executable, '-B', str(HELPER), '--root', str(self.repo),
            '--base', base or self.base, '--canonical-root', str(self.tool),
            '--output', str(self.output)], text=True, capture_output=True,
            env={**os.environ, **(env or {})})
        self.report = json.loads((self.output / 'result.json').read_text())
        return result

    def assert_candidate_restored(self):
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.tip)
        self.assertEqual(self.git('write-tree').stdout.strip(), self.tree)
        self.assertEqual(self.git('status', '--porcelain').stdout, '')

    def test_original_arguments_run_at_base_on_exact_candidate_tree_and_restore(self):
        self.candidate()
        result = self.run_helper()
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        trace = json.loads((self.repo / '.echo-semantic/local/mock-call.json').read_text())
        self.assertEqual(trace['observedHead'], self.base)
        self.assertEqual(trace['candidateMarker'], 'candidate\n')
        self.assertEqual(trace['argv'], ['record', '--root', str(self.repo), *self.args])
        self.assertTrue(self.report['baseHeadCandidateTreeVerified'])
        self.assertTrue(self.report['candidateRestoredAndClean'])
        self.assertFalse(self.report['strictExecuted'])
        self.assert_candidate_restored()
        self.assertEqual((self.repo / '.echo-semantic/preflight.json').read_text(),
                         'MOCK SENTINEL - NOT A VALID PREFLIGHT')
        self.assertEqual((self.output / 'preflight.json').read_bytes(),
                         (self.repo / '.echo-semantic/preflight.json').read_bytes())

    def test_producer_failure_preserves_real_exit_raw_and_restores_candidate(self):
        self.candidate()
        result = self.run_helper(env={'MOCK_PRODUCER_FAIL': '1'})
        self.assertEqual(result.returncode, 23)
        self.assertEqual(self.report['producerReturnCode'], 23)
        self.assertTrue(self.report['candidateRestoredAndClean'])
        calls = json.loads((self.output / 'commands.json').read_text())
        failed = next(call for call in calls if call['returnCode'] == 23)
        self.assertIn('controlled producer failure', Path(failed['stderr']).read_text())
        self.assert_candidate_restored()

    def test_no_deletion_does_not_invoke_old_authorization_or_switch(self):
        self.candidate(delete=False, args={})
        self.assertEqual(self.run_helper().returncode, 0)
        self.assertFalse(self.report['producerInvoked'])
        calls = json.loads((self.output / 'commands.json').read_text())
        self.assertFalse(any('switch' in item['argv'] for item in calls))
        self.assert_candidate_restored()

    def test_dirty_tracked_input_is_rejected_without_discarding_it(self):
        self.candidate()
        (self.repo / 'keep.txt').write_text('user change\n')
        self.assertEqual(self.run_helper().returncode, 1)
        self.assertFalse(self.report['producerInvoked'])
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.tip)
        self.assertEqual((self.repo / 'keep.txt').read_text(), 'user change\n')

    def test_nonignored_extra_input_is_rejected(self):
        self.candidate()
        (self.repo / 'extra.txt').write_text('extra')
        self.assertEqual(self.run_helper().returncode, 1)
        self.assertFalse(self.report['producerInvoked'])
        self.assertTrue((self.repo / 'extra.txt').is_file())

    def test_index_assume_unchanged_cannot_hide_different_source_bytes(self):
        self.candidate()
        # 真实Git index标志使status/diff不报告变化；逐blob核对仍必须拒绝。
        self.git('update-index', '--assume-unchanged', '--', 'keep.txt')
        (self.repo / 'keep.txt').write_bytes(b'candidate-shadow\n')
        self.assertEqual(self.git('status', '--porcelain').stdout, '')
        self.assertEqual(self.run_helper().returncode, 1)
        self.assertIn('原字节', self.report['error'])
        self.assertFalse(self.report['producerInvoked'])

    def test_unrelated_base_is_rejected(self):
        self.candidate()
        unrelated = self.git('commit-tree', self.tree, '-m', 'unrelated root').stdout.strip()
        self.assertNotEqual(self.run_helper(base=unrelated).returncode, 0)
        self.assertFalse(self.report['producerInvoked'])
        self.assert_candidate_restored()

    def test_invalid_versioned_arguments_are_rejected_before_switch(self):
        self.candidate(args=['--root', '/wrong'])
        self.assertEqual(self.run_helper().returncode, 1)
        self.assertFalse(self.report['restorationAttempted'])
        self.assert_candidate_restored()

    def test_future_deletion_is_not_automatically_added_to_declared_authority(self):
        self.candidate(extra_delete=True)
        self.assertEqual(self.run_helper().returncode, 0)
        trace = json.loads((self.repo / '.echo-semantic/local/mock-call.json').read_text())
        self.assertIn('future.txt', self.report['deletedCodePaths'])
        self.assertNotIn('future.txt', trace['argv'])
        # 原Action还必须执行；mock通过不代表新增删除被真实strict接受。
        self.assertFalse(self.report['strictExecuted'])

    def test_producer_source_mutation_cannot_become_success_during_restore(self):
        self.candidate()
        self.assertEqual(self.run_helper(env={'MOCK_PRODUCER_MUTATE': '1'}).returncode, 1)
        self.assertFalse(self.report['candidateRestoredAndClean'])
        self.assertEqual(self.git('rev-parse', 'HEAD').stdout.strip(), self.tip)
        self.assertEqual((self.repo / 'keep.txt').read_text(), 'unexpected mutation')
        self.assertTrue((self.output / 'preflight.json').is_file())


if __name__ == '__main__':
    unittest.main()
