use wasm_bindgen::prelude::*;

use crate::Request;
use web_sys::RequestInit;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = :: js_sys :: Object , js_name = EventTarget)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventTarget` class."]
    pub type EventTarget;

    #[wasm_bindgen (extends = EventTarget , extends = :: js_sys :: Object , js_name = WorkerGlobalScope)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerGlobalScope` class."]
    pub type WorkerGlobalScope;

    #[wasm_bindgen (catch , method , structural , js_class = "WorkerGlobalScope" , js_name = atob)]
    #[doc = "The `atob()` method."]
    pub fn atob(this: &WorkerGlobalScope, atob: &str) -> Result<String, JsValue>;

    #[wasm_bindgen (catch , method , structural , js_class = "WorkerGlobalScope" , js_name = btoa)]
    #[doc = "The `btoa()` method."]
    pub fn btoa(this: &WorkerGlobalScope, btoa: &str) -> Result<String, JsValue>;

    #[wasm_bindgen (method , structural , js_class = "WorkerGlobalScope" , js_name = fetch)]
    #[doc = "The `fetch()` method."]
    pub fn fetch_with_request(this: &WorkerGlobalScope, input: &Request) -> ::js_sys::Promise;

    #[wasm_bindgen (method , structural , js_class = "WorkerGlobalScope" , js_name = fetch)]
    #[doc = "The `fetch()` method."]
    pub fn fetch_with_str(this: &WorkerGlobalScope, input: &str) -> ::js_sys::Promise;

    #[wasm_bindgen (method , structural , js_class = "WorkerGlobalScope" , js_name = fetch)]
    #[doc = "The `fetch()` method."]
    pub fn fetch_with_request_and_init(
        this: &WorkerGlobalScope,
        input: &Request,
        init: &RequestInit,
    ) -> ::js_sys::Promise;

    #[wasm_bindgen (method , structural , js_class = "WorkerGlobalScope" , js_name = fetch)]
    #[doc = "The `fetch()` method."]
    pub fn fetch_with_str_and_init(
        this: &WorkerGlobalScope,
        input: &str,
        init: &RequestInit,
    ) -> ::js_sys::Promise;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}
