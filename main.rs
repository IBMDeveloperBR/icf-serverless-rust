extern crate reqwest;

use std::env;
use reqwest::header::*;

fn get_iam_token(apikey: &String) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let mut msg_body = String::from("grant_type=urn:ibm:params:oauth:grant-type:apikey&apikey=");
    msg_body.push_str(apikey);
    //println!("{}", msg_body);

    let res = reqwest::Client::new()
        .post("https://iam.cloud.ibm.com/identity/token")
        .headers(headers)
        .body(msg_body)
        .send()?
        .text()?;
    //println!("{}", res);

    Ok(res)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let apikey = &args[1];

    println!("{}", get_iam_token(&apikey).unwrap());
}
