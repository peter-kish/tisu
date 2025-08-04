use std::path::Path;

use crate::{filter::FilterCollection, tisu_error::TisuError};

pub trait FilterImporter {
    fn load(
        file: impl AsRef<Path>,
        wildcard: Option<u32>,
    ) -> Result<FilterCollection<Option<u32>>, TisuError>;
}
