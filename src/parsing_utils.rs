use std::ops::{AddAssign, Deref};
use atoi::{atoi, FromRadix10Checked, FromRadix10SignedChecked};
use num::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Num, One, Zero};
use num::traits::{NumAssign, NumOps};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use crate::XMLElement;

/// [`Reader`] that returns bytes
///
/// Used when XML is read from Entrez or file.
pub type XmlReader<'a> = Reader<&'a [u8]>;

/// Parses a single [`BytesText`] event and sets external variable
///
/// # Arguments
///
/// - `current`: name of current [`BytesStart`]. Used to check if XML data should be parsed.
/// - `element`: start element that encapsulates desired text
/// - `to`: external variable to parse to
/// - `reader`: [`XmlReader`]
///
/// # Panics
///
/// Panics when [`try_next_string()`] returns `None`
pub fn parse_next_string_into<T>(current: &QName, element: &BytesStart, to: &mut T, reader: &mut XmlReader)
where
    T: From<String> {
    if *current == element.name() {
        let text = try_next_string(reader);
        if text.is_some() {
            *to = text.unwrap().into();
        }
    }
}

/// Parse the given bytes into an integer
///
/// # Panics
///
/// Panics when [`atoi`] returns `None`
fn parse_int<T>(text: &[u8]) -> T
where
    T: FromRadix10SignedChecked
{
    atoi::<T>(text.as_ref()).expect("Conversion error")
}

/// Parse the given bytes into a [`String`]
fn parse_string(text: &[u8]) -> String {
    text.escape_ascii().to_string()
}

/// Parses the next [`Event::Text`] as an integer
pub fn try_next_int<T>(reader: &mut XmlReader) -> Option<T>
where
    T: FromRadix10SignedChecked {
    if let Event::Text(text) = reader.read_event().unwrap() {
        Some(parse_int(text.deref()))
    }
    else {
        None
    }
}

/// Parses the next [`Event::Text`] as an integer
pub fn try_next_string(reader: &mut XmlReader) -> Option<String> {
    if let Event::Text(text) = reader.read_event().unwrap() {
        parse_string(text.deref()).into()
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
                        parse_string(text.deref())
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
            Event::Text(text) => nums.push(parse_int(text.deref())),
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
        F: Fn(&mut Reader<&[u8]>) -> Option<T>
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