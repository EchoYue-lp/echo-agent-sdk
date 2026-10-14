"""Python draft callback adapter, consumed by parent before repository publication.

Pure boundary functions only. Official language SDK still owns ACP and invocation
cancellation; HTTP backend owns only its two test Store implementations.
"""
from copy import deepcopy

ITEM_TIMESTAMPS = ('created_at', 'updated_at', 'last_accessed', 'expires_at')


def decode_store_input(operation, request, from_wire):
    value = deepcopy(request)
    if operation == 'store_put':
        value['value'] = from_wire(value['value'])
    return value


def encode_store_outcome(outcome, to_wire):
    if outcome.get('outcome') != 'result':
        return deepcopy(outcome)
    result = deepcopy(outcome['result'])
    operation = result['operation']
    value = result['value']
    def item(raw):
        encoded = deepcopy(raw)
        encoded['value'] = to_wire(encoded['value'])
        for name in ITEM_TIMESTAMPS:
            if encoded[name] is not None:
                encoded[name] = str(encoded[name])
        return encoded
    if operation == 'store_get' and value is not None:
        value = item(value)
    elif operation in {'store_search', 'store_search_with', 'store_list'}:
        value = [item(raw) for raw in value]
    elif operation in {'store_prune_expired', 'store_dedup_by_content'}:
        value = str(value)
    return {'outcome': 'result', 'result': {'operation': operation, 'value': value}}
