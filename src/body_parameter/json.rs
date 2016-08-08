use serde_json::Value;
use serde_json::ser;
use body_parameter::RequestBodyEntity;

struct JsonBodyParameter {
    pub jsonObject: Value
}

impl JsonBodyParameter {
    pub fn new(obj: Value) -> JsonBodyParameter {
        return JsonBodyParameter { jsonObject: obj };
    }
    
    pub fn contentType(&self) -> &str {
        return "application/json";
    }
    
    pub fn buildEntity(&self) -> RequestBodyEntity {
        return RequestBodyEntity::Data(ser::to_vec(&self.jsonObject).unwrap());
    }
}