pub struct RequestBody {
    pub content_type: &'static str,
    pub body: String
}



pub trait BodyBuilder {
    fn build(&self) -> RequestBody;
}

pub mod json;
