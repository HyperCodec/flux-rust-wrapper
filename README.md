# hf-text-to-image
 
This is an unofficial wrapper for the huggingface Serverless Inference API that supports the `image` crate.

### Installing dependency
This crate is not published on crates.io.
To use it as a dependency, you must add the following to your Cargo.toml:
```toml
[dependencies.hf-text-to-image]
git = "https://github.com/HyperCodec/hf-text-to-image.git"
```

### Usage
First, create a HF client:

```rust
use hf_text_to_image::prelude::*;

// use your model of choice (as long as it supports the `text-to-image` task)
const API_URL: &str = "https://api-inference.huggingface.co/models/black-forest-labs/FLUX.1-schnell";
let client = HFClient::new("hf_yourtoken", API_URL);
```

Then, you construct your prompt:
```rust
let payload = InferencePayload {
    prompt: "Astronaut riding a horse",
    ..default()
}
```

Finally, request your image and save it.

```rust
let image = client.request_inference(payload).await?;
image.save("astronaut.png")?;
```

### License
This crate uses the `Apache 2.0` license.
