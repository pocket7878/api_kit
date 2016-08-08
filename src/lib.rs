extern crate serde;
extern crate serde_json;
extern crate hyper;

pub mod api_request;
pub mod body_parameter;

use hyper::client;
use hyper::Url;
use api_request::ApiRequest;
use api_request::HttpMethod;


pub fn sendRequest<ResponseType>(request: &ApiRequest<ResponseType>) -> Option<ResponseType> {
    let client = client::Client::new();
    let mut requestUri: Url = Url::parse(&format!("{}{}", request.base_url(), request.path())).unwrap();
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
        Err(_) => None
    };
    return responseObject;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
