use crate::map::Map;
use crate::regen_error::RegenError;
use tiled::Loader;

pub struct TiledMapLoader {}

impl TiledMapLoader {
    pub fn load(file: &str) -> Result<Map<u32>, RegenError> {
        let mut loader = Loader::new();
        let tmx_map = loader
            .load_tmx_map(file)
            .map_err(|_| RegenError::InvalidArgument)?;

        let mut tile_layers = tmx_map
            .layers()
            .filter_map(|layer| match layer.layer_type() {
                tiled::LayerType::Tiles(layer) => Some(layer),
                _ => None,
            });
        let tile_layer = tile_layers.next().ok_or(RegenError::InvalidArgument)?;

        if let tiled::TileLayer::Finite(finite) = tile_layer {
            let mut result = Map::<u32>::new((finite.width(), finite.height()).into());
            for x in 0..finite.width() {
                for y in 0..finite.height() {
                    let tile = finite
                        .get_tile(x as i32, y as i32)
                        .ok_or(RegenError::InvalidArgument)?;
                    result.set((x, y).into(), tile.id())?;
                }
            }
            Ok(result)
        } else {
            Err(RegenError::InvalidArgument)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let result = TiledMapLoader::load("/home/peter/tilemap.tmx");

        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get_size(), (10, 10).into());
        assert_eq!(map.get((0, 0).into()).unwrap(), &0);
        assert_eq!(map.get((1, 1).into()).unwrap(), &1);
    }
}
