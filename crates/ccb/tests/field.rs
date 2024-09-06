use ccb::{field::Field, serialize::Serialize};

#[test]
pub fn field_serialization_works() {
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
