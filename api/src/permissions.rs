use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Default, Serialize)]
pub struct Permissions(u64);

bitflags! {
    impl Permissions: u64 {
        // Texture Handling 0-19
        const TEXTURE_EDIT = 1 << 00;
        const TEXTURE_USE = 1 << 01;
        // User and group Handling 20-29
        const USER_EDIT = 1 << 20;
        const GROUP_EDIT = 1 << 21;

    }
}

impl Permissions {
    pub fn from_str(string: &str) -> Self {
        let val = string.parse().unwrap_or_default();
        Self(val)
    }

    pub fn to_str(&self) -> String {
        self.0.to_string()
    }
}

impl From<i64> for Permissions {
    fn from(value: i64) -> Self {
        Self(value as u64)
    }
}

impl Into<i64> for Permissions {
    fn into(self) -> i64 {
        self.0 as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permissions() {
        let user_perm= Permissions::TEXTURE_USE;
        let creator_perm= Permissions::TEXTURE_EDIT;
        let admin_perm=Permissions::GROUP_EDIT |Permissions::USER_EDIT;



        dbg!(user_perm);
        dbg!(creator_perm);
        dbg!(admin_perm);
    }
}
