mod application;
mod domain;
mod infrastructure;

use application::ApplicationUserService;
use domain::UserService;
use infrastructure::UserRepository as UserRepo;
//use crate::infrastructure::db::conversation_repository::UserRepository;

fn main() {
    println!("\x1b[33mHello from main!\x1b[0m");

    let user_svc = UserService::new();
    let user_repo = UserRepo::new();

    let app_service = ApplicationUserService::new(user_svc, user_repo);

    let user_info = app_service.get_user(1);

    println!("User Info: {}", user_info);
}
