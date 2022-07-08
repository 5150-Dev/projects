extern crate minreq;
use crate::events::aws_lambda;
use std::env;

/**
  * Start the event loop
  * 1: Fetch from the server (blocking call)
  * 2: if the request_id is present/valid execute the function
  * 3: Send the response that the function returned
  *
  */
pub fn start_loop(lambda: fn(&aws_lambda::LambdaEvent) -> String) {
    loop {
        let (event, request_id) = fetch_from_server();
        match request_id {
            Some(r_id) => {
                let response = lambda(&event);
                send_response(response, r_id.as_str());
            },
            None => println!("No request ID"),
        };
    }
}

/**
  * Fetch a request from the local server.
  * The key is obtained from the environment
  *
  */
fn fetch_from_server() -> (aws_lambda::LambdaEvent, Option<String>) {
    // Get the host and port to query
    //
    let key = "AWS_LAMBDA_RUNTIME_API";
    let server_url = match env::var_os(key) {
        Some(val) => format!(
            "http://{}/2018-06-01/runtime/invocation/next",
            val.into_string().unwrap()
        ),
        None => panic!("{} is not defined in the environment.", key),
    };

    // Get the event and context for the Lambda
    //
    let response = minreq::get(server_url).send().unwrap();
    match response.headers.get("lambda-runtime-aws-request-id") {
        Some(r_id) => (
            aws_lambda::LambdaEvent::new(&response),
            Some(r_id.to_string()),
        ),
        None => (
            aws_lambda::LambdaEvent::new(&response),
            None
        )
    }
}

/**
  * Send the response back to the local server
  * That will be returned to API GW
  *
  */
fn send_response(response: String, request_id: &str) {
    let key = "AWS_LAMBDA_RUNTIME_API";
    let server_url = match env::var_os(key) {
        Some(val) => format!(
            "http://{}/2018-06-01/runtime/invocation/{}/response",
            val.into_string().unwrap(),
            request_id
        ),
        None => panic!("{} is not defined in the environment.", key),
    };
    
    println!("POST {} to {}", response, server_url);
    minreq::post(server_url).with_body(response).send().unwrap();
}
