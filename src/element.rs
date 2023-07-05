use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;

pub trait XMLElement {
    fn start_bytes() -> BytesStart<'static>;
    fn from_reader(reader: &mut Reader<&[u8]>) -> Self;

    fn vec_from_reader<'a, E>(reader: &mut Reader<&[u8]>, end: E) -> Vec<Self>
    where
        E: Into<Option<BytesEnd<'a>>>,
        Self: Sized
    {
        let binding = Self::start_bytes();
        let end = end.into().unwrap_or(binding.to_end());
        let mut ids = Vec::new();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == Self::start_bytes().name() {
                        ids.push(Self::from_reader(reader));
                    }
                }
                Event::End(e) => {
                    if e.name() == end.name() {
                        break;
                    }
                }
                _ => (),
            }
        }

        ids
    }
}