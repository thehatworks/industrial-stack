use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Map, Value};
use chrono::{DateTime, Utc};
use std::fmt;

#[macro_use]
extern crate serde_json;

// You will want to look here
// https://github.com/rustwasm/wasm-bindgen

// This is Serde & Serde-JSON
#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    done: bool,
    priority: Number,
    due_date: String,
    title: String,
    steps: Vec<String>,
    followers: Map<String, Value>,
}

fn x_for_bool(checked: &bool) -> &str {
    match checked {
        true => "x",
        false => " "
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", x_for_bool(&self.done))
    }
}

fn inc_serialized_datetime_by_one_month(isodate: &str) -> Result<String, JsError> {
    let datetime = DateTime::parse_from_rfc3339(isodate)?;
    let datetime_utc = datetime.with_timezone(&Utc);
    Ok(format!("{}", datetime_utc))
}

#[wasm_bindgen]
pub fn possibly_recur_todo(serialized_todo: &str) -> Result<String, JsError> {
    // trailing ? is rust's "try catch"
    let todo: Todo = serde_json::from_str(serialized_todo)?;

    // show how we can do some print-style debugging in rust
    // and pass it all the way out to deno runtime's console.log
    log(format!("this is possible because we derived Debug above: {:?}", todo).as_str());
    log(format!("this is possible because we impl'd fmt::Display for Todo: {}", todo).as_str());


    let modified_data = match todo {
        Todo { done: true, .. } => Todo {
            done: false,
            due_date: inc_serialized_datetime_by_one_month(todo.due_date.as_str())?,
            ..todo
        },
        _ => todo
    };

    Ok(serde_json::to_string(&modified_data)?)
}

#[wasm_bindgen]
extern "C" {
    // this is how to bring a library function from javascript into rust (i.e. console.log)
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


// here's how to unit test your rust functions
#[cfg(test)]
mod tests {
    use super::possibly_recur_todo;

    #[test]
    fn it_works() {
        let fixture = r#"
        {
            "done": true,
            "priority": 42,
            "title": "Important Task",
            "due_date": "2022-05-23T18:29:58.011Z"
            "steps": ["Read", "Calculate", "Write"],
            "followers": {
                "me": {
                    "email": "me@example.com"
                },
                "coworker: {
                    "email": "coworker@example.com"
                }
            }
        }"#;
        
        let check_result = json!({
           "done": false,
           "priority": 42i32,
           "title": "Important Task",
           "due_date": "2022-05-23T18:29:58.011Z",
           "steps": ["Read", "Calculate", "Write"],
           "followers": {
               "me": {
                   "email": "me@example.com"
                }
            }
        });
        
        /*
        let result = match possibly_recur_todo(fixture) {
            OK(serialized) => serde_json::from_str(serialized),
            Error(_) => panic!("darn!")
        };

        assert_eq!(result, check_result);
        */

    }
}
