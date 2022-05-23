use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Number, Map};

// You will want to look here
// https://github.com/rustwasm/wasm-bindgen


// This is Serde & Serde-JSON
#[derive(Debug, Serialize, Deserialize)]
struct ExampleRecord {
    truthy: bool,
    mathsy: Number,
    spelly: String,
    county: Vec<Value>,
    mappy: Map<String, Value>,
}

#[wasm_bindgen]
pub fn typed_and_serialized_from_deno(data: &str) -> Result<String, JsError> {
    // Some JSON input data as a &str. Maybe this comes from the user.

    // trailing ? is rust's "try catch"
    let some_data: ExampleRecord = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    log(format!("this is possible because we derived Debug above: {:?}", some_data).as_str());
    log(format!("Specific values: {:?} {}", some_data.truthy, some_data.spelly).as_str());


    let modified_data = match some_data {
        _ => ExampleRecord { truthy: !some_data.truthy, spelly: String::from("base case of match from Rust 🦀"), ..some_data }
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
    #[test]
    fn it_works() {
        let fixture = r#"
        {
            "truthy": false,
            "mathsy": 42,
            "spelly": "hi there",
            "county": [],
            "mappy": {}
        }"#;
        /*        
        let fixture_json = json!({
           truthy: false,
           mathsy: 42,
           spelly: "hi there",
           county: [],
           mappy: {}
       });
        */
        

        /* 
        assert_eq!(
            super::typed_and_serialized_from_deno(&fixture).unwrap(),
            String::from("hi there")
        );
        */
    }
}
