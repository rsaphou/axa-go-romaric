//Working without error
mod generated;
mod utils;
use anyhow::{anyhow, Result};
use pdk::hl::*;
use pdk::logger;
use serde_json::json;
extern crate serde_json;
use serde_json::{Value};
use crate::generated::config::Config;
use pdk::metadata::Metadata;
use chrono::{Local,DateTime, Utc, TimeZone};
use pdk::jwt::JWTClaimsParser;
use pdk::jwt::TokenProvider;
use regex::Regex;
use url::Url;
use crate::utils::vec_u8_to_int;
use crate::utils::get_content_body_and_length;  
use crate::utils::get_content_length;
use crate::utils::HeadersType;
use crate::utils::MAX_BUFFER_SIZE;

// /* Size by default for the Buffer  */
// const MAX_BUFFER_SIZE: usize = 1_000_000; // 1 Mo


async fn request_filter(request_state: RequestState, _config: &Config, metadata: &Metadata, stream: StreamProperties) -> Flow<String> {
    let headers_state = request_state.into_headers_state().await;
    
    let mut client_id: Option<String> = Default::default();
    let mut scope: Option<String> = Default::default();
    let mut audience: Option<String> = Default::default();
    let mut issuer: Option<String> = Default::default();
    let mut expiration: Option<DateTime<Utc>> = None;
    let mut expi_str: String = String::new();
    
    let auth_header = headers_state.handler().header("authorization").unwrap_or_default();
    if !auth_header.is_empty() {
        let token = TokenProvider::bearer(headers_state.handler());
        if token.is_err() {
            
        } else {
            let parsed_claims = JWTClaimsParser::parse(token.unwrap());
            if parsed_claims.is_err() {
              
            } else {
                let claims = parsed_claims.unwrap();
                client_id = claims.get_claim("client_id");
                scope = claims.get_claim("scope");
                audience = claims.get_claim("aud");
                issuer = claims.get_claim("iss");
                expiration = claims.expiration();
                expi_str = match expiration {
                    Some(ex) => ex.to_rfc3339(),
                     None => String::from("N/A"),
                }
            }
        }
    }else {}

    let x_axa_client_id = headers_state.handler().header("x-axa-client-id ").unwrap_or_default();

    let start_time: DateTime<Local> = Local::now();
    let formatted_start_time = start_time.format("%Y-%m-%d %H:%M:%S%.6f").to_string();
    let path = headers_state.path();
    let url = headers_state.authority();
    let method = headers_state.method();
    let proto = headers_state.handler().header("x-forwarded-proto").unwrap_or_default();
    let modified_url = format!("{}://{}", proto, url);
    let request_content_type = headers_state.handler().header("content-type").unwrap_or_default(); 
    let user_agent = headers_state.handler().header("user-agent").unwrap_or_default();
    let mut query_string = String::new();
    if let Some(index) = path.find('?'){
        query_string = path[index + 1..].to_string();
    }
    let mut version: String = String::new();
    let mut label: String = String::new();
    let label_version = metadata.api_metadata.version.clone().unwrap_or_default();
    let re = Regex::new(r"^[a-zA-Z]+-[a-zA-Z]+-[a-zA-Z]+-v\d").unwrap();
    if re.is_match(&label_version) && !label_version.is_empty() {
        let mut parts = label_version.split('-').collect::<Vec<&str>>();
        version = parts.pop().unwrap_or("").to_string();
        label = format!("{}-{}-{}", parts[0], parts[1], parts[2]);
    }else{}
    
	let tls_version = String::from_utf8(stream.read_property(&["connection", "tls_version"]).unwrap_or_default()).unwrap_or_default();
    let trace_id = String::from_utf8(stream.read_property(&["request", "id"]).unwrap_or_default()).unwrap_or_default();
    //let source_port = String::from_utf8(stream.read_property(&["source", "port"]).unwrap_or_default()).unwrap_or_default();
    let source_port =  vec_u8_to_int(stream.read_property(&["source", "port"]).unwrap_or_default());
    let destination_port = String::from_utf8(stream.read_property(&["destination", "port"]).unwrap_or_default()).unwrap_or_default();
    let request_header = String::from_utf8(stream.read_property(&["request", "headers"]).unwrap_or_default()).unwrap_or_default();
	
    // Compute the content-length from headers
    let mut content_length = headers_state.handler().header("Content-Length").unwrap_or_default();
     if content_length.is_empty() {
        // if Content-lenght header is empty, we process as stream and count each chunk size
        content_length = get_content_length(HeadersType::RequestHeaders(headers_state)).await;
        // End of compting body size
    }
    else {
        logger::debug!("Request payload size from header Content-Length: {} kb ", content_length);
    }   

    let request_data_value = json!({ 
        "http.request.referrer": modified_url,
        "client.user.id": client_id,
        "url.query": query_string,
        "api.label": label,
        "event.start": formatted_start_time,
        "client.user.jwt.scope": scope,
        "client.user.jwt.audience": audience,
        "client.user.jwt.issuer": issuer,
        "client.user.jwt.expiration": expi_str,
		"tls.version": tls_version,
        "traceId": trace_id,
        "source_port":source_port,
        "destination_port": destination_port,
        "http.request.body.bytes":content_length,
        "x-axa-client-id": x_axa_client_id
});
    Flow::Continue(request_data_value.to_string())
}


