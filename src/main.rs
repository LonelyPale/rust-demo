mod crypto;
mod db;
mod fs;
mod net;
mod os;

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
