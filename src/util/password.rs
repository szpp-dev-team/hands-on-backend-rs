use sha2::{Digest, Sha256};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod test {
    use crate::util::password::hash_password;

    #[test]
    fn test_encrypt_password() {
        const PASSWORD: &str = "szpp3776";

        assert_eq!(
            hash_password(PASSWORD),
            "4ba035334e14de65e2f5a07be6590e12fe05dd3edbc457347cda48432f1e7619"
        );
    }
}
