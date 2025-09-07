use std::fmt::Debug;

use prost::encoding::{WireType, MIN_TAG};
use prost::{bytes::Buf, DecodeError, Enumeration, Message};

#[derive(Clone, PartialEq, Message)]
struct Person {
    #[prost(string, tag = "1")]
    pub id: String, // tag=1
    // NOTE: Old "name" field has been removed
    // pub name: String, // tag=2 (Removed)
    #[prost(string, tag = "6")]
    pub given_name: String, // tag=6
    #[prost(string, repeated)]
    pub middle_names: Vec<String>, // tag=7
    #[prost(uint32, repeated)]
    pub digits: Vec<u32>, // tag=8
    #[prost(enumeration = "Gender")]
    pub gender: i32, // tag=9
    #[prost(message, repeated)]
    pub friends: Vec<Person>, // tag=10
}
fn decode_varint_slow(buf: &[u8]) -> Result<(u64, usize), DecodeError> {
    let mut value = 0;
    let mut idx = 0;
    for count in 0..core::cmp::min(10, buf.remaining()) {
        let byte = buf[idx];
        value |= u64::from(byte & 0x7F) << (count * 7);
        if byte <= 0x7F {
            // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
            // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
            if count == 9 && byte >= 0x02 {
                return Err(DecodeError::new("invalid varint"));
            } else {
                return Ok((value, idx));
            }
        }
        idx += 1;
    }

    Err(DecodeError::new("invalid varint"))
}

fn decode_varint_slice(bytes: &[u8]) -> Result<(u64, usize), DecodeError> {
    // Fully unrolled varint decoding loop. Splitting into 32-bit pieces gives better performance.

    // Use assertions to ensure memory safety, but it should always be optimized after inline.
    assert!(!bytes.is_empty());
    assert!(bytes.len() > 10 || bytes[bytes.len() - 1] < 0x80);

    let mut b: u8 = unsafe { *bytes.get_unchecked(0) };
    let mut part0: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((u64::from(part0), 1));
    };
    part0 -= 0x80;
    b = unsafe { *bytes.get_unchecked(1) };
    part0 += u32::from(b) << 7;
    if b < 0x80 {
        return Ok((u64::from(part0), 2));
    };
    part0 -= 0x80 << 7;
    b = unsafe { *bytes.get_unchecked(2) };
    part0 += u32::from(b) << 14;
    if b < 0x80 {
        return Ok((u64::from(part0), 3));
    };
    part0 -= 0x80 << 14;
    b = unsafe { *bytes.get_unchecked(3) };
    part0 += u32::from(b) << 21;
    if b < 0x80 {
        return Ok((u64::from(part0), 4));
    };
    part0 -= 0x80 << 21;
    let value = u64::from(part0);

    b = unsafe { *bytes.get_unchecked(4) };
    let mut part1: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 5));
    };
    part1 -= 0x80;
    b = unsafe { *bytes.get_unchecked(5) };
    part1 += u32::from(b) << 7;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 6));
    };
    part1 -= 0x80 << 7;
    b = unsafe { *bytes.get_unchecked(6) };
    part1 += u32::from(b) << 14;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 7));
    };
    part1 -= 0x80 << 14;
    b = unsafe { *bytes.get_unchecked(7) };
    part1 += u32::from(b) << 21;
    if b < 0x80 {
        return Ok((value + (u64::from(part1) << 28), 8));
    };
    part1 -= 0x80 << 21;
    let value = value + ((u64::from(part1)) << 28);

    b = unsafe { *bytes.get_unchecked(8) };
    let mut part2: u32 = u32::from(b);
    if b < 0x80 {
        return Ok((value + (u64::from(part2) << 56), 9));
    };
    part2 -= 0x80;
    b = unsafe { *bytes.get_unchecked(9) };
    part2 += u32::from(b) << 7;
    // Check for u64::MAX overflow. See [`ConsumeVarint`][1] for details.
    // [1]: https://github.com/protocolbuffers/protobuf-go/blob/v1.27.1/encoding/protowire/wire.go#L358
    if b < 0x02 {
        return Ok((value + (u64::from(part2) << 56), 10));
    };

    // We have overrun the maximum size of a varint (10 bytes) or the final byte caused an overflow.
    // Assume the data is corrupt.
    Err(DecodeError::new("invalid varint"))
}

pub fn decode_varint(buf: &[u8]) -> Result<(u64, usize), DecodeError> {
    let bytes = buf.chunk();
    let len = bytes.len();
    if len == 0 {
        return Err(DecodeError::new("invalid varint"));
    }

    let byte = bytes[0];
    if byte < 0x80 {
        // buf.advance(1);
        Ok((u64::from(byte), 1))
    } else if len > 10 || bytes[len - 1] < 0x80 {
        let (value, advance) = decode_varint_slice(bytes)?;
        // buf.advance(advance);
        Ok((value, advance))
    } else {
        decode_varint_slow(buf)
    }
}

pub fn decode_key(buf: &[u8]) -> Result<(u32, WireType, usize), DecodeError> {
    let (key, offset) = decode_varint(buf)?;
    if key > u64::from(u32::MAX) {
        return Err(DecodeError::new(format!("invalid key value: {}", key)));
    }
    let wire_type = WireType::try_from(key & 0x07)?;
    let tag = key as u32 >> 3;

    if tag < MIN_TAG {
        return Err(DecodeError::new("invalid tag value: 0"));
    }

    Ok((tag, wire_type, offset))
}

pub fn decode_length_delimiter(buf: &[u8]) -> Result<(usize, usize), DecodeError> {
    let (length, offset) = decode_varint(buf)?;
    if length > usize::MAX as u64 {
        return Err(DecodeError::new(
            "length delimiter exceeds maximum usize value",
        ));
    }
    Ok((length as usize, offset))
}

