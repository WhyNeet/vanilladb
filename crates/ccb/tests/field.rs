use std::collections::HashMap;

use ccb::{deserialize::Deserialize, field::Field, serialize::Serialize};

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

    assert_eq!(
        &buffer[..],
        [
            9, 30, 0, 0, 0, 115, 116, 97, 114, 115, 0, 3, 4, 0, 0, 0, 100, 0, 0, 0, 110, 97, 109,
            101, 0, 0, 7, 0, 0, 0, 119, 104, 121, 110, 101
        ]
    );
}

#[test]
fn string_deserialization_works() {
    let buffer = [0, 5, 0, 0, 0, 119, 111, 114, 108, 100]
        .to_vec()
        .into_boxed_slice();
    let field = Field::deserialize(buffer).unwrap();

    assert_eq!(format!("{:?}", field.value()), "\"world\"");
}
