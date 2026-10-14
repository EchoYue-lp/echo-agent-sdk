"""Independent exact-value oracle over saved actual Host observations."""
import hashlib
import os
import json
import math
import unittest
from pathlib import Path

ROOT=Path(__file__).parent
EVIDENCE=json.loads((ROOT/os.environ.get('COMPUTATION_EVIDENCE','python-computation-evidence.json')).read_text())
ROWS={row['id']:row for row in EVIDENCE['rows']}


def wire(value):
    kind=value['kind']
    if kind=='null':return None
    if kind in {'string','bool','f64'}:return value['value']
    if kind in {'u64','i64'}:return int(value['value'])
    if kind=='list':return [wire(item)for item in value['value']]
    if kind=='map':return {wire(item['key']):wire(item['value'])for item in value['value']}
    if kind=='variant':return value
    raise AssertionError(kind)


def data(identifier):
    result=ROWS[identifier]['result'];output=json.loads(result['output'])
    assert result['success']is True
    assert wire(result['data'])==output,identifier+' ToolResult output/data mismatch'
    return output


class ExactGreenObservations(unittest.TestCase):
    def test_statistics_known_sample_and_no_inference(self):
        value=data('stats-exact');column=value['columns'][0]
        self.assertFalse(value['inference']);self.assertEqual(value['analysis_kind'],'exploratory_descriptive')
        self.assertEqual((column['mean'],column['p25'],column['median'],column['p75']),(2.5,1.75,2.5,3.25))
        self.assertAlmostEqual(column['sample_std_dev'],math.sqrt(5/3))
        self.assertEqual((column['total_count'],column['valid_count'],column['missing_or_non_finite_count']),(4,4,0))
        self.assertEqual(data('stats-nonnumeric')['columns'],[{'name':'label','error':'Non-numeric column'}])
    def test_structured_errors_paths_missing_null_and_limit(self):
        self.assertEqual(ROWS['structured-valid']['result'],{'valid':True,'errors':[]})
        invalid=ROWS['structured-invalid']['result'];self.assertFalse(invalid['valid']);self.assertEqual({error['instance_path']for error in invalid['errors']},{'/count','/name'})
        self.assertEqual(ROWS['structured-no-sample']['result'],{'valid':True,'errors':[]})
        self.assertFalse(ROWS['structured-null-sample']['result']['valid']);self.assertEqual(ROWS['structured-null-sample']['result']['errors'][0]['instance_path'],'')
        self.assertEqual(len(ROWS['structured-error-bound']['result']['errors']),16)
        self.assertEqual(ROWS['structured-bad-schema']['error']['code'],'invalid_value');self.assertEqual(ROWS['structured-too-large']['error']['code'],'payload_too_large')
    def test_guard_exact_unicode_bytes_and_modes(self):
        match=ROWS['guard-utf8-detect']['result']['matches'][0]
        self.assertEqual((int(match['start']),int(match['end']),match['matched']),(8,25,'alice@example.com'))
        self.assertEqual(ROWS['guard-utf8-redact']['result'],{'redacted':'中😀 [REDACTED:EMAIL] 尾'})
        for mode,variant in [('detect','detected'),('reject','rejected'),('redact','redacted')]:self.assertEqual(ROWS['guard-'+mode]['result']['value']['variant'],variant)
    def test_chart_all_seven_mark_mappings(self):
        for kind,mark in [('line','line'),('bar','bar'),('pie','arc'),('scatter','point'),('area','area'),('heatmap','rect'),('boxplot','boxplot')]:
            spec=json.loads(ROWS['chart-'+kind]['result']['output']);self.assertEqual(spec['mark'],mark);self.assertEqual((spec['width'],spec['height'],spec['title']),(321,123,'中😀标题'));self.assertEqual(spec['data']['values'],[{'label':'中文😀','value':1},{'label':'尾','value':4}])
    def test_data_aggregate_join_and_quality_exact(self):
        aggregate=data('data-aggregate');self.assertEqual(aggregate['rows'],2);self.assertEqual(aggregate['columns'],4);self.assertFalse(aggregate['truncated'])
        self.assertEqual(sorted(aggregate['data'],key=lambda row:row['group']),[{'group':'a','value_sum':3,'value_mean':1.5,'value_count':2},{'group':'b','value_sum':7,'value_mean':3.5,'value_count':2}])
        joined=data('data-join');self.assertEqual(joined['total_rows'],4);self.assertEqual(sorted(joined['preview'],key=lambda row:row['id']),[{'id':1,'group':'a','quarter':'q1','value':1,'label':'中文'},{'id':2,'group':'a','quarter':'q2','value':2,'label':None},{'id':3,'group':'b','quarter':'q1','value':3,'label':'emoji😀'},{'id':4,'group':'b','quarter':'q2','value':4,'label':None}])
        missing=data('data-missing');self.assertEqual(missing['total_missing_cells'],1);self.assertEqual(missing['overall_missing_pct'],8.33);column=next(row for row in missing['per_column']if row['name']=='value');self.assertEqual((column['total'],column['non_null'],column['missing_count'],column['missing_pct']),(4,3,1,25.0))
        consistency=data('data-consistency');self.assertEqual(consistency['total_issues'],1);self.assertEqual(consistency['issues'][0]['detail'],'2 values outside range');self.assertEqual(consistency['issues'][0]['type'],'range_violation')
    def test_error_fidelity_and_unchanged_failure_sentinel(self):
        for row in EVIDENCE['rows']:
            if'error'in row:
                self.assertEqual(row['error']['retryable'],'never');self.assertTrue(row['error']['operation']);self.assertTrue(row['error']['message'])
        for name in ['stats-unknowncolumn','chart-unknown','chart-wrongformat','data-join-keyfail','data-pivot-colfail','data-export-preserve-sentinel']:
            error=ROWS[name]['error'];self.assertEqual(error['code'],'framework_error');self.assertIn({'key':'framework_terminal_kind','value':'failed'},error['details']['fields'])
        self.assertEqual(ROWS['chart-trailingnullarity']['error']['code'],'invalid_value')
        self.assertEqual(EVIDENCE['actual_side_effects']['sentinel_contents'],'unchanged\n');self.assertEqual(EVIDENCE['actual_side_effects']['sentinel_sha256'],hashlib.sha256(b'unchanged\n').hexdigest())

    def test_generic_validation_preserves_outputs_before_failed_effects(self):
        self.assertEqual(EVIDENCE['actual_exports']['output_text'],'中文😀\nalpha')
        self.assertEqual(EVIDENCE['actual_exports']['output_json'],[{'id':1,'value':1},{'id':2,'value':2},{'id':3,'value':3},{'id':4,'value':4}])
        for name in ['media-export-missing-required','media-export-wrongoptional-type','chart-wrong-width-type','chart-required-missing','stats-required-missing']:
            error=ROWS[name]['error'];self.assertEqual(error['code'],'framework_error');self.assertEqual(error['retryable'],'never');self.assertIn({'key':'framework_failure_category','value':'tool'},error['details']['fields'])
        self.assertEqual(ROWS['media-export-extra-null']['error']['code'],'invalid_value')
        self.assertEqual(EVIDENCE['actual_side_effects']['sentinel_sha256'],hashlib.sha256(b'unchanged\n').hexdigest())


class OriginalSemanticCounterexamples(unittest.TestCase):
    def test_overlap_should_redact_complete_card_once(self):
        self.assertEqual(ROWS['guard-overlap-redact']['result'],{'redacted':'[REDACTED:CARD]'})
    def test_pivot_should_preserve_every_input_cell(self):
        value=data('data-pivot')['pivot_result']
        self.assertEqual(set(value['pivot_values']),{'q1','q2'})
        self.assertEqual(sorted(value['data'],key=lambda row:row['group']),[{'group':'a','q1':1,'q2':2},{'group':'b','q1':3,'q2':4}])

if __name__=='__main__':unittest.main()
