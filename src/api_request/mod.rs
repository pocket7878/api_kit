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

pub trait ApiRequest<ResponseType> {
    fn base_url(&self) -> &str;
    fn method(&self) -> HttpMethod;
    fn path(&self) -> &str;
    fn queryParameters(&self) -> Vec<(&str, &str)>;
    fn interceptRequest<'a>(&self, requestBuilder: RequestBuilder<'a>) -> RequestBuilder<'a> {
        return requestBuilder;
    }
    fn responseFromObject(&self, response: &mut Response) -> Option<ResponseType>;
}