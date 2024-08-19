# flux-serverless
 
This is an unofficial wrapper for the huggingface Serverless Inference API of Flux that supports the `image` crate.

### Usage
First, create a Flux model:

```rust
use flux_serverless::*;

let model = FluxModel::new("hf_yourtoken", 1, SCHNELL);
```

Then, you can prompt the 