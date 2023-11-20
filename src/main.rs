mod crypto;
mod db;
mod encoding;
mod error;
mod fs;
mod log;
mod net;
mod os;
mod web;

fn main() {
    println!("Hello, world!");
    os::info::version()
}

struct Value {}

fn borrow(val: &Value) {}

fn eat(val: Value) {}

#[test]
fn test() {
    let x = Value {};
    let _ref_to_val: &Value = &x;
    // eat(x); //error
    borrow(_ref_to_val);
    eat(x); //ok
}

/*
export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include

*/
