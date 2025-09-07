#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use prost::encoding::{WireType, MIN_TAG};
use prost::{bytes::Buf, DecodeError, Enumeration, Message};
struct Person {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "6")]
    pub given_name: String,
    #[prost(string, repeated)]
    pub middle_names: Vec<String>,
    #[prost(string)]
    pub formatted_name: String,
    #[prost(uint32, tag = "3")]
    pub age: u32,
    #[prost(uint32)]
    pub height: u32,
    #[prost(enumeration = "Gender")]
    pub gender: i32,
    #[prost(string, tag = "16")]
    pub name_prefix: String,
    #[prost(string)]
    pub name_suffix: String,
    #[prost(string)]
    pub maiden_name: String,
}
#[automatically_derived]
impl ::core::clone::Clone for Person {
    #[inline]
    fn clone(&self) -> Person {
        Person {
            id: ::core::clone::Clone::clone(&self.id),
            given_name: ::core::clone::Clone::clone(&self.given_name),
            middle_names: ::core::clone::Clone::clone(&self.middle_names),
            formatted_name: ::core::clone::Clone::clone(&self.formatted_name),
            age: ::core::clone::Clone::clone(&self.age),
            height: ::core::clone::Clone::clone(&self.height),
            gender: ::core::clone::Clone::clone(&self.gender),
            name_prefix: ::core::clone::Clone::clone(&self.name_prefix),
            name_suffix: ::core::clone::Clone::clone(&self.name_suffix),
            maiden_name: ::core::clone::Clone::clone(&self.maiden_name),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Person {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Person {
    #[inline]
    fn eq(&self, other: &Person) -> bool {
        self.age == other.age
            && self.height == other.height
            && self.gender == other.gender
            && self.id == other.id
            && self.given_name == other.given_name
            && self.middle_names == other.middle_names
            && self.formatted_name == other.formatted_name
            && self.name_prefix == other.name_prefix
            && self.name_suffix == other.name_suffix
            && self.maiden_name == other.maiden_name
    }
}
impl ::prost::Message for Person {
    #[allow(unused_variables)]
    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
        if self.id != "" {
            ::prost::encoding::string::encode(1u32, &self.id, buf);
        }
        if self.age != 0u32 {
            ::prost::encoding::uint32::encode(3u32, &self.age, buf);
        }
        if self.height != 0u32 {
            ::prost::encoding::uint32::encode(4u32, &self.height, buf);
        }
        if self.gender != Gender::default() as i32 {
            ::prost::encoding::int32::encode(5u32, &self.gender, buf);
        }
        if self.given_name != "" {
            ::prost::encoding::string::encode(6u32, &self.given_name, buf);
        }
        ::prost::encoding::string::encode_repeated(7u32, &self.middle_names, buf);
        if self.formatted_name != "" {
            ::prost::encoding::string::encode(8u32, &self.formatted_name, buf);
        }
        if self.name_prefix != "" {
            ::prost::encoding::string::encode(16u32, &self.name_prefix, buf);
        }
        if self.name_suffix != "" {
            ::prost::encoding::string::encode(17u32, &self.name_suffix, buf);
        }
        if self.maiden_name != "" {
            ::prost::encoding::string::encode(18u32, &self.maiden_name, buf);
        }
    }
    #[allow(unused_variables)]
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::wire_type::WireType,
        buf: &mut impl ::prost::bytes::Buf,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError> {
        const STRUCT_NAME: &'static str = "Person";
        match tag {
            1u32 => {
                let mut value = &mut self.id;
                ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "id");
                    error
                })
            }
            3u32 => {
                let mut value = &mut self.age;
                ::prost::encoding::uint32::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "age");
                    error
                })
            }
            4u32 => {
                let mut value = &mut self.height;
                ::prost::encoding::uint32::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "height");
                    error
                })
            }
            5u32 => {
                let mut value = &mut self.gender;
                ::prost::encoding::int32::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "gender");
                    error
                })
            }
            6u32 => {
                let mut value = &mut self.given_name;
                ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "given_name");
                    error
                })
            }
            7u32 => {
                let mut value = &mut self.middle_names;
                ::prost::encoding::string::merge_repeated(wire_type, value, buf, ctx).map_err(
                    |mut error| {
                        error.push(STRUCT_NAME, "middle_names");
                        error
                    },
                )
            }
            8u32 => {
                let mut value = &mut self.formatted_name;
                ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "formatted_name");
                    error
                })
            }
            16u32 => {
                let mut value = &mut self.name_prefix;
                ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "name_prefix");
                    error
                })
            }
            17u32 => {
                let mut value = &mut self.name_suffix;
                ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "name_suffix");
                    error
                })
            }
            18u32 => {
                let mut value = &mut self.maiden_name;
                ::prost::encoding::string::merge(wire_type, value, buf, ctx).map_err(|mut error| {
                    error.push(STRUCT_NAME, "maiden_name");
                    error
                })
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }
    #[inline]
    fn encoded_len(&self) -> usize {
        0 + if self.id != "" {
            ::prost::encoding::string::encoded_len(1u32, &self.id)
        } else {
            0
        } + if self.age != 0u32 {
            ::prost::encoding::uint32::encoded_len(3u32, &self.age)
        } else {
            0
        } + if self.height != 0u32 {
            ::prost::encoding::uint32::encoded_len(4u32, &self.height)
        } else {
            0
        } + if self.gender != Gender::default() as i32 {
            ::prost::encoding::int32::encoded_len(5u32, &self.gender)
        } else {
            0
        } + if self.given_name != "" {
            ::prost::encoding::string::encoded_len(6u32, &self.given_name)
        } else {
            0
        } + ::prost::encoding::string::encoded_len_repeated(7u32, &self.middle_names)
            + if self.formatted_name != "" {
                ::prost::encoding::string::encoded_len(8u32, &self.formatted_name)
            } else {
                0
            }
            + if self.name_prefix != "" {
                ::prost::encoding::string::encoded_len(16u32, &self.name_prefix)
            } else {
                0
            }
            + if self.name_suffix != "" {
                ::prost::encoding::string::encoded_len(17u32, &self.name_suffix)
            } else {
                0
            }
            + if self.maiden_name != "" {
                ::prost::encoding::string::encoded_len(18u32, &self.maiden_name)
            } else {
                0
            }
    }
    fn clear(&mut self) {
        self.id.clear();
        self.age = 0u32;
        self.height = 0u32;
        self.gender = Gender::default() as i32;
        self.given_name.clear();
        self.middle_names.clear();
        self.formatted_name.clear();
        self.name_prefix.clear();
        self.name_suffix.clear();
        self.maiden_name.clear();
    }
}
impl ::core::default::Default for Person {
    fn default() -> Self {
        Person {
            id: ::prost::alloc::string::String::new(),
            age: 0u32,
            height: 0u32,
            gender: Gender::default() as i32,
            given_name: ::prost::alloc::string::String::new(),
            middle_names: ::prost::alloc::vec::Vec::new(),
            formatted_name: ::prost::alloc::string::String::new(),
            name_prefix: ::prost::alloc::string::String::new(),
            name_suffix: ::prost::alloc::string::String::new(),
            maiden_name: ::prost::alloc::string::String::new(),
        }
    }
}
impl ::core::fmt::Debug for Person {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let mut builder = f.debug_struct("Person");
        let builder = {
            let wrapper = {
                #[allow(non_snake_case)]
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.id)
            };
            builder.field("id", &wrapper)
        };
        let builder = {
            let wrapper = {
                #[allow(non_snake_case)]
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.given_name)
            };
            builder.field("given_name", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(
                    &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                );
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let mut vec_builder = f.debug_list();
                        for v in self.0 {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            vec_builder.entry(&Inner(v));
                        }
                        vec_builder.finish()
                    }
                }
                ScalarWrapper(&self.middle_names)
            };
            builder.field("middle_names", &wrapper)
        };
        let builder = {
            let wrapper = {
                #[allow(non_snake_case)]
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.formatted_name)
            };
            builder.field("formatted_name", &wrapper)
        };
        let builder = {
            let wrapper = {
                #[allow(non_snake_case)]
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.age)
            };
            builder.field("age", &wrapper)
        };
        let builder = {
            let wrapper = {
                #[allow(non_snake_case)]
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.height)
            };
            builder.field("height", &wrapper)
        };
        let builder = {
            let wrapper = {
                struct ScalarWrapper<'a>(&'a i32);
                impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        let res: ::core::result::Result<Gender, _> =
                            ::core::convert::TryFrom::try_from(*self.0);
                        match res {
                            Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                            Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                        }
                    }
                }
                ScalarWrapper(&self.gender)
            };
            builder.field("gender", &wrapper)
        };
        let builder = {
            let wrapper = {
                #[allow(non_snake_case)]
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name_prefix)
            };
            builder.field("name_prefix", &wrapper)
        };
        let builder = {
            let wrapper = {
                #[allow(non_snake_case)]
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.name_suffix)
            };
            builder.field("name_suffix", &wrapper)
        };
        let builder = {
            let wrapper = {
                #[allow(non_snake_case)]
                fn ScalarWrapper<T>(v: T) -> T {
                    v
                }
                ScalarWrapper(&self.maiden_name)
            };
            builder.field("maiden_name", &wrapper)
        };
        builder.finish()
    }
}
#[allow(dead_code)]
impl Person {
    ///Returns the enum value of `gender`, or the default if the field is set to an invalid enum value.
    pub fn gender(&self) -> Gender {
        ::core::convert::TryFrom::try_from(self.gender).unwrap_or(Gender::default())
    }
    ///Sets `gender` to the provided enum value.
    pub fn set_gender(&mut self, value: Gender) {
        self.gender = value as i32;
    }
}
fn decode_varint_slow(buf: &[u8]) -> Result<(u64, usize), DecodeError> {
    let mut value = 0;
    let mut idx = 0;
    for count in 0..core::cmp::min(10, buf.remaining()) {
        let byte = buf[idx];
        value |= u64::from(byte & 0x7F) << (count * 7);
        if byte <= 0x7F {
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
    {
        match !bytes.is_empty() {
            true => {}
            _ => ::core::panicking::panic("assertion failed: !bytes.is_empty()"),
        }
    };
    {
        match bytes.len() > 10 || bytes[bytes.len() - 1] < 0x80 {
            true => {}
            _ => ::core::panicking::panic(
                "assertion failed: bytes.len() > 10 || bytes[bytes.len() - 1] < 0x80",
            ),
        }
    };
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
    if b < 0x02 {
        return Ok((value + (u64::from(part2) << 56), 10));
    };
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
        Ok((u64::from(byte), 1))
    } else if len > 10 || bytes[len - 1] < 0x80 {
        let (value, advance) = decode_varint_slice(bytes)?;
        Ok((value, advance))
    } else {
        decode_varint_slow(buf)
    }
}
pub fn decode_key(buf: &[u8]) -> Result<(u32, WireType, usize), DecodeError> {
    let (key, offset) = decode_varint(buf)?;
    if key > u64::from(u32::MAX) {
        return Err(DecodeError::new(::alloc::__export::must_use({
            ::alloc::fmt::format(format_args!("invalid key value: {0}", key))
        })));
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
    {
        ::std::io::_print(format_args!("{0:?}\n", offset));
    };
    let (length, length_offset) = decode_length_delimiter(&buf[offset..])?;
    offset += length_offset;
    {
        ::std::io::_print(format_args!("{0:?}\n", offset));
    };
    let mut res = Vec::with_capacity(length);
    for _ in 0..length {
        let (l, length_offset) = decode_length_delimiter(&buf[offset..])?;
        res.push(offset);
        {
            ::std::io::_print(format_args!("{0:?}\n", (length_offset, l)));
        };
        offset += length_offset + l;
        {
            ::std::io::_print(format_args!("{0:?}\n", offset));
        };
    }
    Ok((res, offset))
}
pub struct PersonObserver<'a> {
    inner: &'a [u8],
    id: Option<usize>,
    given_name: Option<usize>,
    middle_names: Option<usize>,
}
#[automatically_derived]
impl<'a> ::core::default::Default for PersonObserver<'a> {
    #[inline]
    fn default() -> PersonObserver<'a> {
        PersonObserver {
            inner: ::core::default::Default::default(),
            id: ::core::default::Default::default(),
            given_name: ::core::default::Default::default(),
            middle_names: ::core::default::Default::default(),
        }
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
                res.middle_names.replace(offset);
                {
                    ::std::io::_print(format_args!("{0:?}\n", &buf[offset..]));
                };
                let (_, total_offset) = parse_repeating(buf, offset)?;
                offset += total_offset;
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
    pub fn middle_names(&'a self) -> Option<Vec<&'a str>> {
        self.middle_names.map(|p| {
            parse_repeating(self.inner, p)
                .unwrap()
                .0
                .into_iter()
                .map(|p| parse_str(self.inner, p).unwrap())
                .collect()
        })
    }
}
pub enum Gender {
    Unknown = 0,
    Female = 1,
    Male = 2,
}
#[automatically_derived]
impl ::core::clone::Clone for Gender {
    #[inline]
    fn clone(&self) -> Gender {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Gender {}
#[automatically_derived]
impl ::core::fmt::Debug for Gender {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Gender::Unknown => "Unknown",
                Gender::Female => "Female",
                Gender::Male => "Male",
            },
        )
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Gender {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Gender {
    #[inline]
    fn eq(&self, other: &Gender) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Gender {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
impl Gender {
    ///Returns `true` if `value` is a variant of `Gender`.
    pub fn is_valid(value: i32) -> bool {
        match value {
            0 => true,
            1 => true,
            2 => true,
            _ => false,
        }
    }
    #[deprecated = "Use the TryFrom<i32> implementation instead"]
    ///Converts an `i32` to a `Gender`, or `None` if `value` is not a valid variant.
    pub fn from_i32(value: i32) -> ::core::option::Option<Gender> {
        match value {
            0 => ::core::option::Option::Some(Gender::Unknown),
            1 => ::core::option::Option::Some(Gender::Female),
            2 => ::core::option::Option::Some(Gender::Male),
            _ => ::core::option::Option::None,
        }
    }
}
impl ::core::default::Default for Gender {
    fn default() -> Gender {
        Gender::Unknown
    }
}
impl ::core::convert::From<Gender> for i32 {
    fn from(value: Gender) -> i32 {
        value as i32
    }
}
impl ::core::convert::TryFrom<i32> for Gender {
    type Error = ::prost::UnknownEnumValue;
    fn try_from(value: i32) -> ::core::result::Result<Gender, ::prost::UnknownEnumValue> {
        match value {
            0 => ::core::result::Result::Ok(Gender::Unknown),
            1 => ::core::result::Result::Ok(Gender::Female),
            2 => ::core::result::Result::Ok(Gender::Male),
            _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
        }
    }
}
fn main() {
    let p = Person {
        id: "foobar".into(),
        given_name: "Benjamin".into(),
        middle_names: <[_]>::into_vec(::alloc::boxed::box_new(["Joseph".into(), "Joe".into()])),
        ..Default::default()
    };
    let bytes = p.encode_to_vec();
    let observer = PersonObserver::new(&bytes).unwrap();
    {
        ::std::io::_print(format_args!(
            "{0:?} {1:?}\n",
            observer.id(),
            observer.given_name()
        ));
    };
}
