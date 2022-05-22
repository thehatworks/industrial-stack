use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn string_manip(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn entrypoint(name: &str) -> String {
    let out = string_manip(name);
    log(out.as_str());
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let out = super::string_manip("YO");
        assert_eq!(out, "Hello, YO!");
    }
}
