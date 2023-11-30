use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Promise;

#[wasm_bindgen(module = "@tensorflow/tfjs-core")]
extern "C" {
    #[wasm_bindgen(js_name = setBackend)]
    pub fn set_backend(backend_name: &str) -> Promise;
}
