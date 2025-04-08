use crate::domain::UserService;
use crate::infrastructure::UserRepository;

pub struct ApplicationUserService {
    user_service: UserService,
    user_repository: UserRepository,
}

impl ApplicationUserService {
    pub fn new(user_svc: UserService, user_repo: UserRepository) -> Self {
        ApplicationUserService {
            user_service: user_svc,
            user_repository: user_repo,
        }
    }

    pub fn get_user(&self, id: u32) -> String {
        println!("Application: Getting user info for id {}", id);
        let user = self.user_service.get_user(id);
        let repo_user = self.user_repository.find_user(id);
        format!("{} - {} - {}", user.id, user.name, repo_user.name)
    }
}
