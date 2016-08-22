use hyper::client::response::Response;
use api_request::{ApiRequest, ApiRequestBuilder};
use std::rc::Rc;
use hyper::client::{Client, Body};
use hyper::client::RequestBuilder;
use hyper::client::request::Request;
use hyper::Url;
use hyper::net::{Fresh, Streaming};
use api_client::ApiClient;
use api_request::HttpMethod;
use body_builder::{BodyBuilder, RequestBody};
use std::error::Error as StdErr;
use hyper::error::Error as HyErr;
use std::result::Result;
use hyper::status::StatusCode;
use error::{ApiError, ErrorTiming, ResponseError};
use std::boxed::Box;
use std::vec::Vec;
use std::io::Read;
use hyper::header::{ContentLength, ContentType, Headers};
use std::io::{Write, Error as StdioErr};
use std::cell::RefCell;

pub mod log;

pub trait Interceptor {
    fn intercept(&self, chain: InterceptorChain) -> Result<Rc<RefCell<Response>>, ApiError>;
}

pub struct InterceptorChain {
    interceptors: Rc<Vec<Box<Interceptor>>>,
    index: usize,
    request: Rc<ApiRequest>,
}

impl InterceptorChain {
    pub fn new(request: Rc<ApiRequest>, interceptors: Vec<Box<Interceptor>>) -> InterceptorChain {
        return InterceptorChain {
            interceptors: Rc::new(interceptors),
            index: 0,
            request: request,
        };
    }

    pub fn request(&self) -> Rc<ApiRequest> {
        return self.request.clone();
    }

    pub fn proceed(&self, request: Rc<ApiRequest>) -> Result<Rc<RefCell<Response>>, ApiError> {
        let next = InterceptorChain {
            interceptors: self.interceptors.clone(),
            index: &self.index + 1,
            request: request,
        };

        let ref interceptor = self.interceptors[self.index];

        let result: Result<Rc<RefCell<Response>>, ApiError> = interceptor.intercept(next);

        return result;
    }
}

pub struct SendRequestInterceptor<ResponseType: 'static> {
    pub request_builder: Rc<ApiRequestBuilder<ResponseType>>,
}

impl<ResponseType> Interceptor for SendRequestInterceptor<ResponseType> {
    fn intercept(&self, chain: InterceptorChain) -> Result<Rc<RefCell<Response>>, ApiError> {
        let hyclient = Client::new();
        let api_request = chain.request();
        let base_url = api_request.base_url.clone();
        let mut requestUri: Url = Url::parse(&format!("{}{}", base_url, api_request.path)).unwrap();
        {
            let mut querySerializer = requestUri.query_pairs_mut();
            for queryPair in &api_request.queryParameters {
                querySerializer.append_pair(&queryPair.0, &queryPair.1);
            }
        }
        let mut req = match Request::new(api_request.method.clone(), requestUri) {
            Ok(req) => req,
            Err(err) => {
                let apiError = ApiError::new::<HyErr>(err, ErrorTiming::AtRequest);
                return Err(apiError);
            }
        };

        let body = api_request.body.clone();
        match body {
            Some(bd) => {
                req.headers_mut().set(ContentType(bd.content_type.parse().unwrap()));
                req.headers_mut().set(ContentLength(bd.body.len() as u64));
            }
            None => {
                req.headers_mut().set(ContentLength(0));
            }
        };

        let mut req_started: Request<Streaming> = match self.request_builder
            .interceptRequest(req) {
            Ok(interceptedRequest) => {
                match interceptedRequest.start() {
                    Ok(started_request) => started_request,
                    Err(err) => {
                        let apiError = ApiError::new::<HyErr>(err, ErrorTiming::AtRequest);
                        return Err(apiError);
                    }
                }
            }
            Err(err) => return Err(err),
        };

        match api_request.body.clone() {
            Some(body) => {
                match req_started.write(body.body.as_bytes()) {
                    Ok(_) => (),
                    Err(err) => {
                        let apiError = ApiError::new::<StdioErr>(err, ErrorTiming::AtRequest);
                        return Err(apiError);
                    }
                }
            }
            None => (),
        }

        let mut result = match req_started.send() {
            Ok(resp) => resp,
            Err(err) => {
                let apiError = ApiError::new::<HyErr>(err, ErrorTiming::AtNetwork);
                return Err(apiError);
            }
        };

        match self.request_builder.interceptResponse(Rc::new(RefCell::new(result))) {
            Ok(interceptedResponse) => Ok(interceptedResponse),
            Err(err) => Err(err),
        }
    }
}
