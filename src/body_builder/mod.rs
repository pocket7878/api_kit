use std::marker::Copy;

pub struct RequestBody {
    pub content_type: String,
    pub body: String,
}

impl Clone for RequestBody {
    fn clone(&self) -> RequestBody { 
        return RequestBody {
            content_type: self.content_type.clone(),
            body: self.body.clone()
        }
    }
}

pub trait BodyBuilder {
    fn build(&self) -> RequestBody;
}

pub mod json;
pub mod form;
