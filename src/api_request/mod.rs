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
use std::error::{Error as StdErr};
use api_client::SessionError;

pub trait ApiRequest<ResponseType, E: StdErr> {
    //Define this function when you want to override base url
    fn base_url(&self) -> Option<&str>;
    fn method(&self) -> HttpMethod;
    fn path(&self) -> &str;
    fn queryParameters(&self) -> Vec<(&str, &str)>;
    fn interceptRequest<'a>(&self, requestBuilder: RequestBuilder<'a>) -> RequestBuilder<'a> {
        return requestBuilder;
    }
    fn responseFromObject(&self, response: &mut Response) -> Result<ResponseType, SessionError<E>>;
}