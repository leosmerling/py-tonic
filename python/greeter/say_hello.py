import gc
import os
import psutil

from greeter.hw import Extra, HelloRequest, HelloResponse

def execute(request):
    # print("PYTHON", request, type(request), type(request).__dict__)
    # print("encode", request.encode())
    # print("decode", HelloRequest.decode(request.encode()))
    response = HelloResponse()
    response.message = f"Hello {request.name}"
    # response.extra = request.extra
    extra = Extra()
    extra.text= request.extra.text + " processed"
    extra.num = request.extra.num + 1
    response.extra = extra
    # print("PYTHON", response, type(response))
    return response


def memory_footprint():
    """Returns memory (in MB) being used by Python process"""
    # gc.collect()
    mem = psutil.Process(os.getpid()).memory_info().rss
    return int(mem / 1024 )  # snapshot
