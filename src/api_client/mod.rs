use hyper::client::Client;
use hyper::client::response::Response;
use hyper::client::RequestBuilder;
use hyper::Url;
use api_request::HttpMethod;
use api_request::ApiRequest;
use body_parameter::BodyParameter;
use std::error::Error as StdErr;
use hyper::error::Error as HyErr;
use std::result::Result;
use hyper::status::StatusCode;
use error::{ApiError, ErrorTiming, ResponseError};
use std::boxed::Box;
use std::vec::Vec;
use hyper::client::Body;
use std::io::Read;

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
        let mut hyperRequest = match request.method() {
            HttpMethod::GET => client.get(requestUri),
            HttpMethod::POST => client.post(requestUri),
            HttpMethod::PUT => client.put(requestUri),
            HttpMethod::PATCH => client.patch(requestUri),
            HttpMethod::DELETE => client.patch(requestUri),
        };
        let mut interceptedRequest = request.interceptRequest(hyperRequest);
        let result = match interceptedRequest {
            Ok(req) => Ok(req.send()),
            Err(err) => Err(err),
        };
        return match result {
            Ok(resp) => {
                let result = match resp {
                    Ok(mut hyresp) => {
                        match request.interceptResponse(&mut hyresp) {
                            Ok(mut interceptedResponse) => {
                                request.responseFromObject(&mut interceptedResponse)
                            }
                            Err(err) => Err(err),
                        }
                    }
                    Err(err) => {
                        let apiError = ApiError::new::<HyErr>(err, ErrorTiming::AtNetwork);
                        Err(apiError)
                    }
                };
                result
            }
            // Error on interceptRequest
            Err(err) => Err(err),
        };
    }
}
