mod auth;
#[allow(dead_code)]
mod sessions;
#[allow(dead_code)]
mod users;

fn main() {
    pretty_env_logger::init();
    println!("auth-service");
}
