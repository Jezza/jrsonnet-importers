# Importers for all!

### This just adds some extra importers for use with jrsonnet.

* `jrsonnet_importers::CargoTracer`
All files imported that resolve to a physical file on the system are emitted via the cargo directives.
* `jrsonnet_importers::from_fn`
Allows you to easily define semantics of imported files.


```rust
#[derive(rust_embed::Embed)]
#[folder = "api/"]
#[prefix = "api/"]
struct ApiFiles;

use jrsonnet_importers::*;

let importer = CargoTracer::new({
    // So we can import files next to the 'input' file.
    let folders = vec![parent.to_path_buf()];
    
    from_fn(move |from, path| {
        // Handy function to resolve a file from folders.
        if let Some(source) = resolve_from(&folders, path)? {
            return Ok(Some(source));
        }
        
        // Failing that, we attempt to resolve the file via the embedded files, courtesy of `rust_embed`
        if let Some(source) = resolve_embed::<ApiFiles>(from, path)? {
            return Ok(Some(source));
        }
        
        Ok(None)
    })
});

state.set_import_resolver(importer);

```