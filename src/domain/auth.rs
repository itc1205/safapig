pub trait Authentificator<C, U, I>
    where C: Credentials,
          U: User,
          I: Identity
{
   fn registrate(&mut self, credentials: C) -> Result<I, AuthentificationError>; 
   fn login(&self, credentials: C) -> Result<U, AuthentificationError>;
}

pub trait Identity {
    fn as_string(&self) -> String;
}

pub trait Role {
    // Returns user level of role
    fn as_integer(&self) -> u8;
}

pub trait User {
    fn get_user_role(&self) -> impl Role;
    fn get_user_identity(&self) -> impl Identity;
    fn get_user_credentials(&self) -> impl Credentials;
}

pub trait Credentials {
    fn get_login(&self) -> String;
    fn get_hashed_password(&self) -> String;
}

pub trait AuthentificatorRepository
{
    type C: Credentials;
    type U: User;
    type I: Identity;
    fn get_user(&self, credentials: &Self::C) -> Option<Self::U>;
    fn save_user(&mut self, credentials: Self::C) -> Self::I;
}

pub struct AuthentificationError {
}

pub struct AuthentificationManager<R>
    where R: AuthentificatorRepository 
{
    repo: R,
}
impl<R> Authentificator<R::C, R::U, R::I> for AuthentificationManager<R>
    where R: AuthentificatorRepository,
{
    fn login(&self, credentials: R::C)-> Result<R::U, AuthentificationError> {
        let user = self.repo.get_user(&credentials);
        if user.is_none() {
            return Err(AuthentificationError {});
        }
        Ok(user.unwrap())
    }

    fn registrate(&mut self, credentials: R::C) -> Result<R::I, AuthentificationError> {
        let user = self.repo.get_user(&credentials);
        if user.is_some() {
            return Err(AuthentificationError {});
        }
        Ok(self.repo.save_user(credentials))
    }
}
