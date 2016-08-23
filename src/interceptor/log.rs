use std::rc::Rc;
use std::cell::RefCell;
use interceptor::{Interceptor, InterceptorChain};
use hyper::client::Response;
use error::ApiError;

pub struct LogInterceptor {
}

impl Interceptor for LogInterceptor {
    fn intercept(&self, chain: InterceptorChain) -> Result<Rc<RefCell<Response>>, ApiError> {
        let api_request = chain.request();
        println!("Uri: {}{}", api_request.base_url, api_request.path);
        return chain.proceed(api_request);
    }
}
