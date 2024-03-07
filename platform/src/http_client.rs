#![allow(unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::clone_on_copy)]

use roc_std::{RocList, RocStr};
use std::{iter::FromIterator, time::Duration};
use crate::roc_app::{InternalRequest, InternalResponse, InternalHeader, InternalMethod};

pub fn send_req(roc_request: &InternalRequest) -> InternalResponse {
    let mut builder = reqwest::blocking::ClientBuilder::new();

    if roc_request.timeout.is_TimeoutMilliseconds() {
        let ms: u64 = roc_request.timeout.clone().unwrap_TimeoutMilliseconds();
        builder = builder.timeout(Duration::from_millis(ms));
    }

    let client = match builder.build() {
        Ok(c) => c,
        Err(_) => {
            return InternalResponse {
                status: 500,
                body: RocList::empty(),
                headers: RocList::empty(),
            }; // TLS backend cannot be initialized
        }
    };

    let method = {
        use reqwest::Method;
        use crate::roc_app::InternalMethod::*;

        match roc_request.method {
            Connect => Method::CONNECT,
            Delete => Method::DELETE,
            Get => Method::GET,
            Head => Method::HEAD,
            Options => Method::OPTIONS,
            Patch => Method::PATCH,
            Post => Method::POST,
            Put => Method::PUT,
            Trace => Method::TRACE,
        }
    };

    let url = roc_request.url.as_str();

    let mut req_builder = client.request(method, url);

    for header in roc_request.headers.iter() {
        req_builder = req_builder.header(header.name.as_str(), header.value.as_slice());
    }

    let mime_type_str = roc_request.mimeType.as_str();
    let bytes = roc_request.body.as_slice().to_vec();

    req_builder = req_builder.header("Content-Type", mime_type_str);
    req_builder = req_builder.body(bytes);

    let request = match req_builder.build() {
        Ok(req) => req,
        Err(err) => {
            return InternalResponse {
                status: 400,
                body: RocList::from_slice(err.to_string().as_bytes()),
                headers: RocList::empty(),
            }; // Bad Request 400
        }
    };

    match client.execute(request) {
        Ok(response) => {
            let headers_iter = response
                .headers()
                .iter()
                .map(|(name, value)| InternalHeader {
                    name: RocStr::from(name.as_str()),
                    value: RocList::from(value.as_bytes()),
                });

            let headers = RocList::from_iter(headers_iter);

            let status = response.status().as_u16();
            let bytes = response.bytes().unwrap_or_default();
            let body: RocList<u8> = RocList::from_iter(bytes.into_iter());

            InternalResponse {
                status,
                body,
                headers,
            }
        }

        Err(err) => {
            if err.is_timeout() {
                InternalResponse {
                    status: 408, // 408 Request Timeout
                    body: RocList::from_slice(err.to_string().as_bytes()),
                    headers: RocList::empty(),
                }
            } else if err.is_request() {
                InternalResponse {
                    status: 400, // 400 Bad Request
                    body: RocList::from_slice(err.to_string().as_bytes()),
                    headers: RocList::empty(),
                }
            } else {
                // TODO handle more errors
                InternalResponse {
                    status: 404, // 404 Not Found
                    body: RocList::from_slice(err.to_string().as_bytes()),
                    headers: RocList::empty(),
                }
            }
        }
    }
}
