use serde_json::Value;
use serde_json::ser;
use body_builder::BodyBuilder;
use std::io::{Read, BufReader};
use hyper::client::{Body, RequestBuilder};
use std::convert::Into;

struct JSONBodyBuilder {
    pub jsonObject: Value,
}

impl JSONBodyBuilder {
    pub fn new(obj: Value) -> JSONBodyBuilder {
        return JSONBodyBuilder { jsonObject: obj };
    }
}

impl BodyBuilder for JSONBodyBuilder {
    fn contentType(&self) -> &str {
        return "application/json";
    }

    fn build(&self) -> String {
        let json_str: String = ser::to_string(&self.jsonObject).unwrap();
        return json_str;
    }
}
