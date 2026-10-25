//struct practice------------------------------

fn main() {
    struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    }

    let user2 = User {
        email: string::from("another@gmail.com"),
        ..user1
    
    };
}

fn build_user(email: String, username: string) -> User {
    User { active: true,
         username, 
         email,
          sign_in_count: 1, }
    }