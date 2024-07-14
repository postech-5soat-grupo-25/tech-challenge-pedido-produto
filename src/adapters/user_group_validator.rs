use std::str::FromStr;

use crate::traits::user_group_validator_adapter::{UserGroup, UserGroupValidatorAdapter};

pub struct UserGroupValidator;

impl FromStr for UserGroup {
  type Err = ();

  fn from_str(input: &str) -> Result<UserGroup, Self::Err> {
      match input {
          "Admin" => Ok(UserGroup::Admin),
          "Kitchen" => Ok(UserGroup::Kitchen),
          _ => Err(()),
      }
  }
}

impl UserGroupValidator {
    pub fn new() -> Self {
        Self
    }
}

impl UserGroupValidatorAdapter for UserGroupValidator {
  fn validate_user_group(&self, comming_user_group: String, expected_user_group: UserGroup) -> bool {
    let comming_user_group = UserGroup::from_str(&comming_user_group);
    match comming_user_group {
        Ok(comming_user_group) => comming_user_group == expected_user_group,
        Err(_) => false,
    }
  }
}

unsafe impl Sync for UserGroupValidator {}
unsafe impl Send for UserGroupValidator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_user_group() {
        let user_group_validator = UserGroupValidator::new();

        assert_eq!(user_group_validator.validate_user_group("Admin".to_string(), UserGroup::Admin), true);
        assert_eq!(user_group_validator.validate_user_group("Kitchen".to_string(), UserGroup::Kitchen), true);
        assert_eq!(user_group_validator.validate_user_group("invalid".to_string(), UserGroup::Admin), false);
        assert_eq!(user_group_validator.validate_user_group("invalid".to_string(), UserGroup::Kitchen), false);
    }
}