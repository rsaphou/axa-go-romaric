//Working without error
mod generated;
mod utils;
use anyhow::{anyhow, Result};
use pdk::hl::*;
use pdk::logger;
use serde_json::json;
use serde_json::Value;
use crate::generated::config::Config;
use pdk::metadata::Metadata;
use crate::utils::vec_u8_to_int;
use crate::utils::get_content_body_and_length; 
use crate::utils::HeadersType;
use crate::utils::MAX_BUFFER_SIZE;

async fn request_filter(request_state: RequestState, _config: &Config, _metadata: &Metadata, stream: StreamProperties) -> Flow<(Option<String>, Option<String>, String)> {
    // Get trace ID
    let trace_id = String::from_utf8(stream.read_property(&["request", "id"]).unwrap_or_default()).unwrap_or_default();
    let headers_state = request_state.into_headers_state().await;
    let mut body_bytes = String::new();
    let mut body_content = String::new();
    // Get request body content and message
    //let body_state = headers_state.into_body_state().await;
    
    let (bytes, content) = get_content_body_and_length(HeadersType::RequestHeaders(headers_state)).await;
    body_bytes = bytes;
    body_content = content;
    logger::debug!("Request body bytes length: {}", body_bytes.to_string());
    logger::debug!("Request body content :  {}", body_content.len());
    //let body_content = String::from_utf8_lossy(&body_state.handler().body()).to_string();
    let body_message = if let Ok(json_body) = serde_json::from_str::<Value>(&body_content) {
        json_body.get("message").and_then(|m| m.as_str()).map(String::from)
    } else {
        None
    };
    Flow::Continue((Some(body_content), body_message, trace_id))
}

async fn response_filter(response_state: ResponseState, _config: &Config, request_data: RequestData<(Option<String>, Option<String>, String)>) {
    let headers_state = response_state.into_headers_state().await;
    let content_length = headers_state.handler().header("content-lenght").unwrap_or("0".to_owned());
    let response_status = headers_state.status_code();
    let has_error = !response_status.to_string().starts_with("20");
    
    let mut log_data = json!({});
    logger::debug!("Response status: {}, Content-Length: {}", response_status, content_length);
    if let RequestData::Continue((body_content, body_message, trace_id)) = request_data {
        log_data.as_object_mut().unwrap().insert("traceId".to_string(), serde_json::Value::String(trace_id));
       // log_data.as_object_mut().unwrap().insert("http.request.body.content".to_string(), Value::String(content));

        // Add body content and message with appropriate field names based on error status
        if let Some(content) = body_content {
            //if has_error {
            //    log_data.as_object_mut().unwrap().insert("error.body.content".to_string(), Value::String(content));
           // } else {
                logger::debug!("Logging request body content of length: {}", content.len());
                log_data.as_object_mut().unwrap().insert("http.request.body.content".to_string(), Value::String(content));
            //}
        }
        if let Some(message) = body_message {
         //   if has_error {
          //      log_data.as_object_mut().unwrap().insert("error.body.message".to_string(), Value::String(message));
           // } else {
                logger::debug!("Logging request body message: {}", message.len());
                log_data.as_object_mut().unwrap().insert("http.request.body.message".to_string(), Value::String(message));
            //}
        }
        // Process the response body
        // Verify if content length > 1M 
        if content_length.parse::<i32>().unwrap() > 1_000_000 {
            let content = String::from("Response content length exceeds 1M.");
            log_data.as_object_mut().unwrap().insert("error.response.content".to_string(), Value::String(content));
        }
        else {
            // Receive the response body
            let body_state = headers_state.into_body_state().await;

            if body_state.contains_body() {
    
                // Process the body
                let body_content = String::from_utf8(body_state.handler().body()).ok();
                let body_message = if let Ok(json_body) = serde_json::from_str::<Value>(&body_content.clone().unwrap()) {
                    json_body.get("message").and_then(|m| m.as_str()).map(String::from)
                } else {
                    None
                };
            
                // Add body content and message with appropriate field names based on error status
                if let Some(content) = body_content {
                  //  if has_error {
                   //     log_data.as_object_mut().unwrap().insert("error.response.content".to_string(), Value::String(content));
                  //  } else {
                        log_data.as_object_mut().unwrap().insert("http.response.body.content".to_string(), Value::String(content));
                    //}
                }
    
                if let Some(message) = body_message {
                 //   if has_error {
                 //       log_data.as_object_mut().unwrap().insert("error.response.message".to_string(), Value::String(message));
                  //  } else {
                        log_data.as_object_mut().unwrap().insert("http.response.body.message".to_string(), Value::String(message));
                   // }
                }
            }
    
        }
        if has_error {
           // logger::error!("[accessLog] {}", log_data);
        } else {
            logger::info!("[accessLog] {}", log_data);
        }
    }
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration, metadata: Metadata) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;
    
    let filter = on_request(|rs, stream| request_filter(rs, &config, &metadata, stream))
        .on_response(|res, request_data| response_filter(res, &config, request_data));
    
    launcher.launch(filter).await?;
    Ok(())
}