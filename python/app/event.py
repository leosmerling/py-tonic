import py_tonic
py_tonic.init_event("app.event")

__steps__ = ["step1", "step2"]

async def step1(payload: HelloRequest) -> str:
    print("python step1", payload, "name", payload.name)
    return payload.name + "1"

async def step2(payload: str) -> HelloResponse:
    print("python step2", payload)
    return HelloResponse(message=payload + "2")
