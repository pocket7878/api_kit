pub trait BodyBuilder {
    fn contentType(&self) -> &str;
    fn build(&self) -> String;
}

mod json;
