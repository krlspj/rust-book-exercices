mod application;
mod domain;
mod infrastructure;

use application::ApplicationUserService;

fn main() {
    println!("\x1b[33mHello from main!\x1b[0m");

    let app_service = ApplicationUserService::new();
    let user_info = app_service.get_user(1);
    println!("User Info: {}", user_info);
}
