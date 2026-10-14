import unittest
from fixture import Backend
from store_adapter import decode_store_input, encode_store_outcome


class AdapterTests(unittest.TestCase):
    def test_item_unsigned_timestamps_and_native_value_boundary(self):
        raw = {'namespace':['sdk'],'key':'x','value':{'content':'中文🌊'},'created_at':(1<<64)-1,'updated_at':1,'score':None,'importance':5.0,'last_accessed':None,'expires_at':2}
        result = encode_store_outcome({'outcome':'result','result':{'operation':'store_get','value':raw}}, lambda v:{'kind':'native-test-marker','value':v})
        self.assertEqual(result['result']['value']['created_at'],'18446744073709551615')
        self.assertEqual(result['result']['value']['value']['value'], {'content':'中文🌊'})
        self.assertIsNone(result['result']['value']['last_accessed'])
        self.assertEqual(raw['created_at'],(1<<64)-1)
    def test_wire_decode_then_native_backend_is_single_conversion(self):
        request={'namespace':['sdk'],'key':'x','value':{'kind':'string','value':'中文🌊'}}
        decoded=decode_store_input('store_put',request,lambda wire:wire['value'])
        backend=Backend(clock=lambda:100)
        backend.call('A','store_put',decoded)
        self.assertEqual(backend.call('A','store_get',{'namespace':['sdk'],'key':'x'})['value'],'中文🌊')
        self.assertEqual(request['value']['kind'],'string')
    def test_error_outcome_is_preserved_not_default_success(self):
        value={'outcome':'error','error':{'code':'extension_failed','message':'native fault','retryable':'never','details':{'中文':'🌊'}}}
        self.assertEqual(encode_store_outcome(value,lambda _:None),value)


if __name__=='__main__':unittest.main()
