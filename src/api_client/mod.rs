use hyper::client::Client;
use hyper::client::response::Response;
use hyper::client::RequestBuilder;
use hyper::client::request::Request;
use hyper::Url;
use api_request::HttpMethod;
use api_request::{ApiRequest, ApiRequestBuilder};
use interceptor::{Interceptor, InterceptorChain, SendRequestInterceptor};
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
use std::rc::Rc;

pub struct RealCall<ResponseType: 'static> {
    api_request: Rc<ApiRequest>,
    api_request_builder: Rc<ApiRequestBuilder<ResponseType>>,
    pub interceptors: Vec<Box<Interceptor>>,
}

impl<ResponseType> RealCall<ResponseType> {
    pub fn addInterceptor(mut self, interceptor: Box<Interceptor>) -> RealCall<ResponseType> {
        self.interceptors.push(interceptor);
        return self
    }

    pub fn send(mut self) -> Result<ResponseType, ApiError> {
        self.interceptors.push(Box::new(SendRequestInterceptor {
            request_builder: self.api_request_builder.clone()
        }));
        let chain = InterceptorChain::new(
            self.api_request.clone(), 
            self.interceptors);
        let result = chain.proceed(self.api_request);
        return match result {
            Ok(mut response) => {
                self.api_request_builder.responseFromObject(response)
            },
            Err(apiErr) => Err(apiErr)
        }
    }
}

pub trait ApiClient {
    fn base_url(&self) -> String;
    fn call<ResponseType>(&self, request_builder: Rc<ApiRequestBuilder<ResponseType>>) -> RealCall<ResponseType> {
        let api_request = self.build_api_request(request_builder.clone());
        return RealCall {
            api_request: Rc::new(api_request),
            api_request_builder: request_builder.clone(),
            interceptors: Vec::new()
        }
    }
    fn build_api_request<ResponseType>(&self, request_builder: Rc<ApiRequestBuilder<ResponseType>>) -> ApiRequest {
        let base_url = match request_builder.base_url() {
            Some(url) => url,
            None => self.base_url()
        };
        return ApiRequest {
            base_url: base_url,
            method: request_builder.method(),
            path: request_builder.path(),
            headers: None,
            queryParameters: request_builder.queryParameters(),
            body: request_builder.requestBody()
        }
    }
}
