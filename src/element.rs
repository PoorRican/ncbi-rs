use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::events::attributes::Attributes;
use quick_xml::Reader;

/// Handles parsing of simple data from [`Event::Empty`] values
///
/// An empty value is used in Entrez output to typically to enclose boolean values.
/// An empty tag named `TagName` is denoted by `<TagName value="{value}" />`, where
/// the enclosed value is `{value}` denoted by the "value" attribute name. In the case of
/// boolean values, `true` is denoted by "true"; `false` by "false".
///
/// [`XmlValue`] differs from [`XmlNode`] in that it's main reader function
/// [`Self::from_attributes`] accepts [`Attributes`], whereas [`XmlNode::from_reader()`]
/// parses raw bytes from [`XMLReader`].
pub trait XmlValue {
    fn start_bytes() -> BytesStart<'static>;
    fn from_attributes(bytes: Attributes) -> Option<Self> where Self: Sized;
}

/// Contains methods for parsing XML data
///
/// A struct or enum that implements [`XmlNode`] is able to be parsed by XML data.
///
/// ## How ASN.1 types should be parsed
///
/// Two types of values parsed by `XmlNode`: enums and structs.
/// Both field/variant name/value have tags to separate between the field/variant and the
/// enclosed value. Decision what should be parsed is denoted by [`Self::start_bytes()`].
pub trait XmlNode {

    /// Return starting element
    ///
    /// This function should be used my implementations of [`XmlNode::from_reader()`]
    /// to signify the starting and stopping of parsed data and to distinguish between
    /// other [`XmlNode`]'s.
    ///
    /// Element tags are all hardcoded and do not yet take into account namespaces. There
    /// is no simple, idiomatic way to gracefully implement this. Therefore, changing
    /// this is not planned on being implemented especially since the ASN.1 data format
    /// is not likely to change much.
    fn start_bytes() -> BytesStart<'static>;

    /// Process the XML data as `Self`
    ///
    /// This function is called once [`Self::start_bytes()`] has been encountered,
    /// so implementations should assume that the data is [`Self`] therefore
    /// field or variant values must begin being parsed.
    ///
    /// Parsed struct (or `None` in the case of an enum) should be returned upon
    /// encountering a closing tag derived from [`Self::start_bytes()`]. This halts
    /// consumption of data from `reader` so that remaining data can be consumed by
    /// subsequent parsing functions.
    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized;

    fn is_end(element: &BytesEnd) -> bool {
        element.name() == Self::start_bytes().name()
    }
}

/// An element with the [`XmlVecNode`] trait is able to be parsed into a [`Vec`].
pub trait XmlVecNode: XmlNode {

    /// Parse XML data as multiple [`XmlNode`] objects contained by [`Vec`]
    ///
    /// [`Self::vec_from_reader()`] (or the helper function [`crate::parsing_utils::read_vec_node()`])
    /// should be used to parse multiple [`XmlNode`] objects into a vector at once. Using these helper
    /// functions reduces errors in parsing implementation.
    fn vec_from_reader<'a, E>(reader: &mut Reader<&[u8]>, end: E) -> Vec<Self>
    where
        E: Into<Option<BytesEnd<'a>>>,
        Self: Sized,
    {
        let binding = Self::start_bytes();
        let end = end.into().unwrap_or(binding.to_end());
        let mut items = Vec::new();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == Self::start_bytes().name() {
                        if let Some(val) = Self::from_reader(reader) {
                            items.push(val);
                        }
                    }
                }
                Event::End(e) => {
                    if e.name() == end.name() {
                        break;
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => (),
            }
        }

        items
    }
}
