use std::path::{Path, PathBuf};

use crate::filter::FilterProperties;
use crate::map::Map;
use crate::rect2::Rect2u;
use crate::tisu_error::TisuError;

pub struct MapLayer {
    pub map: Map<Option<u32>>,
}

pub struct PropertyRect {
    pub rect: Rect2u,
    pub properties: FilterProperties,
}

pub struct PropertyLayer {
    pub property_rects: Vec<PropertyRect>,
}

impl PropertyLayer {
    pub fn get_properties_for_rects(
        &self,
        rect1: Rect2u,
        rect2: Rect2u,
    ) -> Option<FilterProperties> {
        for property_rect in &self.property_rects {
            if property_rect.rect.contains_rect(&rect1) && property_rect.rect.contains_rect(&rect2)
            {
                return Some(property_rect.properties.clone());
            }
        }

        None
    }
}

pub struct LoadResult {
    pub map_layers: Vec<MapLayer>,
    pub property_layers: Vec<PropertyLayer>,
    pub tileset_path: PathBuf,
}

impl LoadResult {
    pub fn get_properties_for_rects(
        &self,
        rect1: Rect2u,
        rect2: Rect2u,
    ) -> Result<FilterProperties, TisuError> {
        for property_layer in &self.property_layers {
            if let Some(properties) = property_layer.get_properties_for_rects(rect1, rect2) {
                return Ok(properties);
            }
        }
        Ok(FilterProperties::default())
    }
}

pub trait Importer {
    fn load(file: impl AsRef<Path>) -> Result<LoadResult, TisuError>;
}
