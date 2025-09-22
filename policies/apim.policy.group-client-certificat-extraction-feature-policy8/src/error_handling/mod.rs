use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PolicyError {
    Unauthorized(String),
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error_type: String,
    status_code: u32,
    error_message: String,
    policy_name: String,
    trace_id: String,
}

impl Display for PolicyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let message = match self {
            // PolicyError::Internal(msg) => format!("Internal Error: {}", msg),
            // PolicyError::BadRequest(msg) => format!("Bad Request: {}", msg),
            PolicyError::Unauthorized(msg) => format!("Unauthorized: {}", msg),
            // PolicyError::Forbidden(msg) => format!("Forbidden: {}", msg),
            // PolicyError::NotFound(msg) => format!("Not Found: {}", msg),
            // PolicyError::ServerError(msg) => format!("Server Error: {}", msg),
            // PolicyError::Conflict(msg) => format!("Conflict: {}", msg),
            // PolicyError::BadGateway(msg) => format!("Bad Gateway: {}", msg),
            // PolicyError::NotImplemented(msg) => format!("Not Implemented: {}", msg),
        };

        write!(f, "{}", message) // This only formats, it does not log
    }
}

impl std::error::Error for PolicyError {}

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn handle_error(
        error: PolicyError,
        trace_id: String,
        policy_name: String,
    ) -> (u32, String, String) {
        //logger::error!("{}", error);

        let (status_code, error_type, error_message) = match error {
            // PolicyError::Internal(msg) => (500, "InternalError".to_string(), msg),
            // PolicyError::BadRequest(msg) => (400, "BadRequest".to_string(), msg),
            PolicyError::Unauthorized(msg) => (401, "Unauthorized".to_string(), msg),
            // PolicyError::Forbidden(msg) => (403, "Forbidden".to_string(), msg),
            // PolicyError::NotFound(msg) => (404, "NotFound".to_string(), msg),
            // PolicyError::ServerError(msg) => (500, "ServerError".to_string(), msg),
            // PolicyError::Conflict(msg) => (409, "Conflict".to_string(), msg),
            // PolicyError::BadGateway(msg) => (502, "BadGateway".to_string(), msg),
            // PolicyError::NotImplemented(msg) => (501, "NotImplemented".to_string(), msg),
        };

        let error_response = ErrorResponse {
            error_message,
            status_code: status_code,
            error_type: error_type.clone(),
            policy_name: policy_name.clone(),
            trace_id: trace_id,
        };

        let json_response = serde_json::to_string(&error_response).unwrap_or_else(|_| r#"{"error_message":"Serialization Error","status_code":500,"error_type":"InternalServerError"}"#.to_string());

        // Send properties to the OOTB Message logging policy
        //let policy_bytes = Some(policy_name.as_bytes());
        //stream.set_property(&["policy.name"], policy_bytes);

        (status_code, error_type, json_response)
    }
}

// Example of use: Simple use case
/*fn some_policy_logic() -> Result<(), PolicyError> {
    // Simulate a failure
    Err(PolicyError::BadRequest("Invalid input data".to_string()))
}

fn example_policy_implementation() -> (u32, String) {
    match some_policy_logic() {
        Ok(_) => (200, "".to_string()),
        Err(e) => ErrorHandler::handle_error(e, "abc1233455".to_string(), "x-axa-context".to_string(), stream),
    }
}

fn main() {
    let (status_code, response_body) = example_policy_implementation();
    println!("Status Code: {}", status_code);
    println!("Response Body: {}", response_body);
}*/

mod tests;
