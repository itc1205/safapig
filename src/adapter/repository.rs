use crate::domain::auth::{AuthentificatorRepository, Credentials, User, Role, Identity};

struct InMemoryAuthentificationRepository {
    db: Vec<UserMock>
}

#[derive(Clone)]
struct UserMock {
    credentials: CredentialsMock,
    role: MockRole,
    identity: IdentityMock,
}


#[derive(Clone)]
struct CredentialsMock {
    login: String, 
    password: String,
}

#[derive(Clone, Copy)]
struct IdentityMock {
    id: u128,
}

#[derive(Clone, Copy)]
enum MockRole {
    USER = 2,
    MODERATOR = 3,
    ADMIN = 4,
    CREATOR = 5
}

impl PartialEq for CredentialsMock {
    fn eq(&self, other: &Self) -> bool {
        self.login == other.login
    }
}

impl Identity for IdentityMock {
    fn as_string(&self) -> String {
        self.id.to_string()
    }
}

impl Role for MockRole {
    fn as_integer(&self) -> u8 {
       *self as u8 
    }
}

impl Credentials for CredentialsMock {
    fn get_login(&self) -> String {
        self.login.clone()
    }        
    fn get_hashed_password(&self) -> String {
        self.password.clone()
    }
}

impl User for UserMock {
    fn get_user_role(&self) -> impl Role {
        self.role 
    }
    fn get_user_identity(&self) -> impl Identity {
        self.identity
    }
    fn get_user_credentials(&self) -> impl Credentials {
        self.credentials.clone()
    }
}

impl AuthentificatorRepository for InMemoryAuthentificationRepository {
    type C = CredentialsMock;
    type U = UserMock; 
    type I = IdentityMock;

    fn get_user(&self, credentials: &Self::C) -> Option<Self::U> {
        for user in &self.db {
            if user.credentials == *credentials {
                return Some(user.clone());
            }
        }
        None
    }

    fn save_user(&mut self, credentials: Self::C) -> Self::I {
        let identity = IdentityMock{id: self.db.len() as u128};
        self.db.push(UserMock {
            credentials,
            identity,
            role: MockRole::USER
        });
        identity
    }
}
