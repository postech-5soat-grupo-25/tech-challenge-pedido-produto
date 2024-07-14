use mockall::*;

#[derive(Debug, PartialEq)]
pub enum UserGroup {
    Admin,
    Kitchen,
}

#[automock]
pub trait UserGroupValidatorAdapter{
    fn validate_user_group(&self, comming_user_group: String, expected_user_group: UserGroup) -> bool;
}