pub fn parse_str(buf: &[u8], offset: usize) -> Result<&str, DecodeError> {
    let (length, length_offset) = decode_length_delimiter(&buf[offset..])?;
    let start = offset + length_offset;
    std::str::from_utf8(&buf[start..(start + length)]).map_err(|e| DecodeError::new(e.to_string()))
}

pub fn parse_repeating(
    buf: &[u8],
    original_offset: usize,
) -> Result<(Vec<usize>, usize), DecodeError> {
    let mut offset = original_offset;

    let (length, length_offset) = decode_length_delimiter(&buf[offset..])?;
    offset += length_offset;

    let mut res = Vec::with_capacity(length);

    for _ in 0..length {
        let (_, length_offset) = decode_length_delimiter(&buf[offset..])?;
        res.push(offset);
        offset += length_offset;
    }

    Ok((res, offset - original_offset))
}

#[derive(Default)]
pub struct PersonObserver<'a> {
    inner: &'a [u8],
    id: Option<usize>,
    given_name: Option<usize>,
    // Sized repeating fields that are not packed
    // I feel like there is no way to do this...
    // without needing to iterate over the entire buffer again...
    middle_names: Vec<usize>,
    // A packed repeating field
    digits: Option<usize>,
    gender: Gender,
    friends: Vec<usize>,
}

impl<'a> Debug for PersonObserver<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Person")
            .field("id", &self.id())
            .field("given_name", &self.given_name())
            .field("middle_names", &self.middle_names())
            .field("digits", &self.digits())
            .field("gender", &self.gender)
            .field("friends", &self.friends())
            .finish()
    }
}

impl<'a> PersonObserver<'a> {
    pub fn new(buf: &'a [u8]) -> Result<Self, DecodeError> {
        let inner = buf;
        let mut offset = 0;

        let mut res = Self {
            inner,
            ..Default::default()
        };

        while offset < buf.len() {
            let (tag, _, next_offset) = decode_key(&buf[offset..])?;

            if tag == 1 {
                offset += next_offset;
                res.id.replace(offset);
                let (length, delim_offset) = decode_length_delimiter(&buf[offset..])?;
                offset += delim_offset + length;
            }

            if tag == 6 {
                offset += next_offset;
                res.given_name.replace(offset);
                let (length, delim_offset) = decode_length_delimiter(&buf[offset..])?;
                offset += delim_offset + length;
            }

            if tag == 7 {
                offset += next_offset;
                res.middle_names.push(offset);
                let (length, delim_offset) = decode_length_delimiter(&buf[offset..])?;
                offset += delim_offset + length;
            }

            if tag == 8 {
                offset += next_offset;
                res.digits.replace(offset);
                let (_, shift) = parse_repeating(buf, offset)?;
                offset += shift;
            }

            if tag == 9 {
                offset += next_offset;
                let (r, shift) = decode_varint(&buf[offset..])?;
                res.gender =
                    Gender::try_from(r as i32).map_err(|e| DecodeError::new(e.to_string()))?;
                offset += shift;
            }

            if tag == 10 {
                offset += next_offset;
                res.friends.push(offset);
                let (length, delim_offset) = decode_length_delimiter(&buf[offset..])?;
                offset += delim_offset + length;
            }
        }

        Ok(res)
    }
    pub fn id(&'a self) -> Option<&'a str> {
        self.id.map(|p| parse_str(self.inner, p).unwrap())
    }

    pub fn given_name(&'a self) -> Option<&'a str> {
        self.given_name.map(|p| parse_str(self.inner, p).unwrap())
    }

    pub fn middle_names(&'a self) -> Vec<&'a str> {
        self.middle_names
            .iter()
            .map(|p| parse_str(self.inner, *p).unwrap())
            .collect()
    }

    pub fn digits(&'a self) -> Vec<u32> {
        if let Some(offset) = self.digits {
            let (v, _) = parse_repeating(self.inner, offset).unwrap();
            v.iter()
                .map(|p| decode_varint(&self.inner[*p..]).unwrap().0 as u32)
                .collect()
        } else {
            Vec::with_capacity(0)
        }
    }

    pub fn gender(&'a self) -> Gender {
        self.gender
    }

    pub fn friends(&'a self) -> Vec<PersonObserver<'a>> {
        self.friends
            .iter()
            .map(|p| {
                let (length, shift) = decode_length_delimiter(&self.inner[*p..]).unwrap();
                let start = *p + shift;
                PersonObserver::new(&self.inner[start..(start + length)]).unwrap()
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
pub enum Gender {
    Unknown = 0,
    Female = 1,
    Male = 2,
}

fn main() {
    let p2 = Person {
        id: "apple".into(),
        given_name: "Steve".into(),
        middle_names: vec!["Paul".into()],
        digits: vec![3, 4, 5],
        gender: Gender::Male as i32,
        ..Default::default()
    };

    let p3 = Person {
        id: "microsoft".into(),
        given_name: "William".into(),
        middle_names: vec!["Henry".into()],
        digits: vec![6, 7, 8],
        gender: Gender::Male as i32,
        ..Default::default()
    };

    let p = Person {
        id: "anduril".into(),
        given_name: "Palmer".into(),
        middle_names: vec!["Freeman".into()],
        digits: vec![1, 2, 3],
        gender: Gender::Male as i32,
        friends: vec![p2, p3]
    };

    let bytes = p.encode_to_vec();

    let observer = PersonObserver::new(&bytes).unwrap();

    println!("{:?}", observer);
}
