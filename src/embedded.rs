use crate::FromSource;
use jrsonnet_evaluator::error::Result as JResult;
use jrsonnet_gcmodule::Trace;
use jrsonnet_parser::{SourcePath, SourcePathT};
use std::any::Any;
use std::path::Path;

pub fn resolve_embed<T: rust_embed::Embed>(
    from: FromSource,
    path: &str,
) -> JResult<Option<SourcePath>> {
    let source_path;
    let path = match from {
        FromSource::Virtual(sibling) => {
            if let Some(sibling) = sibling.rsplit_once("/") {
                let (folder, _file) = sibling;
                let path = path.strip_prefix("./").unwrap_or(path);
                source_path = format!("{}/{}", folder, path);
                &source_path
            } else {
                // The sibling isn't in a "folder", so we can just return the path, and use that directly.
                path
            }
        }
        FromSource::Physical(_) => path,
    };

    let Some(file) = T::get(path) else {
        return Ok(None);
    };

    let source = SourceEmbed::new(path, file.data.to_vec());

    Ok(Some(SourcePath::new(source)))
}

macro_rules! any_ext_impl {
    ($T:ident) => {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn dyn_hash(&self, mut hasher: &mut dyn std::hash::Hasher) {
            use std::hash::Hash;
            self.hash(&mut hasher)
        }
        fn dyn_eq(&self, other: &dyn $T) -> bool {
            let Some(other) = other.as_any().downcast_ref::<Self>() else {
                return false;
            };
            let this = <Self as $T>::as_any(self)
                .downcast_ref::<Self>()
                .expect("restricted by impl");
            this == other
        }
        fn dyn_debug(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            <Self as std::fmt::Debug>::fmt(self, fmt)
        }
    };
}

#[derive(Trace, Hash, PartialEq, Eq, Debug)]
pub struct SourceEmbed {
    pub path: String,
    pub data: Vec<u8>,
}

impl SourceEmbed {
    pub fn new(path: impl Into<String>, data: impl Into<Vec<u8>>) -> Self {
        Self {
            path: path.into(),
            data: data.into(),
        }
    }
}

impl std::fmt::Display for SourceEmbed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path)
    }
}

impl SourcePathT for SourceEmbed {
    fn is_default(&self) -> bool {
        false
    }

    fn path(&self) -> Option<&Path> {
        None
    }
    any_ext_impl!(SourcePathT);
}
