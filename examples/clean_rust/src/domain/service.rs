use super::model::User;

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        UserService
        //return UserService;
    }

    pub fn get_user(&self, id: u32) -> User {
        println!("Domain: Getting user with id {}", id);
        return User {
            id,
            name: format!("User: {}", id),
        };
    }
}
