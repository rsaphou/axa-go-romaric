use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct SubjectDn {
    pub email: Option<String>,
    pub common_name: String,
    pub organization_unit: Vec<String>,
    pub organization: Option<String>,
    pub location: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
}

pub fn parse_subject_dn(subject_field: &str) -> Result<SubjectDn> {
    let mut email = None;
    let mut cn = None;
    let mut ou: Vec<String> = Vec::new();
    let mut o = None;
    let mut l = None;
    let mut st = None;
    let mut c = None;

    for segment in subject_field.split(',') {
        let kv: Vec<&str> = segment.splitn(2, '=').collect();
        if kv.len() != 2 {
            continue;
        }

        let key = kv[0].trim().to_lowercase();
        let value = kv[1].trim();

        match key.as_str() {
            "1.2.840.113549.1.9.1" => {
                if value.starts_with('#') {
                    email = decode_hex_email(value);
                } else {
                    email = Some(value.to_string());
                }
            }
            "cn" => cn = Some(value.to_string()),
            "ou" => ou.push(value.to_string()),
            "o" => o = Some(value.to_string()),
            "l" => l = Some(value.to_string()),
            "st" => st = Some(value.to_string()),
            "c" => c = Some(value.to_string()),
            _ => {}
        }
    }

    Ok(SubjectDn {
        email,
        common_name: cn.ok_or(anyhow!("Common name missing from peer cert."))?,
        organization_unit: ou,
        organization: o,
        location: l,
        state: st,
        country: c,
    })
}


fn decode_hex_email(encoded: &str) -> Option<String> {
    if !encoded.starts_with('#') {
        return None;
    }

    let hex_str = &encoded[1..];
    let bytes = hex::decode(hex_str).ok()?;
    Some(String::from_utf8_lossy(&bytes).to_string())
}

mod tests;
