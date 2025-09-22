
use pdk::hl::{StreamProperties};
use pdk::hl::PropertyAccessor;
use hex;
use sha2::{Digest, Sha256};

pub fn read_property(stream: &StreamProperties, path: &[&str]) -> String {
    let bytes = stream.read_property(path).unwrap_or_default();
    String::from_utf8_lossy(&bytes).to_string()
}

pub fn hash_dns_value(dns_value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(dns_value);
    hex::encode(hasher.finalize())
}

pub fn check_not_empty(s: &str) -> Result<(), &'static str> {
    if s.trim().is_empty() {
        Err("SubjectDN is empty")
    } else {
        Ok(())
    }
}


mod tests;