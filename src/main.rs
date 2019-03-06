extern crate reqwest;
extern crate rustc_serialize;
extern crate openssl_probe;

use std::env;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use reqwest::header::HeaderMap;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::ACCEPT;
use reqwest::Client;

fn get_iam_token(apikey: &String) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let mut msg_body = String::from("grant_type=urn:ibm:params:oauth:grant-type:apikey&apikey=");
    msg_body.push_str(apikey);
    //println!("{}", msg_body);

    let res = Client::new()
        .post("https://iam.cloud.ibm.com/identity/token")
        .headers(headers)
        .body(msg_body)
        .send()?
        .text()?;
    //println!("{}", res);

    Ok(res)
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let args: Vec<String> = env::args().collect();
    //println!("Size of Vec<String>: {}", args.len());

    if args.len() == 1 {
        println!("Empty argument.");
    } else {
        println!("{}", &args[1]);
    }
    //let apikey = &args[1];
    //println("{}", apikey);
    //println!("{}", get_iam_token(&apikey).unwrap());
}
