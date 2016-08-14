use serde::ser::Serialize;
use serde_json::ser;
use serde_json::value::Value;
use body_builder::{BodyBuilder, RequestBody};
use std::io::{Read, BufReader};
use hyper::client::{Body, RequestBuilder};
use std::convert::Into;
use std::string::ToString;
use url::form_urlencoded;

pub struct FormBodyBuilder {
    pub data: Vec<(String, Box<ToString>)>
}

impl FormBodyBuilder {
    pub fn new(obj: Vec<(String, Box<ToString>)>) -> FormBodyBuilder {
        return FormBodyBuilder { data: obj };
    }
}

impl BodyBuilder for FormBodyBuilder {
    fn build(&self) -> RequestBody {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        for datum in &self.data {
            encoder.append_pair(&*(datum.0), &*(datum.1.to_string()));
        }
        let result = encoder.finish();
        return RequestBody {
            content_type: "application/x-www-form-urlencoded",
            body: result
        }
    }
}
