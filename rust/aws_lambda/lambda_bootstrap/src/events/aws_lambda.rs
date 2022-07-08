extern crate minreq;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct LambdaEvent {
    pub resource: Option<String>,
    pub path: Option<String>,
    pub httpMethod: String,
    pub headers: HashMap<String, String>,
    pub multiValueHeaders: HashMap<String, Vec<String>>,
    pub queryStringParameters: Option<String>,
    pub multiValueQueryStringParameters: Option<HashMap<String, Vec<String>>>,
    pub pathParameters: Option<Vec<String>>,
    pub stageVariables: Option<HashMap<String, String>>,
    pub requestContext: RequestContext,
    pub body: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub cognitoIdentityPoolId: Option<String>,
    pub accountId: Option<String>,
    pub cognitoIdentityId: Option<String>,
    pub caller: Option<String>,
    pub sourceIp: Option<String>,
    pub principalOrgId: Option<String>,
    pub accessKey: Option<String>,
    pub cognitoAuthenticationType: Option<String>,
    pub cognitoAuthenticationProvider: Option<String>,
    pub userArn: Option<String>,
    pub userAgent: Option<String>,
    pub user: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestContext {
    pub resourceId: String,
    pub authorizer: Option<HashMap<String, String>>,
    pub resourcePath: Option<String>,
    pub httpMethod: Option<String>,
    pub extendedRequestId: Option<String>,
    pub requestTime: Option<String>,
    pub path: Option<String>,
    pub accountId: Option<String>,
    pub protocol: Option<String>,
    pub stage: Option<String>,
    pub domainPrefix: Option<String>,
    pub requestTimeEpoch: usize,
    pub requestId: Option<String>,
    pub domainName: String,
    pub apiId: String,
}

impl LambdaEvent {
    pub fn new(response: &minreq::Response) -> Self {
        let evt = match response.json::<LambdaEvent>() {
            Ok(e) => e,
            Err(why) => panic!("Can't parse the event: {:?}. Reason: {}", response.as_str(), why),
        };

        evt
    }

    pub fn to_string(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json_str) => match json::parse(&json_str) {
                Ok(parsed_json) => json::stringify(parsed_json),
                Err(_) => String::from(""),
            },
            Err(_) => String::from(""),
        }
    }
}
