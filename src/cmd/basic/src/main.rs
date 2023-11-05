use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file_name = String::new();
    let mut flag_a = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-a" => {
                flag_a = true;
            }
            _ => {
                file_name = arg.to_string();
            }
        }
    }

    println!("File name: {}", file_name);
    println!("Flag a: {}", flag_a);
}
