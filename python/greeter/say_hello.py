from greeter.hw import HelloRequest, HelloResponse

def execute(request):
    print("PYTHON", request, type(request), type(request).__dict__)
    # print("encode", request.encode())
    # print("decode", HelloRequest.decode(request.encode()))
    response = HelloResponse()
    response.message = f"Hello {request.name}"
    print("PYTHON", response, type(response))
    return response
