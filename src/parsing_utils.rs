use std::ops::{AddAssign, Deref};
use atoi::{atoi, FromRadix10Checked, FromRadix10SignedChecked};
use num::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Num, One, Zero};
use num::traits::{NumAssign, NumOps};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use crate::{XMLElement, XMLElementVec};

/// [`Reader`] that returns bytes
///
/// Used when XML is read from Entrez or file.
pub type XmlReader<'a> = Reader<&'a [u8]>;

/// Parses a single [`BytesText`] event and sets external variable
///
/// Used for building structs.
///
/// # Arguments
///
/// - `current`: name of current [`BytesStart`]. Used to check if XML data should be parsed.
/// - `element`: start element that encapsulates desired text
/// - `to`: external variable to parse to
/// - `reader`: [`XmlReader`]
pub fn parse_next_string_to<T>(current: &QName, element: &BytesStart, to: &mut T, reader: &mut XmlReader)
where
    T: From<String> {
    if *current == element.name() {
        let text = next_string(reader);
        if text.is_some() {
            *to = text.unwrap().into();
        }
    }
}

/// Parses a single [`BytesText`] event and sets external variable
///
/// Used for building structs.
///
/// # Arguments
///
/// - `current`: name of current [`BytesStart`]. Used to check if XML data should be parsed.
/// - `element`: start element that encapsulates desired value
/// - `to`: external variable to parse integer into
/// - `reader`: [`XmlReader`]
pub fn parse_next_int_to<T>(current: &QName, element: &BytesStart, to: &mut T, reader: &mut XmlReader)
where
T: FromRadix10SignedChecked,
{
    if *current == element.name() {
        let text = next_int(reader);
        if text.is_some() {
            *to = text.unwrap();
        }
    }
}

/// Parses a single [`BytesText`] event and sets external variable
///
/// Used for building structs.
///
/// # Arguments
///
/// - `current`: name of current [`BytesStart`]. Used to check if XML data should be parsed.
/// - `element`: start element that encapsulates desired value
/// - `to`: external variable to parse integer into
/// - `reader`: [`XmlReader`]
pub fn parse_next_int_to_option<T>(current: &QName, element: &BytesStart, to: &mut Option<T>, reader: &mut XmlReader)
    where
        T: FromRadix10SignedChecked,
{
    if *current == element.name() {
        *to = next_int(reader);
    }
}

/// Parse the given bytes into an integer
///
/// # Panics
///
/// Panics when [`atoi`] returns `None`
pub fn to_int<T>(text: &[u8]) -> T
where
    T: FromRadix10SignedChecked
{
    atoi::<T>(text.as_ref()).expect("Conversion error")
}

/// Parse the given bytes into a [`String`]
pub fn to_string(text: &[u8]) -> String {
    text.escape_ascii().to_string()
}

/// Parses the next [`Event::Text`] as an integer
pub fn next_int<T>(reader: &mut XmlReader) -> Option<T>
where
    T: FromRadix10SignedChecked {
    if let Event::Text(text) = reader.read_event().unwrap() {
        Some(to_int(text.deref()))
    }
    else {
        None
    }
}

/// Parses the next [`Event::Text`] as an integer
pub fn next_string(reader: &mut XmlReader) -> Option<String> {
    if let Event::Text(text) = reader.read_event().unwrap() {
        to_string(text.deref()).into()
    }
    else {
        None
    }
}

/// Parse each [`BytesText`] within the enclosed element as a [`String`]
///
/// # Parameters
/// - `reader`: [`XmlReader`]
/// - `end`: denotes end of container
///
/// # Returns
/// [`String`] objects contained by `end`
pub fn parse_vec_str_unchecked(reader: &mut XmlReader, end: &BytesEnd) -> Vec<String>
{
    let mut items = Vec::new();
    loop {
        match reader.read_event().unwrap() {
                Event::Text(text) => {
                    // remove whitespace
                    let text =
                        to_string(text.deref())
                            .trim()
                            .to_string();
                    // do not add empty or escape codes
                    if !(text == "\\\\n" || text.is_empty()) {
                        items.push(text)
                    }
                },
            Event::End(e) => {
                if e.name() == end.name() {
                    return items
                }
            }
            _ => ()
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
pub fn parse_vec_int_unchecked<T>(reader: &mut Reader<&[u8]>, end: &BytesEnd) -> Vec<T>
    where
        T: FromRadix10SignedChecked,
{
    let mut nums = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Text(text) => nums.push(to_int(text.deref())),
            Event::End(e) => {
                if e.name() == end.name() {
                    return nums
                }
            }
            _ => ()
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
pub fn get_vec_node<T, F>(reader: &mut Reader<&[u8]>, item_element: &BytesStart, parser: &F, end: &BytesEnd) -> Vec<T>
    where
        F: Fn(&mut XmlReader) -> Option<T>
{
    let mut items = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(e) => {
                if e.name() == item_element.name() {
                    items.push(parser(reader).unwrap());
                }
            }
            Event::End(e) => {
                if e.name() == end.name() {
                    return items
                }
            }
            _ => ()
        }
    }
}

/// Used for parsing nodes denoted by `element` and setting to an external variable, `to`
pub fn try_node_to<T: XMLElement>(current: &QName, element: &BytesStart, to: &mut T, reader: &mut XmlReader) {
    if *current == element.name() {
        *to = T::from_reader(reader).unwrap()
    }
}

/// Used for parsing vec nodes denoted by `element` and setting to an external variable, `to`
pub fn try_node_to_vec<T: XMLElementVec>(current: &QName, element: &BytesStart, to: &mut Vec<T>, reader: &mut XmlReader) {
    if *current == element.name() {
        *to = T::vec_from_reader(reader, element.to_end())
    }
}

/// Used for parsing nodes denoted by `element` and setting to an external option, `to`
pub fn try_node_to_option<T: XMLElement>(current: &QName, element: &BytesStart, to: &mut Option<T>, reader: &mut XmlReader) {
    if *current == element.name() {
        *to = T::from_reader(reader)
    }
}
