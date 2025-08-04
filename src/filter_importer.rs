use std::path::Path;

use crate::{filter::FilterCollection, map_importer::MapImporter, tisu_error::TisuError};

pub trait FilterImporter {
    fn load<T: MapImporter>(
        file: impl AsRef<Path>,
        wildcard: Option<u32>,
    ) -> Result<FilterCollection<Option<u32>>, TisuError>;
}
