# wasm-tensorflow-tfjs-core
What if you wanted to do this in JavaScript?
```js
import '@tensorflow/tfjs-backend-webgl'
import { setBackend } from '@tensorflow/tfjs-core'

await setBackend('wegbl')
```

Just add this crate and you can do it in Rust!
```rs
// Somehow import '@tensorflow/tfjs-backend-webgl'
use wasm_tensorflow_tfjs_core::{set_backend, BackendName};

let result = set_backend(BackendName::Webgl).await;
```
