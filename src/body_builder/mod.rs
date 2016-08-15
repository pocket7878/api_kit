pub struct RequestBody {
    pub content_type: String,
    pub body: String,
}



pub trait BodyBuilder {
    fn build(&self) -> RequestBody;
}

pub mod json;
pub mod form;
