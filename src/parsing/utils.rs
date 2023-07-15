use quick_xml::Reader;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use atoi::FromRadix10SignedChecked;
use std::ops::Deref;
use quick_xml::events::attributes::Attributes;
use crate::parsing::{XmlNode, XmlValue, XmlVecNode};

/// [`Reader`] that returns bytes
///
/// Used when XML is read from Entrez or file.
pub type XmlReader<'a> = Reader<&'a [u8]>;

/// Parse the given bytes into an integer
///
/// # Panics
///
/// Panics when [`atoi`] returns `None`
pub fn bytes_to_int<T>(text: &[u8]) -> T
where
    T: FromRadix10SignedChecked,
{
    atoi::atoi::<T>(text.as_ref()).expect("Conversion error")
}

/// Parse the given bytes into a [`String`]
pub fn bytes_to_string(text: &[u8]) -> String {
    text.escape_ascii().to_string()
}

/// parse the given tag for its attributes
pub fn read_attributes<T: XmlValue>(current: &BytesStart) -> Option<T> {
    T::from_attributes(current.html_attributes())
}

pub fn read_bool_attribute(current: &BytesStart) -> Option<bool> {
    if let Some(attributes) = attribute_value(current.html_attributes()) {
        match attributes.as_str() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None
        }
    } else {
        None
    }
}

pub fn attribute_value(attributes: Attributes) -> Option<String> {
    let value = BytesStart::new("value");
    for attribute in attributes {
        if let Ok(attr) = attribute {
            if attr.key == value.name() {
                let _inner = attr.unescape_value().unwrap().to_string();
                let inner = _inner.get(2.._inner.len() - 2).unwrap();
                return Some(inner.to_string())
            }
        }
    }
    return None
}

/// Parses the next [`Event::Text`] as an integer
pub fn read_int<T>(reader: &mut XmlReader) -> Option<T>
where
    T: FromRadix10SignedChecked,
{
    if let Event::Text(text) = reader.read_event().unwrap() {
        Some(bytes_to_int(text.deref()))
    } else {
        None
    }
}

pub fn read_real(reader: &mut XmlReader) -> Option<String> {
    if let Event::Text(text) = reader.read_event().unwrap() {
        bytes_to_string(text.deref()).into()
    } else {
        None
    }
}

/// Parses the next available [`Event::Text`] data as an integer
pub fn read_string(reader: &mut XmlReader) -> Option<String> {
    if let Event::Text(text) = reader.read_event().unwrap() {
        bytes_to_string(text.deref()).into()
    } else {
        None
    }
}

/// Parses the next available XML data as a [`XmlNode`]
pub fn read_node<T: XmlNode>(reader: &mut XmlReader) -> Option<T> {
    T::from_reader(reader)
}

/// Parse each [`BytesText`] within the enclosed element as a [`String`]
///
/// # Parameters
/// - `reader`: [`XmlReader`]
/// - `end`: denotes end of container
///
/// # Returns
/// [`String`] objects contained by `end`
pub fn read_vec_str_unchecked(reader: &mut XmlReader, end: &BytesEnd) -> Vec<String> {
    let mut items = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Text(text) => {
                // remove whitespace
                let text = bytes_to_string(text.deref()).trim().to_string();
                // do not add empty or escape codes
                if is_alphanum(text.as_str()) {
                    items.push(text.to_string())
                }
            }
            Event::End(e) => {
                if e.name() == end.name() {
                    return items;
                }
            }
            _ => (),
        }
    }
}

/// Parse each [`BytesText`] within the enclosed element as an integer
///
/// # Parameters
/// - `reader`: [`XmlReader`]
/// - `end`: denotes end of container
///
/// # Returns
/// Integers contained by `end`
pub fn read_vec_int_unchecked<T>(reader: &mut Reader<&[u8]>, end: &BytesEnd) -> Vec<T>
where
    T: FromRadix10SignedChecked,
{
    let mut nums = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Text(text) => {
                let string = text.deref().escape_ascii().to_string();
                let string = string.trim();
                if is_alphanum(string) {
                    nums.push(bytes_to_int(string.as_bytes()))
                }
            },
            Event::End(e) => {
                if e.name() == end.name() {
                    return nums;
                }
            }
            _ => (),
        }
    }
}

/// Attempt to parse each [`BytesStart`] within the enclosed element as an object
///
/// # Parameters
/// - `reader`: [`XmlReader`]
/// - `end`: denotes end of container
///
/// # Returns
/// Parsed object contained by `end`
pub fn read_vec_node<'a, T: XmlVecNode, E>(reader: &mut Reader<&[u8]>, end: E) -> Vec<T>
where
    E: Into<Option<BytesEnd<'a>>>,
{
    T::vec_from_reader(reader, end)
}

fn is_alphanum(text: &str) -> bool {
    // do not add empty or escape codes
    !(text == "\\\\n" || text.is_empty())
}
