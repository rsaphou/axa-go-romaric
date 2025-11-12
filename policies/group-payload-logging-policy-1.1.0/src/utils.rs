//Working without error
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


pub enum HeadersType {
    RequestHeaders(RequestHeadersState),
    ResponseHeaders(ResponseHeadersState),
}
/* Size by default for the Buffer  */
pub const MAX_BUFFER_SIZE: usize = 1000_000; // 1 Mo


pub fn vec_u8_to_int(vec: Vec<u8>) -> i64 {
    let mut num : i64 = 0;
    let mut base : i64 = 1;
    if vec.len() > 0 {
        for i in 0..(vec.len()-1)
        {
            num = num + (base * (vec[i] as i64));
            base = base * 256;
        }
    }else{};
    num
}


/* This function retrieves the body content and its length, limiting the content to 1 MB
    It takes as input HeadersType enum to handle both request and response headers
    It returns a tuple of (body length as String, body content as String) */
// pub async fn get_content_body_and_length(headers_state: HeadersType) -> (String, String) {
//     let mut total_size = 0usize;
//     let mut body_content = String::new();
//     let mut buffer = Vec::new();

//      match headers_state {
//         HeadersType::RequestHeaders(state) => {
//             let body_stream_state = state.into_body_stream_state().await;
//             let mut body_as_stream = body_stream_state.stream();
//             logger::debug!("Body stream processing for content and length...");
//             while let Some(chunk) = body_as_stream.next().await {
//                 let mut chunk_bytes = chunk.into_bytes();
//                 let  chunk_size = chunk_bytes.len();
//                 logger::debug!("Chunk size: {}", chunk_size.to_string());
              
//                 total_size += chunk_size;
//                 if total_size < MAX_BUFFER_SIZE
//                 {
//                         //let remaining_space = MAX_BUFFER_SIZE - body_content.len();
//                         //if chunk_size <= remaining_space {
//                             //body_content.push_str(&String::from_utf8_lossy(&chunk_bytes));
//                             //buffer.append(&mut chunk_bytes);
//                             //buffer.extend_from_slice(&chunk_bytes);
//                             buffer.extend_from_slice(&chunk_bytes);
//                             logger::debug!("Chunk added to body content, current size: {}", buffer.len());
//                         // else {
//                             // Take the remain to reach 1 Mo
//                             //body_content.push_str(&String::from_utf8_lossy(&chunk_bytes[..remaining_space]));
//                           //  logger::debug!("Body content truncated at 1 MB, but continuing to count total size {} ", total_size.to_string());
//                        // }
//                     }
//                 else {
//                     logger::debug!("Body content truncated at 1 MB");
//                     break;
//                     }
//                 }
//         }
//         HeadersType::ResponseHeaders(state) => {
//             let body_stream_state = state.into_body_stream_state().await;
//             let mut body_as_stream = body_stream_state.stream();
//             logger::debug!("Body stream processing for content and length...");
//             while let Some(chunk) = body_as_stream.next().await {
//                 let chunk_bytes = chunk.into_bytes();
//                 let chunk_size = chunk_bytes.len();
//                 total_size += chunk_size;
//                 if body_content.len() < MAX_BUFFER_SIZE
//                 {
//                     let remaining_space = MAX_BUFFER_SIZE - body_content.len();
//                         if chunk_size <= remaining_space {
//                             body_content.push_str(&String::from_utf8_lossy(&chunk_bytes));
//                         } else {
//                             // Take the remain to reach 1 Mo
//                             body_content.push_str(&String::from_utf8_lossy(&chunk_bytes[..remaining_space]));
//                             logger::debug!("Body content truncated at 1 MB, but continuing to count total size");
//                         }
//                     }
//             }
//         }
//     }
//     body_content = String::from_utf8_lossy(&buffer).to_string();
//     buffer.clear();
//     logger::debug!("Fin streaming .....");
//     //logger::debug!("Payload size: {} kb, content {}", total_size as f32 / 1000.0, body_content);
//     (total_size.to_string(), body_content)
// }
pub async fn get_content_body_and_length(headers_state: HeadersType) -> (String, String) {
    let mut total_size = 0usize;
    let mut buffer = Vec::new();

    match headers_state {
        HeadersType::RequestHeaders(state) => {
            
            state.handler().remove_header("content-lenght");
            let body_stream_state = state.into_body_stream_state().await;
            let mut body_as_stream = body_stream_state.stream();
            
            //headers_handler.remove_header("content-length");
            logger::debug!("Body stream processing for content and length...");
            
            while let Some(chunk) = body_as_stream.next().await {
                 logger::debug!("....Reading chunk...");
                let chunk_bytes = chunk.into_bytes();
                let chunk_size = chunk_bytes.len();
                total_size += chunk_size;
                
                if buffer.len() < MAX_BUFFER_SIZE {
                    logger::debug!("....Adding chunk to buffer... {}", chunk_size);
                    // Calculate remaining space in buffer
                    let remaining_space = MAX_BUFFER_SIZE - buffer.len();
                    logger::debug!("Remaining space in buffer: {}", remaining_space);
                    let bytes_to_add = chunk_size.min(remaining_space);
                    logger::debug!("Bytes to add from chunk: {}", bytes_to_add);
                    // Only add what fits
                    buffer.extend_from_slice(&chunk_bytes[..bytes_to_add]);
                    logger::debug!("Chunk added to body content, current size: {}", buffer.len());
                    
                    // Check if buffer is now full
                    if buffer.len() >= MAX_BUFFER_SIZE {
                        logger::debug!("Body content truncated at 1 MB");
                        break;
                        // Continue reading to get total_size, but don't buffer
                    }
                }
            }
        }
    HeadersType::ResponseHeaders(state) => {
            let body_stream_state = state.into_body_stream_state().await;
            let mut body_as_stream = body_stream_state.stream();
            logger::debug!("Body stream processing for content and length...");
            
             while let Some(chunk) = body_as_stream.next().await {
                 logger::debug!("....Reading chunk...");
                let chunk_bytes = chunk.into_bytes();
                let chunk_size = chunk_bytes.len();
                total_size += chunk_size;
                
                if buffer.len() < MAX_BUFFER_SIZE {
                    logger::debug!("....Adding chunk to buffer... {}", chunk_size);
                    // Calculate remaining space in buffer
                    let remaining_space = MAX_BUFFER_SIZE - buffer.len();
                    logger::debug!("Remaining space in buffer: {}", remaining_space);
                    let bytes_to_add = chunk_size.min(remaining_space);
                    logger::debug!("Bytes to add from chunk: {}", bytes_to_add);
                    // Only add what fits
                    buffer.extend_from_slice(&chunk_bytes[..bytes_to_add]);
                    logger::debug!("Chunk added to body content, current size: {}", buffer.len());
                    
                    // Check if buffer is now full
                    if buffer.len() >= MAX_BUFFER_SIZE {
                        logger::debug!("Body content truncated at 1 MB");
                        break;
                        // Continue reading to get total_size, but don't buffer
                    }
                }
            }
        }
    }
    
    let body_content = String::from_utf8_lossy(&buffer).to_string();
    buffer.clear();
    logger::debug!("Streaming finished. Total size: {}, Buffered: {}", total_size, buffer.len());
    
    (total_size.to_string(), body_content)
}