// use reqwest;
// GET with reqwest

// #[tokio::main]
// async fn main(){
//     let resp = match reqwest::get("https://httpbin.org/ip").await {
//         Ok(resp) => resp.text().await.unwrap(),
//         Err(err) => panic!("Error: {}", err)
//     };
//     println!("{}", resp);
// }


// POST with reqwest
// use reqwest;
// use std::collections::HashMap;

// fn main(){
//     let mut map = HashMap::new();
//     map.insert("msg", "hello from reqwests!");

//     let client = reqwest::blocking::Client::new();
//     let resp = match client.post("http://httpbin.org/post").json(&map).send(){
//         Ok(resp) => resp.text().unwrap(),
//         Err(err) => panic!("Error: {}", err)
//     };
//     println!("{}", resp)
// }

use hyper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut client = hyper::Client::new();

    let req = hyper::Request::builder()
    .method(hyper::Method::GET)
    .uri("http://httpbin.org/ip")
    .header("user-agent", "the-awesome-agent/007")
    .body(hyper::Body::from(""))?;

let resp = client.request(req).await?;

let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;

let body = String::from_utf8(body_bytes.to_vec()).unwrap();

println!("{}", body);

Ok(())
}

