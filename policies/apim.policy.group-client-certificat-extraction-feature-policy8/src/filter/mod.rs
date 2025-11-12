use crate::error_handling::{ErrorHandler, PolicyError};
use crate::utils::read_property;
use crate::utils::hash_dns_value;
use crate::utils::check_not_empty;
use pdk::hl::*;
use pdk::logger;

pub async fn request_filter(request_state: RequestState, stream: StreamProperties) -> Flow<()> {
    let headers_state = request_state.into_headers_state().await;
    let headers_handler = headers_state.handler();

    let subject_str = read_property(&stream, &["connection", "subject_peer_certificate"]);
    
    match check_not_empty(&subject_str) {
        Err(err) => {
            let (status_code, _, body) = ErrorHandler::handle_error(
                PolicyError::Unauthorized(err.to_string()),
                "trace-id-placeholder".to_string(),
                "client-auth-policy".to_string(),
            );
            return Flow::Break(Response::new(status_code).with_body(body));
        }
        Ok(_subject) => {        
            //let dns_value = read_property(&stream, &["connection", "dns_san_peer_certificate"]);
            let dns_value = read_property(&stream, &["connection", "subject_peer_certificate"]);
            logger::info!("Certificate content sbject dn: {}", subject_str);

            let hex_str = hash_dns_value(&dns_value);
            headers_handler.set_header("x-axa-cert-client-subject", &hex_str);
            logger::info!("Content of host_decoded: {}", hex_str);

            return Flow::Continue(())
        }
    }

}

mod tests;
