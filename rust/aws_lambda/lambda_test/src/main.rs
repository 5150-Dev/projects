extern crate lambda_bootstrap;
use crate::lambda_bootstrap::lambda::response_body::ResponseBody;
use lambda_bootstrap::events;
use lambda_bootstrap::lambda;

use serde::*;
use std::collections::HashMap;

// Create a simple structure for the response body
//
#[derive(Serialize, Debug)]
struct Body {
    status: String,
    message: String,
}

impl lambda::response_body::ResponseBody for Body {}

// Create a structure for returning the response, including a status code, headers and the body
//
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

/**
 * The handler for the Lambda. This is where the event is received. 
 * The event is a JSON payload that's identical to the one you'd get in a Node Lambda
 */
fn handler(e: &events::aws_lambda::LambdaEvent) -> String {
    println!("{}", e.to_string());
    let body = Body {
        status: "success".to_string(),
        message: "all good here".to_string(),
    };
    let response = Response::new(200, None, Some(body.stringify()));
    serde_json::to_string(&response).unwrap()
}

/**
  * Just call the start function and we're done
  *
  */
fn main() {
    lambda_bootstrap::start(handler);
}
