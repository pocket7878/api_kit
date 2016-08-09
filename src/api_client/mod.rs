use hyper::client::Client;
use hyper::Url;
use api_request::HttpMethod;
use api_request::ApiRequest;
use std::error::{Error as StdErr};
use hyper::error::{Error as HyErr};

pub enum SessionError<E: StdErr> {
    NetworkError(HyErr),
    ResponseError(E)
}

pub trait ApiClient {
    fn base_url(&self) -> &str;
    fn sendRequest<ResponseType, E: StdErr>(&self, request: &ApiRequest<ResponseType, E>) -> Result<ResponseType, SessionError<E>> {
        let client = Client::new();
        let base_url = match request.base_url() {
            Some(url) => url,
            None => self.base_url()
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
            HttpMethod::DELETE => client.patch(requestUri)
        };    
        let interceptedRequest = request.interceptRequest(hyperRequest);
        let mut result = interceptedRequest.send();
        let responseObject = match result {
            Ok(mut resp) => request.responseFromObject(&mut resp),
            Err(err) => Err(SessionError::NetworkError(err))
        };
        return responseObject;
    }
}