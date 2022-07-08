extern crate lambda_bootstrap;
use crate::lambda_bootstrap::lambda::response_body::ResponseBody;
use lambda_bootstrap::events;
use lambda_bootstrap::lambda;

use serde::*;
use std::collections::HashMap;

#[derive(Serialize, Debug)]
struct Body {
    status: String,
    message: String,
}

impl lambda::response_body::ResponseBody for Body {}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct Response {
    statusCode: u32,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

#[warn(non_snake_case)]
impl Response {
    pub fn new(
        status_code: u32,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> Self {
        Self {
            statusCode: status_code,
            headers: headers,
            body: body,
        }
    }
}

fn handler(e: &events::aws_lambda::LambdaEvent) -> String {
    println!("{}", e.to_string());
    let body = Body {
        status: "success".to_string(),
        message: "all good here".to_string(),
    };
    let response = Response::new(200, None, Some(body.stringify()));
    serde_json::to_string(&response).unwrap()
}

fn main() {
    lambda_bootstrap::start(handler);
}