async fn response_filter(response_state: ResponseState, _config: &Config, request_data: RequestData<String>, metadata: &Metadata, stream: StreamProperties) {
    
    let end_time: DateTime<Local> = Local::now();
    let formatted_end_time = end_time.format("%Y-%m-%d %H:%M:%S%.6f").to_string();
    let headers_state = response_state.into_headers_state().await;
    let response_content_type = headers_state.handler().header("content-type").unwrap_or_default();
    let content_length = headers_state.handler().header("Content-Length").unwrap_or_default();
    let response_status = headers_state.status_code();
    let response_time = headers_state.handler().header("x-envoy-upstream-service-time").unwrap_or_default();
    let duration_prop = stream.read_property(&["response", "backend_latency"]).unwrap_or_default();
    let v_destinaltion_port =  vec_u8_to_int(stream.read_property(&["destination", "port"]).unwrap_or_default());

    let mut level_val: &str = "INFO";
    let mut response_error = String::new();
    let mut body_bytes = String::new();
    let mut body_content = String::new();

    if !response_status.to_string().starts_with("20") {
        response_error = response_status.to_string();
        level_val = "ERROR";
        if content_length.is_empty() {
            //if Content-lenght header is empty, we process as stream and count each chunk size
            let (bytes, content) = get_content_body_and_length(HeadersType::ResponseHeaders(headers_state)).await;
            body_bytes = bytes;
            body_content = content;
            logger::debug!("Response payload size streamed: {} kb , content {}", body_bytes, body_content);
            // End of compting body size    
        }
            else {
                body_bytes = content_length;
                let size = body_bytes.parse::<usize>().unwrap_or(usize::MAX);
                if size < MAX_BUFFER_SIZE{
                    let response_body_state = headers_state.into_body_state().await;
                    body_content = String::from_utf8_lossy(&response_body_state.handler().body()).to_string();
                } else {
                    // If content-length is greater than MAX_BUFFER_SIZE, we do nothing, by default the buffer is set to 1 MB
                    body_content = String::from("Body content too large to be captured");
                }
                logger::debug!("Response payload size from header Content-Length: {} kb ", body_bytes);
            }   
    }else{
        level_val = "INFO";
        if content_length.is_empty() {
            //if Content-lenght header is empty, we process as stream and count each chunk size
            body_bytes = get_content_length(HeadersType::ResponseHeaders(headers_state)).await;
            logger::debug!("Response payload size streamed: {} kb ", body_bytes);
            // End of compting body size    
        }
            else {
                body_bytes = content_length;
                logger::debug!("Response payload size from header Content-Length: {} kb ", body_bytes);
            }   
        }

    if let RequestData::Continue(request_data_value) = request_data {
      let log_data;
      if !response_status.to_string().starts_with("20"){
        level_val = "ERROR";
        log_data = json!({
            "http.response.mime_type": response_content_type,
            "http.response.status_code": response_status,
            "http.response.body.bytes": body_bytes,
            "event.duration_upstream":response_time,
            "source_port" : v_destinaltion_port,
            "http.response.body.content":body_content
         });
      }else{
        level_val = "INFO";
        log_data = json!({
            "http.response.mime_type": response_content_type,
            "http.response.status_code": response_status,
            "event.end": formatted_end_time,
            "event.duration_upstream":response_time,
            "source_port" : v_destinaltion_port,
            "http.response.body.bytes": body_bytes
         });
      }
    let request_data_string: Value = serde_json::from_str(&request_data_value).unwrap();
    let mut combined_json = request_data_string.as_object().unwrap().clone();

    for (key, value) in log_data.as_object().unwrap().iter() {
        combined_json.insert(key.clone(), value.clone());
    }
    let combined_json_value: Value = Value::Object(combined_json);

    if level_val == "INFO"
    { logger::info!("[accessLog] {}",combined_json_value); }
    else
    { logger::error!("[accessLog] {}",combined_json_value); }
    } else {
        return;
    };
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
    .on_response(|res, request_data, stream| {
        response_filter(res, &config, request_data, &metadata, stream) 
    });
    launcher.launch(filter).await?;
    Ok(())
}