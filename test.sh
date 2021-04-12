#!env bash
export RUST_LOG=warn
export PYTHONPATH=.
rm nohup.out

echo "USAGE: test.sh N (native|python) - current args: $1 $2 $3"

cargo build --release --lib
cd python && ln -s ../target/release/libpy_tonic.so py_tonic.so && cd ..
cargo build --release --bin test-client

echo "Starting server..."
export PYTHONPATH=./python && nohup python python/main.py &
sleep 0.5

echo "Starting $1 hw-clients..."
for (( i=1; i<=$1; i++ ))
do
    echo "Starting client $i..."
    nohup target/release/test-client $2 $3 &
done
sleep 1

tail -f nohup.out
