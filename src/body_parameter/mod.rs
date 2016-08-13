use std::io::Read;
use std::vec::Vec;
use hyper::client::Body;

pub trait BodyParameter {
    fn contentType(&self) -> &str;
    fn build(&self) -> String;
}

mod json;
