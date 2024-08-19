# flux-serverless
 
This is an unofficial wrapper for the huggingface Serverless Inference API of Flux that supports the `image` crate.

### Usage
First, create a Flux model:

```rust
use flux_serverless::*;

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