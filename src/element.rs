use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::events::attributes::Attributes;
use quick_xml::Reader;

/// Parses [`Event::Empty`] values
///
/// Differs from [`XmlNode`] in that
/// [`Self::from_attributes`] accepts [`Attributes`],
/// whereas [`XmlNode::from_reader()`] accepts an [`XMLReader`]
pub trait XmlValue {
    fn start_bytes() -> BytesStart<'static>;
    fn from_attributes(bytes: Attributes) -> Option<Self> where Self: Sized;
}

pub trait XmlNode {
    fn start_bytes() -> BytesStart<'static>;
    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized;

    fn is_end(element: &BytesEnd) -> bool {
        element.name() == Self::start_bytes().name()
    }
}

pub trait XmlVecNode: XmlNode {
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
