use hyper::client::request::Request;
use hyper::net::Fresh;
use hyper::client::response::Response;
use hyper::status::StatusCode;
use std::error::Error as StdErr;
use err::Error;
use error::ApiError;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use hyper::header::Headers;
use body_builder::{BodyBuilder, RequestBody};

pub use hyper::method::Method as HttpMethod;

pub struct ApiRequest {
    pub base_url: String,
    pub method: HttpMethod,
    pub path: String,
    pub headers: Option<Headers>,
    pub queryParameters: Vec<(String, String)>,
    pub body: Option<RequestBody>,
}

pub trait ApiRequestBuilder<ResponseType> {
    // Define this function when you want to override base url
    fn base_url(&self) -> Option<String> {
        return None;
    }
    fn method(&self) -> HttpMethod;
    fn path(&self) -> String;
    fn queryParameters(&self) -> Vec<(String, String)> {
        let vc: Vec<(String, String)> = vec![];
        return vc;
    }
    fn requestBody(&self) -> Option<RequestBody> {
        return None;
    }
    fn interceptRequest(&self, request: Request<Fresh>) -> Result<Request<Fresh>, ApiError> {
        return Ok(request);
    }
    fn interceptResponse(&self,
                             response: Rc<RefCell<Response>>)
                             -> Result<Rc<RefCell<Response>>, ApiError> {
        return Ok(response);
    }
    fn responseFromObject(&self, response: Rc<RefCell<Response>>) -> Result<ResponseType, ApiError>;

}
