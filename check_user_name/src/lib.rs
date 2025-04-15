pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

pub struct User {
    name: String,
    level: AccessLevel,
}

impl User {
  pub fn new(name: String, level: AccessLevel) -> User {
    User {
        name,
        level,
    }
  }
  pub fn send_name(&self) -> Option<&str> {
    match self.level {
        AccessLevel::Guest => None,
        _ => Some(&self.name),
        // AccessLevel::Normal => Some(&self.name),
        // AccessLevel::Admin => Some(&self.name),
    }
  }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name() {
        Some(name) => (true, name),
        None => (false, "ERROR: User is guest")
    }
}