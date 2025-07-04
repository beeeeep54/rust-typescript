use std::env;

fn main() {
    let api_port = env::var("API_PORT").expect("API PORT not found");
    println!("cargo:rustc-env=API_URL={}", api_port);
    println!("cargo:rerun-if-env-changed=API_URL");
}
