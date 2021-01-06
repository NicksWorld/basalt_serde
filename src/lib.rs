#![feature(arbitrary_enum_discriminant)]

mod de;
mod error;
mod ser;

pub mod types;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_bytes, Serializer};

#[cfg(test)]
mod test {
    use crate::types::{varint, varlong};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    struct MyVarint {
        #[serde(with = "varint")]
        my_varint: i32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct MyVarlong {
        #[serde(with = "varlong")]
        my_varlong: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[repr(u32)]
    enum MyVarintEnum {
        None = 0,
        Some { my_string: String } = 1,
        Yes = 2,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct MyContainer {
        my_enum: MyVarintEnum,
    }

    #[repr(u8)]
    enum Chunk {
        Full {
            primary_bitmask: i32,
            heightmaps: String,
            biomes: Vec<i32>,
            size: i32,
        } = 1,
        Partial {
            primary_bitmask: i32,
            heightmaps: String,
            size: i32,
        } = 0,
    }

    struct ChunkData {
        chunk_x: i32,
        chunk_z: i32,
        chunk: Chunk,
    }

    #[test]
    fn varint_enum() {
        let tests: [&[u8]; 3] = [
            &[0x00],
            &[0x01, 0x4, 'T' as u8, 'E' as u8, 'S' as u8, 'T' as u8],
            &[0x02],
        ];

        for test in &tests {
            let deserialized: MyContainer = crate::de::from_bytes(test).unwrap();
            println!("{:?} => {:?}", test, deserialized);

            let serialized = crate::ser::to_bytes(&deserialized).unwrap();
            println!("{:?}", serialized)
        }
    }

    #[test]
    fn varint() {
        let tests: [(i32, &[u8]); 10] = [
            (0, &[0x00]),
            (1, &[0x01]),
            (2, &[0x02]),
            (127, &[0x7f]),
            (128, &[0x80, 0x01]),
            (255, &[0xff, 0x01]),
            (2097151, &[0xff, 0xff, 0x7f]),
            (2147483647, &[0xff, 0xff, 0xff, 0xff, 0x07]),
            (-1, &[0xff, 0xff, 0xff, 0xff, 0x0f]),
            (-2147483648, &[0x80, 0x80, 0x80, 0x80, 0x08]),
        ];

        for (goal, bytes) in &tests {
            let deserialized: MyVarint = crate::de::from_bytes(bytes).unwrap();
            assert_eq!(goal, &deserialized.my_varint);

            let serialized: Vec<u8> = crate::ser::to_bytes(&MyVarint { my_varint: *goal }).unwrap();
            assert_eq!(bytes.to_vec(), serialized.as_slice());
        }
    }

    #[test]
    fn varlong() {
        let tests: [(i64, &[u8]); 11] = [
            (0, &[0x00]),
            (1, &[0x01]),
            (2, &[0x02]),
            (127, &[0x7f]),
            (128, &[0x80, 0x01]),
            (255, &[0xff, 0x01]),
            (2147483647, &[0xff, 0xff, 0xff, 0xff, 0x07]),
            (
                9223372036854775807,
                &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
            ),
            (
                -1,
                &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
            ),
            (
                -2147483648,
                &[0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
            ),
            (
                -9223372036854775808,
                &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
            ),
        ];

        for (goal, bytes) in &tests {
            let deserialized: MyVarlong = crate::de::from_bytes(bytes).unwrap();
            assert_eq!(goal, &deserialized.my_varlong);

            let serialized = crate::ser::to_bytes(&MyVarlong { my_varlong: *goal }).unwrap();
            assert_eq!(bytes.to_vec(), serialized.as_slice());
        }
    }
}
