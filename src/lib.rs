use jrsonnet_evaluator::error::{ErrorKind, Result as JResult};
use jrsonnet_evaluator::parser::{SourceFile, SourcePath};
use std::path::Path;

pub use from_fn::FromSource;
pub use tracer::CargoTracer;

mod from_fn;
mod tracer;

#[cfg(feature = "rust-embed")]
mod embedded;

#[cfg(feature = "rust-embed")]
pub use embedded::{resolve_embed, SourceEmbed};

pub fn from_fn<F>(resolver: F) -> from_fn::FnImportResolver<F>
where
    F: for<'s, 'n> Fn(FromSource<'s>, &'n str) -> JResult<Option<SourcePath>>,
{
    from_fn::FnImportResolver::new(resolver)
}

pub fn resolve_from(libraries: &[impl AsRef<Path>], path: &str) -> JResult<Option<SourcePath>> {
    for library in libraries {
        let library = library.as_ref();
        let library = library.join(path);

        if !library.exists() {
            continue;
        }

        let normalised = library
            .canonicalize()
            .map_err(|e| ErrorKind::ImportIo(e.to_string()))?;

        // @TODO jezza - 30 Sept 2024: We should probably check if we need to create SourceDirectory or a SourceFile
        let resolved = SourcePath::new(SourceFile::new(normalised));

        return Ok(Some(resolved));
    }

    Ok(None)
}