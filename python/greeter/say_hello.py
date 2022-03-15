import gc
import os
import psutil

from greeter.hw import Extra, HelloRequest, HelloResponse, Status

def execute(request):
    # print("PYTHON", request, type(request)) #, type(request).__dict__)
    # print("status", request.extra.status, "new", Status.NEW, "eq")
    # print("request.extra.status == Status.NEW", request.extra.status == Status.NEW)
    # print("Status.NEW == request.extra.status", Status.NEW == request.extra.status)
    
    # print("hash", hash(request.extra.status), hash(Status.NEW))
    # status_map = {
    #     Status.NEW: None,
    #     Status.OLD: None,
    #     request.extra.status: request.extra,
    # }
    # print(status_map)
    # print("Status.OLD == Status.OLD", Status.OLD == Status.OLD)

    response = HelloResponse(
        message=f"Hello {request.name}",
        extra=Extra(
            text=request.extra.text + " processed",
            numx=request.extra.numx + 1,
            status=Status.OLD if request.extra.status == Status.NEW else Status.UNKNOWN
        ),
    )

    # print("request.extra.status == response.extra.status", request.extra.status == response.extra.status)
    # print("PYTHON", response, type(response))
    # print()
    return response


def memory_footprint():
    """Returns memory (in MB) being used by Python process"""
    # gc.collect()
    mem = psutil.Process(os.getpid()).memory_info().rss
    return int(mem / 1024 )  # snapshot
