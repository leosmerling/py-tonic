import gc
import os
import psutil
import time

from greeter.hw_pb2 import Extra, HelloRequest, HelloResponse

def execute(request):
    # print("PYTHON", request, type(request), type(request).__dict__)
    # print("encode", request.encode())
    # print("decode", HelloRequest.decode(request.encode()))
    response = HelloResponse()
    response.message = f"Hello {request.name}"
    # response.extra = request.extra
    response.extra.text= request.extra.text + " processed"
    response.extra.num = request.extra.num + 1
    # print("PYTHON", response, type(response))
    return response


def memory_footprint():
    """Returns memory (in MB) being used by Python process"""
    # gc.collect()
    mem = psutil.Process(os.getpid()).memory_info().rss
    return int(mem / 1024 )  # snapshot

def main():
    batch_size = 1000000
    now = time.time_ns()
    for i in range(10 * batch_size):
        req = HelloRequest()
        req.name = "python"
        req.extra.text = "python test"
        req.extra.num = i
        res = execute(req)
        if i % batch_size == 0:
            elapsed = time.time_ns() - now
            print("RES", res)
            print("elapsed: {}ms time/req:{}ns, mem: {}".format(
                elapsed / 10**6, elapsed / batch_size, memory_footprint()
            ))
            now = time.time_ns()

if __name__ == "__main__":
    main()
