use std::env;
use std::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    // print all ENV args
    println!("ENV variables:");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    // print all docker secrets
    println!("Docker secrets:");
    let paths = fs::read_dir("/run/secrets/");
    if paths.is_ok() {
        for path in paths.unwrap() {
            let p = path.expect("Failed to get path");
            let file_path = p.path();
            println!("{:?}", file_path);
        }
    }

    let routes = warp::any().map(|| "Hello, jinx-service-example!");

    // listen on 0.0.0.0 for docker
    println!("Listening on: 0.0.0.0:3000");
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
