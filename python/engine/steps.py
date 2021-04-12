async def run_event(m, payload):
    steps = getattr(m, "__steps__")
    arg = payload
    for step in steps:
        func = getattr(m, step)
        arg = await func(arg)
    return arg

