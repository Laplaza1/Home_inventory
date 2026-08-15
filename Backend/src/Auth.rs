use std::{format, print, println};
use dotenv::dotenv;
use std::env;
use reqwest::{Method, Request, Url};
use serde_json::json;
use reqwest::Client;




//  Example Auth02

//  async fn call_auth(){
//     let _ = dotenv().is_ok();
//     let domain = match env::var("DOMAIN"){
//                     Ok(val) => {val},
//                     Err(e) => println!("Error: {}",e)
//                 };
//     let url = format!("https://{}/oauth/token",domain);
//     //let x = reqwest::Request::new(Method::POST, Url::parse(&url));
//     let client = Client::new();
    
//     let payloaded = json!({
//         "client_id": env::var("CLIENTID"),
//         "client_secret": env::var("CLIENT_SECRET"),
//         "audience": env::var("AUDIENCE"),
//         "grant_type": "client_credentials"
//     });
//     let response =client.post(url).json(&payloaded).send().await.ok();
//     println!("{:?}", response.text().await);
// }

