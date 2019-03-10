use std::env;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Payload {
    name: String,
}

fn main() {
    /* Read input arguments as a vector of Strings */
    let args: Vec<String> = env::args().collect();

    /* Use serde_json to deserialize a &str into a Payload struct object */
    let mut p: Payload = serde_json::from_str(&args[1]).unwrap();

    /* Work with input data*/
    p.name = format!("Hello {}, this is Rust @ ICF!", p.name);

    /* Log output to stdout */
    println!("{}", serde_json::to_string(&p).unwrap());
}
