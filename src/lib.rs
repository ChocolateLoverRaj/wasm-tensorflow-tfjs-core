use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{js_sys::Promise, JsFuture};

mod bindings;

pub enum BackendName {
    Webgl,
    Cpu,
    Tensorflow,
}

impl ToString for BackendName {
    fn to_string(&self) -> String {
        match self {
            BackendName::Cpu => "cpu",
            BackendName::Webgl => "webgl",
            BackendName::Tensorflow => "tensorflow",
        }
        .into()
    }
}

pub async fn set_backend(backend_name: BackendName) -> Result<bool, JsValue> {
    // bindings::set_backend(&backend_name.to_string());
    let result = JsFuture::from(Promise::from(bindings::set_backend(
        &backend_name.to_string(),
    )))
    .await?
    .as_bool()
    .unwrap();
    Ok(result)
}
