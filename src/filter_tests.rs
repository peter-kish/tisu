use std::path::Path;

use crate::{
    filter::{Filter, FilterCollection},
    map::Map,
    tiled_filter_loader::TiledFilterLoader,
    tiled_map_loader::TiledMapLoader,
    tisu_error::TisuError,
};

#[test]
fn test_constructor_success() {
    let pattern = Map::<u32>::new((2, 2).into());
    let substitute = Map::<u32>::new((2, 2).into());
    let result = Filter::new(pattern.clone(), substitute.clone(), 42);

    assert!(result.is_ok());
    let filter = result.unwrap();
    assert_eq!(filter.pattern(), &pattern);
    assert_eq!(filter.substitute(), &substitute);
}

#[test]
fn test_constructor_failure() {
    let pattern = Map::<u32>::new((2, 2).into());
    let substitute = Map::<u32>::new((3, 2).into());
    let result = Filter::new(pattern, substitute, 42);

    assert_eq!(result.err().unwrap(), TisuError::InvalidMapSize);
}

#[test]
fn test_pattern_matches() {
    let map = Map::<u32>::from_data([[1, 0], [0, 1]]).unwrap();
    let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
    let substitute = Map::<u32>::from_data([[1, 0]]).unwrap();
    let filter = Filter::new(pattern, substitute, 42).unwrap();

    assert!(filter.pattern_matches(&map, (0, 0).into(), &None));
    assert!(!filter.pattern_matches(&map, (0, 1).into(), &None));
    assert!(!filter.pattern_matches(&map, (1, 1).into(), &None));
}

#[test]
fn test_pattern_match_with_wildcard() {
    let map = Map::<u32>::from_data([[1, 0], [1, 1]]).unwrap();
    let pattern = Map::<u32>::from_data([[1, 2]]).unwrap();
    let substitute = Map::<u32>::from_data([[1, 0]]).unwrap();
    let filter = Filter::new(pattern, substitute, 2).unwrap();

    assert!(filter.pattern_matches(&map, (0, 0).into(), &None));
    assert!(filter.pattern_matches(&map, (0, 1).into(), &None));
    assert!(!filter.pattern_matches(&map, (1, 1).into(), &None));
}

#[test]
fn test_substitute() {
    let mut map = Map::<u32>::from_data([[1, 0], [0, 1]]).unwrap();
    let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
    let substitute = Map::<u32>::from_data([[1, 0]]).unwrap();
    let filter = Filter::new(pattern, substitute, 42).unwrap();

    filter.apply_substitute(&mut map, (0, 1).into(), &mut None);
    assert_eq!(map.data(), [1, 0, 1, 0]);

    filter.apply_substitute(&mut map, (1, 0).into(), &mut None);
    assert_eq!(map.data(), [1, 1, 1, 0]);
}

#[test]
fn test_substitute_with_wildcard() {
    let mut map = Map::<u32>::from_data([[1, 0], [0, 1]]).unwrap();
    let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
    let substitute = Map::<u32>::from_data([[1, 2]]).unwrap();
    let filter = Filter::new(pattern, substitute, 2).unwrap();

    filter.apply_substitute(&mut map, (0, 1).into(), &mut None);
    assert_eq!(map.data(), [1, 0, 1, 1]);
}

#[test]
fn test_apply_filter_success() {
    // 1 0 1
    // 1 1 1
    // 1 0 1
    let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
    // 1 0
    let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
    // 0 1
    let substitute = Map::<u32>::from_data([[0, 1]]).unwrap();
    let filter = Filter::new(pattern, substitute, 42).unwrap();
    // 0 1 1
    // 1 1 1
    // 0 1 1
    let expected_data = [0, 1, 1, 1, 1, 1, 0, 1, 1];

    let result = filter.apply(&map);

    assert!(result.is_ok());
    let result_map = result.unwrap();
    assert_eq!(result_map.data(), &expected_data);
    assert_eq!(result_map.size(), map.size());
}

#[test]
fn test_apply_filter_failure() {
    // 1 0 1
    // 1 1 1
    // 1 0 1
    let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
    // 1 0 0 0
    let pattern = Map::<u32>::from_data([[1, 0, 0, 0]]).unwrap();
    // 0 1 1 1
    let substitute = Map::<u32>::from_data([[0, 1, 1, 1]]).unwrap();
    let filter = Filter::new(pattern, substitute, 42).unwrap();

    let result = filter.apply(&map);

    assert_eq!(result.err().unwrap(), TisuError::InvalidMapSize);
}

#[test]
fn test_apply_filter_patter_with_wildcard() {
    // 1 0 1
    // 1 1 1
    // 1 0 1
    let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
    // 1 W 1
    let pattern = Map::<u32>::from_data([[1, 2, 1]]).unwrap();
    // 0 1 0
    let substitute = Map::<u32>::from_data([[0, 1, 0]]).unwrap();
    let filter = Filter::new(pattern, substitute, 2).unwrap();
    // 0 1 0
    // 0 1 0
    // 0 1 0
    let expected_data = [0, 1, 0, 0, 1, 0, 0, 1, 0];

    let result = filter.apply(&map);

    assert!(result.is_ok());
    let result_map = result.unwrap();
    assert_eq!(result_map.data(), &expected_data);
    assert_eq!(result_map.size(), map.size());
}

