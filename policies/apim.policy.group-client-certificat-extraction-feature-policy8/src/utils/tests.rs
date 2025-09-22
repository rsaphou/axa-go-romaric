#[cfg(test)]
mod tests {
    use crate::utils::*;

    use sha2::{Sha256, Digest};

    #[test]
    fn test_hash_dns_value_consistency() {
        let input = "example.com";
        let expected_hash = {
            let mut hasher = Sha256::new();
            hasher.update(input);
            hex::encode(hasher.finalize())
        };

        let actual_hash = hash_dns_value(input);
        assert_eq!(actual_hash, expected_hash);
    }

    #[test]
    fn test_hash_dns_value_empty_string() {
        let input = "";
        let expected_hash = {
            let mut hasher = Sha256::new();
            hasher.update(input);
            hex::encode(hasher.finalize())
        };
        let actual_hash = hash_dns_value(input);
        assert_eq!(actual_hash, expected_hash);
    }

    #[test]
    fn test_check_not_empty_with_non_empty_string() {
        let result = check_not_empty("hello");
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_not_empty_with_empty_string() {
        let result = check_not_empty("");
        assert_eq!(result, Err("SubjectDN is empty"));
    }

}


