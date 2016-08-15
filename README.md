# api_kit

Rust library for build typesafe web api client.

## Sample Usage

```
extern crate api_kit;
extern crate hyper;
extern crate serde_json; 

use hyper::client::response::Response;
use api_kit::api_request::ApiRequest;
use api_kit::api_request::HttpMethod;
use api_kit::api_client::ApiClient;
use api_kit::error::ApiError;
use hyper::header::{Headers, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::client::request::Request;
use hyper::net::Fresh;
use std::io::{Read};

struct CircleCi {
}

impl ApiClient for CircleCi {
    fn base_url(&self) -> &str {
        return "https://circleci.com/api/v1.1";
    }
}

struct CircleCiMeRequest<'a> {
    api_token: &'a str
}

impl<'a> CircleCiMeRequest<'a> {
    fn new(token: &'a str) -> CircleCiMeRequest {
        CircleCiMeRequest {
            api_token: token
        }
    }
}

impl<'a> ApiRequest<serde_json::Value> for CircleCiMeRequest<'a> {
    
    fn method(&self) -> HttpMethod {
        return HttpMethod::Get;
    }
    
    fn path(&self) -> &str {
        return "/me";
    }
    
    fn queryParameters(&self) -> Vec<(&str, &str)> {
        return vec![
            ("circle-token", self.api_token)
        ];
    }
    
    fn interceptRequest(&self, mut request: Request<Fresh>) -> Result<Request<Fresh>, ApiError> {
        request.headers_mut().set(
                    Accept(vec![
                        qitem(Mime(TopLevel::Application, SubLevel::Json,
                                   vec![(Attr::Charset, Value::Utf8)])),
                    ])
                );
        return Ok(request);
    }
    
    fn responseFromObject(&self, response: &mut Response) -> Result<serde_json::Value, ApiError> {
        let mut buffer = String::new();
        response.read_to_string(&mut buffer).unwrap();
        return Ok(serde_json::from_str(&buffer).unwrap());
    }
}

fn main() {
    let ci = CircleCi {} ;
    let me = ci.sendRequest(&CircleCiMeRequest::new("xxxxxxxxxxxxxxxxx"));
    println!("{}", me.unwrap());
}
```

## TODO

- [ ] Support Mutlipart/Form
- [ ] Support Network Interceptor (retry automatic-authenticate..etc)
