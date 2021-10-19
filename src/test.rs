mod lib;
use lib::hw::{HelloRequest, HelloResponse};
use lib::{setup, say_hello};

fn main() {
    setup().expect("Err setup");
    let request = HelloRequest { name: "nobody".into() };
    let result = say_hello(request);
    let expected = HelloResponse { message: "Hello nobody".into() };
    assert_eq!(result, expected);
}
