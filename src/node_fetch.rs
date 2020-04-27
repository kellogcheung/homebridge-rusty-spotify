use js_sys::{Array, Function, Promise};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    fn require(name: &str) -> Function;

    type Response;

    #[wasm_bindgen(method, js_name = json)]
    fn json(this: &Response) -> Promise;

    #[wasm_bindgen(method, js_name = text)]
    fn text(this: &Response) -> Promise;
}

pub enum FetchMethod {
    Post,
    Put,
}

impl FetchMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            &FetchMethod::Post => "POST",
            &FetchMethod::Put => "PUT",
        }
    }
}

#[derive(Serialize)]
struct RequestOptions {
    method: String,
    body: String,
    headers: HashMap<String, String>,
}

pub async fn fetch(
    url: &str,
    method: FetchMethod,
    body: &str,
    headers: HashMap<String, String>,
    empty_response: bool,
) -> Result<JsValue, JsValue> {
    let fetch = require("node-fetch");
    let options = RequestOptions {
        method: method.as_str().to_owned(),
        body: body.to_owned(),
        headers,
    };

    let fetch_result = fetch.apply(
        &JsValue::null(),
        &Array::of2(&JsValue::from(url), &JsValue::from_serde(&options).unwrap()),
    );
    match fetch_result {
        Ok(p) => {
            let promise = Promise::from(p);
            let resp_value = JsFuture::from(promise).await?;
            let resp: Response = resp_value.unchecked_into();

            if empty_response {
                Ok(JsValue::NULL)
            } else {
                let json: JsValue = JsFuture::from(resp.json()).await?;
                Ok(json)
            }
        }
        _ => Err(JsValue::from("Error executing fetch request")),
    }
}