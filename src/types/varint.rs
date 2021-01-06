use serde::{
    de::{SeqAccess, Visitor},
    Serializer,
};

pub type VarInt = i32;

pub fn deserialize<'de, D>(d: D) -> ::std::result::Result<VarInt, D::Error>
where
    D: serde::Deserializer<'de>,
{
    d.deserialize_tuple(5, VarIntVisitor)
}

pub struct VarIntVisitor;

impl<'de> Visitor<'de> for VarIntVisitor {
    type Value = VarInt;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a VarInt encoded as bytes")
    }

    fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut count = 0;
        let mut result = 0u32;
        let mut read;
        while {
            read = seq.next_element::<u8>()?.unwrap();
            let value = read & 0x7F;
            result |= (value as u32) << (7 * count);
            count += 1;
            if count > 5 {
                panic!("Error not implemented");
            }
            (read & 0x80) != 0
        } {}
        Ok(unsafe { std::mem::transmute(result) })
    }
}

pub fn to_bytes(varint: i32) -> Vec<u8> {
    let mut value = unsafe { std::mem::transmute::<i32, u32>(varint) };
    let mut out = vec![];
    while {
        let mut temp = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0x80;
        }
        out.push(temp);
        value != 0
    } {}

    out
}

pub fn serialize<S>(varint: &VarInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bytes(&to_bytes(*varint))
}
