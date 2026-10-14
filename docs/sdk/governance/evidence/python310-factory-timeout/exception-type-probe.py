import asyncio
import builtins
import json
import sys


async def observe():
    future = asyncio.get_running_loop().create_future()
    try:
        await asyncio.wait_for(asyncio.shield(future), timeout=0.001)
    except BaseException as error:
        fact = {
            "python": sys.version,
            "executable": sys.executable,
            "asyncio_timeout_is_builtin": asyncio.TimeoutError is builtins.TimeoutError,
            "raised_type": type(error).__module__ + "." + type(error).__qualname__,
            "matches_builtin": isinstance(error, builtins.TimeoutError),
            "matches_asyncio": isinstance(error, asyncio.TimeoutError),
            "shielded_future_cancelled": future.cancelled(),
            "shielded_future_done": future.done(),
        }
        future.set_result(None)
        fact["same_future_can_settle_after_timeout"] = future.result() is None
        print(json.dumps(fact))


asyncio.run(observe())
