use std::io::{Read};

pub enum RequestBodyEntity<'a> {
    Data(Vec<u8>),
    InputStream(&'a Read)
}

pub trait BodyParameter {
    fn contentType(&self) -> &str;
    fn buildEntity(&self) -> RequestBodyEntity;
}

mod json;