#[test]
fn test_apply_filter_substitute_with_wildcard() {
    // 1 0 1
    // 1 1 1
    // 1 0 1
    let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
    // 1 0 1
    let pattern = Map::<u32>::from_data([[1, 0, 1]]).unwrap();
    // W 1 W
    let substitute = Map::<u32>::from_data([[2, 1, 2]]).unwrap();
    let filter = Filter::new(pattern, substitute, 2).unwrap();
    // 1 1 1
    // 1 1 1
    // 1 1 1
    let expected_data = [1, 1, 1, 1, 1, 1, 1, 1, 1];

    let result = filter.apply(&map);

    assert!(result.is_ok());
    let result_map = result.unwrap();
    assert_eq!(result_map.data(), &expected_data);
    assert_eq!(result_map.size(), map.size());
}

#[test]
fn test_apply_filter_collection_success() {
    // 1 0 1
    // 1 1 1
    // 1 0 1
    let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
    // 1 0
    let pattern1 = Map::<u32>::from_data([[1, 0]]).unwrap();
    // 0 1
    let substitute1 = Map::<u32>::from_data([[0, 1]]).unwrap();
    let filter1 = Filter::new(pattern1, substitute1, 42).unwrap();
    // 0 1 1
    let pattern2 = Map::<u32>::from_data([[0, 1, 1]]).unwrap();
    // 0 0 0
    let substitute2 = Map::<u32>::from_data([[0, 0, 0]]).unwrap();
    let filter2 = Filter::new(pattern2, substitute2, 42).unwrap();
    let filter_collection = FilterCollection::new(&[filter1, filter2]);
    // 0 0 0
    // 1 1 1
    // 0 0 0
    let expected_data = [0, 0, 0, 1, 1, 1, 0, 0, 0];

    let result = filter_collection.apply(&map);

    assert!(result.is_ok());
    let result_map = result.unwrap();
    assert_eq!(result_map.data(), &expected_data);
    assert_eq!(result_map.size(), map.size());
}

#[test]
fn test_apply_empty_filter_collection() {
    // 1 0 1
    // 1 1 1
    // 1 0 1
    let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
    let filter_collection = FilterCollection::new(&[]);
    let result = filter_collection.apply(&map);

    assert!(result.is_ok());
}

#[test]
fn test_apply_filter_collection_failure() {
    // 1 0 1
    // 1 1 1
    // 1 0 1
    let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
    // 1 0
    let pattern1 = Map::<u32>::from_data([[1, 0, 0, 0]]).unwrap();
    // 0 1
    let substitute1 = Map::<u32>::from_data([[1, 1, 1, 1]]).unwrap();
    let filter1 = Filter::new(pattern1, substitute1, 42).unwrap();
    // 0 1 1
    let pattern2 = Map::<u32>::from_data([[0, 1, 1]]).unwrap();
    // 0 0 0
    let substitute2 = Map::<u32>::from_data([[0, 0, 0]]).unwrap();
    let filter2 = Filter::new(pattern2, substitute2, 42).unwrap();
    let filter_collection = FilterCollection::new(&[filter1, filter2]);

    let result = filter_collection.apply(&map);

    assert_eq!(result.err().unwrap(), TisuError::InvalidMapSize);
}

#[test]
fn test_filter_collection_push() {
    let mut fc = FilterCollection::<u32>::default();
    // 1 0
    let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
    // 0 1
    let substitute = Map::<u32>::from_data([[0, 1]]).unwrap();
    let filter = Filter::new(pattern, substitute, 42).unwrap();

    fc.push(filter.clone());

    assert_eq!(fc.filters.len(), 1);
    assert_eq!(fc.filters[0], filter);
}

struct TestData {
    filter_collection: FilterCollection<Option<u32>>,
    input: Map<Option<u32>>,
    expected_output: Map<Option<u32>>,
}

fn load_test_map(file_path: impl AsRef<Path>) -> Map<Option<u32>> {
    let result = TiledMapLoader::load(file_path);
    result.unwrap().map_layers[0].map.clone()
}

fn load_test_data(test_name: &str) -> TestData {
    let filter_collection = TiledFilterLoader::load(
        format!(
            "{}/data/test_{}/filter_collection.tmx",
            env!("CARGO_MANIFEST_DIR"),
            test_name
        )
        .as_str(),
        Some(4),
    )
    .unwrap();

    let input = load_test_map(
        format!(
            "{}/data/test_{}/input.tmx",
            env!("CARGO_MANIFEST_DIR"),
            test_name
        )
        .as_str(),
    );

    let expected_output = load_test_map(
        format!(
            "{}/data/test_{}/expected_output.tmx",
            env!("CARGO_MANIFEST_DIR"),
            test_name
        )
        .as_str(),
    );

    TestData {
        filter_collection,
        input,
        expected_output,
    }
}

#[test]
fn apply_filter_collection() {
    let test_data = load_test_data("apply_filter_collection");

    let output = test_data.filter_collection.apply(&test_data.input).unwrap();

    assert_eq!(test_data.expected_output, output);
}

#[test]
fn apply_filter_collection_probability() {
    let test_data = load_test_data("apply_filter_collection_probability");

    let output = test_data.filter_collection.apply(&test_data.input).unwrap();

    assert_eq!(test_data.expected_output, output);
}

#[test]
fn apply_filter_collection_to_source() {
    let test_data = load_test_data("apply_filter_collection_to_source");

    let output = test_data.filter_collection.apply(&test_data.input).unwrap();

    assert_eq!(test_data.expected_output, output);
}
