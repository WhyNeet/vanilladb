use std::{collections::HashMap, ffi::c_void, ptr};

use trail::{deserialize::Deserialize, field::Field, serialize::Serialize};

#[test]
fn string_serialization_works() {
    let field = Field::string("world".to_string());
    let buffer = field.serialize().unwrap();

    /*
      [0, 5, 0, 0, 0, 119, 111, 114, 108, 100]

      // type - 0 (String)
      // length - 5, 0, 0, 0 (5)
      // value - 119, 111, 114, 108, 100 ("world")
    */
    assert_eq!(&buffer[..], [0, 5, 0, 0, 0, 119, 111, 114, 108, 100]);
}

#[test]
fn map_serialization_works() {
    let name = Field::string("whyneet".to_string());
    let stars = Field::int32(100);

    let mut map = HashMap::new();
    map.insert("name", name);
    map.insert("stars", stars);

    let field = Field::map_str(map);

    let buffer = field.serialize().unwrap();

    assert_eq!(buffer.len(), 37);
    assert_eq!(&buffer[..5], [9, 32, 0, 0, 0]);

    if buffer[5] == 115 {
        assert_eq!(
            &buffer[..],
            [
                9, 32, 0, 0, 0, 115, 116, 97, 114, 115, 0, 3, 4, 0, 0, 0, 100, 0, 0, 0, 110, 97,
                109, 101, 0, 0, 7, 0, 0, 0, 119, 104, 121, 110, 101, 101, 116
            ]
        );
    } else {
        assert_eq!(
            &buffer[..],
            [
                9, 32, 0, 0, 0, 110, 97, 109, 101, 0, 0, 7, 0, 0, 0, 119, 104, 121, 110, 101, 101,
                116, 115, 116, 97, 114, 115, 0, 3, 4, 0, 0, 0, 100, 0, 0, 0
            ]
        );
    }
}

#[test]
fn string_deserialization_works() {
    let buffer = [0, 5, 0, 0, 0, 119, 111, 114, 108, 100];
    let field = Field::deserialize(&buffer).unwrap();

    assert_eq!(format!("{:?}", field.value()), "\"world\"");
}

#[test]
fn map_deserialization_works() {
    let buffer = [
        9, 32, 0, 0, 0, 115, 116, 97, 114, 115, 0, 3, 4, 0, 0, 0, 100, 0, 0, 0, 110, 97, 109, 101,
        0, 0, 7, 0, 0, 0, 119, 104, 121, 110, 101, 101, 116,
    ];
    let field = Field::deserialize(&buffer).unwrap();

    let read_box_ref = |b: &Box<dyn Serialize>| {
        let value = Box::into_raw(unsafe { ptr::read(b as *const Box<dyn Serialize>) });
        let value = value as *mut c_void;
        value
    };

    let value = read_box_ref(field.value()) as *mut HashMap<String, Field>;

    unsafe {
        assert!((*value).get("name").is_some());

        let name = read_box_ref((*value).get("name").unwrap().value()) as *mut String;
        assert_eq!(*name, "whyneet".to_string());

        let stars = read_box_ref((*value).get("stars").unwrap().value()) as *mut i32;
        assert_eq!(*stars, 100);
    }
}

#[test]
fn field_eq_works() {
    assert_eq!(
        Field::string("hello".to_string()),
        Field::string("hello".to_string())
    );
    assert_eq!(Field::byte(8), Field::byte(8));
    assert_eq!(Field::ubyte(8), Field::ubyte(8));
    assert_eq!(Field::int32(8), Field::int32(8));
    assert_eq!(Field::uint32(8), Field::uint32(8));
    assert_eq!(Field::int64(8), Field::int64(8));
    assert_eq!(Field::uint64(8), Field::uint64(8));
    assert_eq!(Field::float32(8f32), Field::float32(8f32));
    assert_eq!(Field::float64(8f64), Field::float64(8f64));

    let map = {
        let name = Field::string("whyneet".to_string());
        let stars = Field::int32(100);

        let mut map = HashMap::new();
        map.insert("name", name);
        map.insert("stars", stars);

        map
    };

    let map2 = {
        let name = Field::string("whyneet".to_string());
        let stars = Field::int32(100);

        let mut map = HashMap::new();
        map.insert("name", name);
        map.insert("stars", stars);

        map
    };

    let map = Field::map_str(map);
    let map2 = Field::map_str(map2);

    assert_eq!(map, map2);

    let map3 = {
        let name = Field::string("whyneet".to_string());
        let stars = Field::int32(99535);

        let mut map = HashMap::new();
        map.insert("name", name);
        map.insert("stars", stars);

        map
    };

    let map3 = Field::map_str(map3);

    assert_ne!(map2, map3);
}
