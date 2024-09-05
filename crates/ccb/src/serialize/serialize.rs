pub trait Serialize {
    fn serialize(self) -> Box<[u8]>;
}
