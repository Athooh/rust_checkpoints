#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    name: String,
    age: u32,
    role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for WorkerRole {
    fn from(role: &str) -> Self {
        match role.to_lowercase().as_str() {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => panic!("Invalid role"),
        }
    }
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            panic!("Invalid input format");
        }

        let name = parts[0].to_string();
        let age = parts[1].parse::<u32>().expect("Invalid age");
        let role = WorkerRole::from(parts[2]);

        OfficeWorker { name, age, role }
    }
}

