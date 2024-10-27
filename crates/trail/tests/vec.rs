use std::ptr;

use trail::{deserialize::Deserialize, field::Field, serialize::Serialize};

#[test]
pub fn vec_serialization_works() {
    let vec = vec![
        Field::string("Hello, world!".to_string()),
        Field::uint32(10),
    ];
    let buffer = vec.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        buffer,
        vec![
            31, 0, 0, 0, 0, 13, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100,
            33, 4, 4, 0, 0, 0, 10, 0, 0, 0
        ]
        .into_boxed_slice()
    );
}

#[test]
pub fn vec_deserialization_works() {
    let buffer = vec![
        31, 0, 0, 0, 0, 13, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33,
        4, 4, 0, 0, 0, 10, 0, 0, 0,
    ];

    let vec = Vec::<Field>::deserialize(&buffer);
    assert!(vec.is_ok());

    let vec = vec.unwrap();

    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(vec[0].value() as *const Box<dyn Serialize>)) as *const String)
                .as_ref()
                .unwrap()
        },
        &"Hello, world!".to_string()
    );

    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(vec[1].value() as *const Box<dyn Serialize>)) as *const u32)
                .as_ref()
                .unwrap()
        },
        &10
    );
}
