## aws_lambda with Rust

This project allows developers to create Lambda functions using Rust.

### Layout

```
lambda_bootstrap
```

This is were all the code for the runtime goes.
It exposes a single function ```lambda_bootstrap::start(handler)``` which will start the event loop and respond to events comming from the source (API GW only for now).

```
lambda_test
```

Simple test Lambda

```
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
```

Essentially, all main has to do is to call ```start(handler)``` and let the handler do its work.
The handler will be called with an event, just like it would in a NodeJS or Python Lambda.

Once the work is done, simply return the response, which will be forwarded to the API GW and sent back.


