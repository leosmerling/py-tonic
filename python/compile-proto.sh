mkdir hw
touch hw/__init.py
python -m grpc_tools.protoc -I=../proto --python_out=./hw ../proto/hw.proto
