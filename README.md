# flux-serverless
 
This is an unofficial wrapper for the huggingface Serverless Inference API of Flux that supports the `image` crate.

### Installing dependency
This crate is not published on crates.io.
To use it as a dependency, you must add `flux-serverless = { git = "https://github.com/HyperCodec/flux-rust-wrapper.git" }` to your Cargo.toml

### Usage
First, create a Flux model:

```rust
use flux_serverless::prelude::*;

let model = FluxModel::new("hf_yourtoken", 1, SCHNELL);
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
let image = model.request_inference(payload).await?;
image.save("astronaut.png")?;
```