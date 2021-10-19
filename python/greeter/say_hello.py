from greeter.hw import HelloResponse

def execute(request):
    print("PYTHON", request, type(request))
    response = HelloResponse()
    response.message = f"Hello {request.name}"
    print("PYTHON", response, type(response))
    return response
