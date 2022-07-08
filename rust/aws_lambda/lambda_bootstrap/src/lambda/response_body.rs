use serde::*;

pub trait ResponseBody {
    fn stringify(&self) -> String
    where
        Self: Serialize,
    {
        match serde_json::to_string(&self) {
            Ok(json_str) => match json::parse(&json_str) {
                Ok(parsed_json) => json::stringify(parsed_json),
                Err(_) => String::from(""),
            },
            Err(_) => String::from(""),
        }
    }
}
