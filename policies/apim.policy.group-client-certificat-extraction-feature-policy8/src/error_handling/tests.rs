#[cfg(test)]
mod tests {
    use crate::error_handling::ErrorResponse;

    use serde_json;
    #[test]
    fn test_error_response_deserialization() {
        let json = r#"
            {
                "error_type": "Unauthorized",
                "status_code": 401,
                "error_message": "Email missing from peer cert.",
                "policy_name": "client-auth-policy",
                "trace_id": "trace-id-placeholder"
            }
        "#;

        let parsed: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.error_type, "Unauthorized");
        assert_eq!(parsed.status_code, 401);
        assert_eq!(parsed.error_message, "Email missing from peer cert.");
        assert_eq!(parsed.policy_name, "client-auth-policy");
        assert_eq!(parsed.trace_id, "trace-id-placeholder");
    }
}
