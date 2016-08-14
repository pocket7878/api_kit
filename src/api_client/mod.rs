use hyper::client::Client;
use hyper::client::response::Response;
use hyper::client::RequestBuilder;
use hyper::client::request::Request;
use hyper::Url;
use api_request::HttpMethod;
use api_request::ApiRequest;
use body_builder::{BodyBuilder, RequestBody};
use std::error::Error as StdErr;
use hyper::error::Error as HyErr;
use std::result::Result;
use hyper::status::StatusCode;
use error::{ApiError, ErrorTiming, ResponseError};
use std::boxed::Box;
use std::vec::Vec;
use hyper::client::Body;
use std::io::Read;
use hyper::header::ContentLength;
use hyper::header::ContentType;
use hyper::header::Headers;
use std::io::{Write, Error as StdioErr};

pub trait ApiClient {
    fn base_url(&self) -> &str;
    fn sendRequest<ResponseType>(&self,
                                 request: &ApiRequest<ResponseType>)
                                 -> Result<ResponseType, ApiError> {
        let client = Client::new();
        let base_url = match request.base_url() {
            Some(url) => url,
            None => self.base_url(),
        };
        let mut requestUri: Url = Url::parse(&format!("{}{}", base_url, request.path())).unwrap();
        {
            let mut querySerializer = requestUri.query_pairs_mut();
            for queryPair in &request.queryParameters() {
                querySerializer.append_pair(queryPair.0, queryPair.1);
            }
        }
        let mut req = match Request::new(request.method(), requestUri) {
            Ok(req) => req,
            Err(err) => {
                let apiError = ApiError::new::<HyErr>(err, ErrorTiming::AtRequest);
                return Err(apiError)
            }
        };

        let body = request.requestBody();
        match body {
            Some(bd) => {
                req.headers_mut().set(ContentType(bd.content_type.parse().unwrap()));
                req.headers_mut().set(ContentLength(bd.body.len() as u64));
            },
            None => {
                req.headers_mut().set(ContentLength(0));
            }
        };

        let mut req_started = match request.interceptRequest(req)  {
            Ok(req) => {
                match req.start() {
                    Ok(req) => req,
                    Err(err) => {
                        let apiError = ApiError::new::<HyErr>(err, ErrorTiming::AtRequest);
                        return Err(apiError);
                    }
                }
            }
            Err(err) => return Err(err)
        };

        match request.requestBody() {
            Some(body) => {
                match req_started.write(body.body.as_bytes()) {
                    Ok(_) => (),
                    Err(err) => {
                        let apiError = ApiError::new::<StdioErr>(err, ErrorTiming::AtRequest);
                        return Err(apiError);
                    }
                }
            },
            None => ()
        }

        let mut result = match req_started.send() {
            Ok(resp) => resp,
            Err(err) => {
                let apiError = ApiError::new::<HyErr>(err, ErrorTiming::AtNetwork);
                return Err(apiError);
            }
        };

        match request.interceptResponse(&mut result) {
            Ok(mut interceptedResponse) => {
                request.responseFromObject(&mut interceptedResponse)
            }
            Err(err) => Err(err),
        }
    }
}
