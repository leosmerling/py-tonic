# py-tonic
POC (for now) of a gRPC server made in Rust (based on Tonic+PYO3 crates) to server Python code.

### Idea

Spin up a gRPC server that runs natively but with the ability
to invoke business logic written in Python.

Main idea comes from an async HTTP framework (https://github.com/hopeit-git/hopeit.engine) where business logic is defined in separated python files, and the framework integrates the HTTP serving+logging+metrics to make life easy for Python developers when creating Microservices or dealing with async scenarios.

## Use Cases

- Machine Learning model serving (where serving logic needs to be written in Python)
- Real time calculations using Python libraries + existing code
- Helping Data Scientists to create decent performance services
- ...

### Test
- Create a python environment with python 3.7+ (i.e. conda environment)
- Have rust nightly installed
- Run
```
./test.sh 3 python
```
This will:
- Build py-tonic binaries and create symlink
- Start a server by running `python/main.py`
- Start 3 clients (generated in Rust using tonic)
- Invoke for each request, `python\steps.py` using the module `app/event.py`

TODOs:
- [ ] Rust generated classes from hw.proto needs to be manually replicated to be used in Python in order to use `#[pyclass]` macro. This should be automated (generated classes should be python compatible or new classes should be automaticaly created with `impl From`).
- [ ] Code generation from proto schemas done using `build.rs` could be triggered from python.
- [ ] `steps.py` is used to make python code compatible with `hopeit.engine`, but we might can simplify this por this project just to serve a single python function since loading both `steps.py` and the actual code to run seems to be a performance bottleneck.
- [ ] Python modules could be statically loaded at start time (needs to be Boxed)
- [ ] Add metrics to the service (similar at what's done in `client.rs`)
- [ ] do not use "expect"
- [ ] ... add more ideas ...
