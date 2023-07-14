use crate::{XmlNode, XmlVecNode, XmlValue};
use atoi::{atoi, FromRadix10SignedChecked};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use std::ops::Deref;

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
pub fn parse_string_to<T>(current: &QName, element: &BytesStart, to: &mut T, reader: &mut XmlReader)
where
    T: From<String>,
{
    if *current == element.name() {
        let text = read_string(reader);
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
pub fn parse_int_to<T>(current: &QName, element: &BytesStart, to: &mut T, reader: &mut XmlReader)
where
    T: FromRadix10SignedChecked,
{
    if *current == element.name() {
        let text = read_int(reader);
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
pub fn parse_int_to_option<T>(
    current: &QName,
    element: &BytesStart,
    to: &mut Option<T>,
    reader: &mut XmlReader,
) where
    T: FromRadix10SignedChecked,
{
    if *current == element.name() {
        *to = read_int(reader);
    }
}

/// Parse the given bytes into an integer
///
/// # Panics
///
/// Panics when [`atoi`] returns `None`
pub fn bytes_to_int<T>(text: &[u8]) -> T
where
    T: FromRadix10SignedChecked,
{
    atoi::<T>(text.as_ref()).expect("Conversion error")
}

/// Parse the given bytes into a [`String`]
pub fn bytes_to_string(text: &[u8]) -> String {
    text.escape_ascii().to_string()
}

pub fn parse_attribute<T: XmlValue>(current: &BytesStart, element: &BytesStart) -> Option<T> {
    if current.name() == element.name() {
        T::from_attributes(current.html_attributes())
    } else {
        None
    }
}

pub fn parse_attribute_to<T: XmlValue>(current: &BytesStart, element: &BytesStart, to: &mut T){
    let value = parse_attribute(current, element);
    if value.is_some() {
        *to = value.unwrap()
    }
}

pub fn parse_attribute_to_option<T: XmlValue>(current: &BytesStart, element: &BytesStart, to: &mut Option<T>){
    let value = parse_attribute(current, element);
    if value.is_some() {
        *to = value
    }
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

/// Parses the next [`Event::Text`] as an integer
pub fn read_string(reader: &mut XmlReader) -> Option<String> {
    if let Event::Text(text) = reader.read_event().unwrap() {
        bytes_to_string(text.deref()).into()
    } else {
        None
    }
}

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
                if !(text == "\\\\n" || text.is_empty()) {
                    items.push(text)
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
            Event::Text(text) => nums.push(bytes_to_int(text.deref())),
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
pub fn parse_vec_node<'a, T: XmlVecNode, E>(reader: &mut Reader<&[u8]>, end: E) -> Vec<T>
where
    E: Into<Option<BytesEnd<'a>>>,
{
    T::vec_from_reader(reader, end)
}

/// Used for parsing nodes denoted by `element` and setting to an external variable, `to`
pub fn parse_node_to<T: XmlNode>(
    current: &QName,
    element: &BytesStart,
    to: &mut T,
    reader: &mut XmlReader,
) {
    if *current == element.name() {
        if let Some(node) = read_node(reader) {
            *to = node
        }
    }
}

/// Used for parsing vec nodes denoted by `element` and setting to an external variable, `to`
pub fn parse_vec_node_to<T: XmlVecNode>(
    current: &QName,
    element: &BytesStart,
    to: &mut Vec<T>,
    reader: &mut XmlReader,
) {
    if *current == element.name() {
        *to = parse_vec_node(reader, element.to_end())
    }
}

/// Used for parsing nodes denoted by `element` and setting to an external option, `to`
pub fn parse_node_to_option<T: XmlNode>(
    current: &QName,
    element: &BytesStart,
    to: &mut Option<T>,
    reader: &mut XmlReader,
) {
    if *current == element.name() {
        let node = read_node(reader);
        if node.is_some() {
            *to = node
        }
    }
}

pub fn parse_vec_node_to_option<T: XmlVecNode>(
    current: &QName,
    element: &BytesStart,
    to: &mut Option<Vec<T>>,
    reader: &mut XmlReader,
) {
    if *current == element.name() {
        *to = parse_vec_node(reader, element.to_end()).into()
    }
}

pub fn check_unimplemented(current: &QName, forbidden: &[&BytesStart<'static>]) {
    for tag in forbidden.iter() {
        if *current == tag.name() {

            eprintln!("Encountered XML tag {}, which has not been implemented yet...", tag.escape_ascii().to_string())
        }
    }
}