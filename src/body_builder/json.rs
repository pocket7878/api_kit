use serde::ser::Serialize;
use serde_json::ser;
use serde_json::value::Value;
use body_builder::{BodyBuilder, RequestBody};
use std::io::{Read, BufReader};
use hyper::client::{Body, RequestBuilder};
use std::convert::Into;

pub struct JSONBodyBuilder {
    pub jsonObject: Value,
}

impl JSONBodyBuilder {
    pub fn new(obj: Value) -> JSONBodyBuilder {
        return JSONBodyBuilder { jsonObject: obj };
    }
}

impl BodyBuilder for JSONBodyBuilder {
    fn build(&self) -> RequestBody {
        let json_str: String = ser::to_string(&self.jsonObject).unwrap();
        let body = RequestBody {
            content_type: "application/json",
            body: json_str,
        };
        return body;
    }
}
