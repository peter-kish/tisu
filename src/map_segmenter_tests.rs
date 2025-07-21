use crate::map::Map;
use crate::map_segmenter;
use crate::rect2::Rect2u;
use crate::tisu_error::TisuError;

#[test]
fn test_is_field_transparent() {
    // 1 0
    // 0 0
    let map = Map::<u32>::from_data([[1, 0], [0, 0]]).unwrap();

    assert!(!map_segmenter::is_field_transparent(
        &map,
        &0,
        (0, 0).into()
    ));
    assert!(map_segmenter::is_field_transparent(&map, &0, (1, 0).into()));
    assert!(map_segmenter::is_field_transparent(&map, &0, (2, 0).into()));
}

#[test]
fn test_is_rect_start() {
    // 0 0 0 0
    // 0 1 1 0
    // 0 0 0 0
    let map = Map::<u32>::from_data([[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]).unwrap();

    assert!(!map_segmenter::is_rect_start(&map, &0, (0, 0).into()));
    assert!(!map_segmenter::is_rect_start(&map, &0, (1, 0).into()));
    assert!(map_segmenter::is_rect_start(&map, &0, (1, 1).into()));
    assert!(!map_segmenter::is_rect_start(&map, &0, (2, 1).into()));
    assert!(!map_segmenter::is_rect_start(&map, &0, (3, 1).into()));
    assert!(!map_segmenter::is_rect_start(&map, &0, (4, 1).into()));

    // 1 1 0
    // 0 0 0
    let map = Map::<u32>::from_data([[1, 1, 0], [0, 0, 0]]).unwrap();
    assert!(map_segmenter::is_rect_start(&map, &0, (0, 0).into()));

    // 0 1 1
    // 0 0 0
    let map = Map::<u32>::from_data([[0, 1, 1], [0, 0, 0]]).unwrap();
    assert!(map_segmenter::is_rect_start(&map, &0, (1, 0).into()));
}

#[test]
fn test_find_rect_start_success() {
    // 0 0 0 0
    // 0 1 1 0
    // 0 0 0 0
    let map = Map::<u32>::from_data([[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]).unwrap();

    let result = map_segmenter::find_rect_start(&map, &0, (0, 0).into());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), (1, 1).into());
}

#[test]
fn test_find_rect_start_failure() {
    // 0 0 0 0
    // 0 0 0 0
    // 0 0 0 0
    let map = Map::<u32>::from_data([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]).unwrap();

    let result = map_segmenter::find_rect_start(&map, &0, (0, 0).into());

    assert_eq!(result.err().unwrap(), TisuError::NotFound);
}

#[test]
fn test_find_rect_size() {
    // 0 0 0 0
    // 0 1 1 0
    // 0 0 0 0
    let map = Map::<u32>::from_data([[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]).unwrap();

    assert_eq!(
        map_segmenter::find_rect_size(&map, &0, (0, 0).into()),
        (0, 0).into()
    );
    assert_eq!(
        map_segmenter::find_rect_size(&map, &0, (1, 1).into()),
        (2, 1).into()
    );
}

#[test]
fn test_extract_segments() {
    // 0 0 0 0 0 0
    // 0 1 1 0 1 0
    // 0 0 0 0 0 0
    // 0 1 1 0 1 0
    // 0 1 1 0 0 0
    // 0 0 0 0 0 0
    let map = Map::<u32>::from_data([
        [0, 0, 0, 0, 0, 0],
        [0, 1, 1, 0, 1, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 1, 1, 0, 1, 0],
        [0, 1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
    ])
    .unwrap();
    let expected_rects = [
        Rect2u::try_from((1, 1, 2, 1)).unwrap(),
        Rect2u::try_from((4, 1, 1, 1)).unwrap(),
        Rect2u::try_from((1, 3, 2, 2)).unwrap(),
        Rect2u::try_from((4, 3, 1, 1)).unwrap(),
    ];

    let result = map_segmenter::extract_segments(&map, &0);

    assert_eq!(result, expected_rects);
}
