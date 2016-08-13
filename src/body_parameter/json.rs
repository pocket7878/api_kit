use serde_json::Value;
use serde_json::ser;
use body_parameter::BodyParameter;
use std::io::{Read, BufReader};
use hyper::client::Body;
use std::convert::Into;

struct JsonBodyParameter {
    pub jsonObject: Value,
}

impl JsonBodyParameter {
    pub fn new(obj: Value) -> JsonBodyParameter {
        return JsonBodyParameter { jsonObject: obj };
    }
}

impl BodyParameter for JsonBodyParameter {
    fn contentType(&self) -> &str {
        return "application/json";
    }

    fn build(&self) -> String {
        let json_str = ser::to_string(&self.jsonObject).unwrap().clone();
        return json_str;
    }
}
