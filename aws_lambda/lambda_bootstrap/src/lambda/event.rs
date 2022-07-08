extern crate minreq;

#[derive(Debug)]
pub struct Event {
    pub Request_ID: String,
    pub Host: String,
}

impl Event {
    pub fn new(response: minreq::Response) -> Self {
        // lambda-runtime-aws-request-id
        Event {
            Host: "".to_string(),
            Request_ID: response
                .headers
                .get("lambda-runtime-aws-request-id")
                .unwrap()
                .to_string(),
        }
    }
}
