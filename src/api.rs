use rand::{Rng, thread_rng};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct RequestParams<T> {
    service: String,
    method: String,
    args: T,
}

#[derive(Serialize, Debug)]
pub struct Request<T> {
    jsonrpc: String,
    method: String,
    id: u32,
    params: RequestParams<T>,
}

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub id: u32,
    pub result: T,
}

impl<T> Request<T> {
    pub fn new(service: &str, method: Option<&str>, args: T) -> Request<T> {
        let mut rng = thread_rng();
        Request {
            jsonrpc: String::from("2.0"),
            method: String::from("call"),
            params: RequestParams {
                service: service.to_string(),
                method: method.unwrap_or("execute_kw").to_string(),
                args,
            },
            id: rng.gen_range(1..10000),
        }
    }
}