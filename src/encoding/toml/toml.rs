use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    title: String,
    owner: Owner,
}

#[derive(Deserialize, Debug)]
struct Owner {
    #[serde(default)]
    name: String,
    #[serde(default = "default_nickname")]
    nickname: String,
    #[serde(default = "default_age")]
    age: u32,
}

fn default_nickname() -> String {
    "昵称".to_string()
}

fn default_age() -> u32 {
    88
}

#[test]
fn de_file() {
    let path = "data/demo.toml";
    let s = fs::read_to_string(path).unwrap();
    let config: Config = toml::from_str(&s).unwrap();
    println!("{:?}", config)
}
