#[cfg(test)]
mod tests {
    use crate::subject_dn::{decode_hex_email, parse_subject_dn};
    
    #[test]
    fn test_parse_subject_with_all_fields() {
        let subject_str = "CN=John Doe,O=Example Corp,OU=Engineering,L=Makati,ST=NCR,C=PH,1.2.840.113549.1.9.1=email@example.com";
        let result = parse_subject_dn(subject_str).unwrap();

        assert_eq!(result.email.as_deref(), Some("email@example.com"));
        assert_eq!(result.common_name, "John Doe");
        assert_eq!(result.organization_unit, vec![
            "Engineering",
        ]);
        assert_eq!(result.organization.as_deref(), Some("Example Corp"));
        assert_eq!(result.location.as_deref(), Some("Makati"));
        assert_eq!(result.state.as_deref(), Some("NCR"));
        assert_eq!(result.country.as_deref(), Some("PH"));
    }

    #[test]
    fn test_parse_subject_with_some_fields() {
        let subject_str = "cn=surrender-api.axa-li-jp-preprod-lpl-int,ou=permittedtemplate:axajapanserver,ou=technology division,o=axa life japan,c=jp";
        let result = parse_subject_dn(subject_str).unwrap();

        assert_eq!(result.email, None);
        assert_eq!(result.common_name, "surrender-api.axa-li-jp-preprod-lpl-int");
        assert_eq!(result.organization_unit, vec![
            "permittedtemplate:axajapanserver",
            "technology division"
        ]);
        assert_eq!(result.organization.as_deref(), Some("axa life japan"));
        assert_eq!(result.location, None);
        assert_eq!(result.state, None);
        assert_eq!(result.country.as_deref(), Some("jp"));
    }

    #[test]
    fn test_parse_subject_with_hex_encoded_email() {
        // "email@example.com" in hex is: 656d61696c406578616d706c652e636f6d
        let subject_str = "CN=John Doe,1.2.840.113549.1.9.1=#656d61696c406578616d706c652e636f6d";
        let result = parse_subject_dn(subject_str).unwrap();

        assert_eq!(result.email.as_deref(), Some("email@example.com"));
        assert_eq!(result.common_name, "John Doe");
    }

    #[test]
    fn test_parse_subject_missing_email() {
        let subject_str = "CN=John Doe,O=Example Corp";
        let result = parse_subject_dn(subject_str).unwrap();
        assert_eq!(result.email, None);
    }

    #[test]
    fn test_parse_subject_missing_common_name() {
        let subject_str = "1.2.840.113549.1.9.1=email@example.com,O=Example Corp";
        let result = parse_subject_dn(subject_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Common name missing from peer cert.");
    }

    #[test]
    fn test_decode_hex_email_valid() {
        let encoded = "#656d61696c406578616d706c652e636f6d"; // "email@example.com"
        let decoded = decode_hex_email(encoded).unwrap();
        assert_eq!(decoded, "email@example.com");
    }

    #[test]
    fn test_decode_hex_email_invalid_format() {
        let encoded = "656d61696c406578616d706c652e636f6d"; // missing '#'
        let decoded = decode_hex_email(encoded);
        assert!(decoded.is_none());
    }

    #[test]
    fn test_decode_hex_email_invalid_hex() {
        let encoded = "#ZZZZZZ"; // invalid hex
        let decoded = decode_hex_email(encoded);
        assert!(decoded.is_none());
    }
}
