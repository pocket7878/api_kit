pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE    
}

use hyper::client::RequestBuilder;
use hyper::net::Fresh;
use hyper::client::response::Response;
use hyper::status::StatusCode;
use std::error::{Error as StdErr};
use err::Error;
use error::ApiError;
use std::fmt;

pub trait ApiRequest<ResponseType> {
    //Define this function when you want to override base url
    fn base_url(&self) -> Option<&str> {
        return None
    }
    fn method(&self) -> HttpMethod;
    fn path(&self) -> &str;
    fn queryParameters(&self) -> Vec<(&str, &str)>;
    fn interceptRequest<'a>(&'a self, requestBuilder: RequestBuilder<'a>) -> Result<RequestBuilder<'a>, ApiError> {
        return Ok(requestBuilder);
    }
    fn interceptResponse<'a>(&'a self, response: &'a mut Response) -> Result<&'a mut Response, ApiError> {
        return Ok(response);
    }
    fn responseFromObject(&self, response: &mut Response) -> Result<ResponseType, ApiError>;
}
