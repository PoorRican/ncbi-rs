use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::parsing::{read_int, read_vec_int_unchecked, XmlNode};

#[derive(Default, Debug, Clone, PartialEq)]
/// The following tags have not been implemented:
/// - `TranslationSet`
/// - `TranslationStack`
/// - `QueryTranslation`
///
/// These terms have not been implemented because they deal with internal
/// DB relationships. This level of refinement is accomplished by [`ESearch`]
/// builder functions.
pub struct ESearchResult {
    count: u64,
    ret_max: u64,
    ret_start: u64,
    id_list: Vec<u64>,
}

impl XmlNode for ESearchResult {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("eSearchResult")
    }

    /// Unknown tags are explicitly not caught by [`UnexpectedTags`]. This
    /// record is only used to fetch biological data which requires ID values.
    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut result = Self::default();

        let _binding = Self::start_bytes();

        let count_tag = BytesStart::new("Count");
        let ret_max_tag = BytesStart::new("RetMax");
        let ret_start_tag = BytesStart::new("RetStart");
        let id_list_tag = BytesStart::new("IdList");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == count_tag.name() {
                        result.count = read_int(reader).unwrap();
                    } else if name == ret_max_tag.name() {
                        result.ret_max = read_int(reader).unwrap();
                    } else if name == ret_start_tag.name() {
                        result.ret_start = read_int(reader).unwrap();
                    } else if name == id_list_tag.name() {
                        result.id_list = read_vec_int_unchecked(reader, &id_list_tag.to_end())
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return Some(result)
                    }
                }
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::Reader;
    use crate::{ESearchResult, get_local_xml};
    use crate::parsing::XmlNode;

    const DATA1: &str = "tests/data/deaminase_protein_search.xml";

    #[test]
    fn read_from_xml() {
        let xml = get_local_xml(DATA1).unwrap();
        let mut reader = Reader::from_str(xml.as_str());

        let expected = ESearchResult {
            count: 2,
            ret_start: 0,
            ret_max: 2,
            id_list: vec![11294, 1387]
        };

        let parsed = ESearchResult::from_reader(&mut reader).unwrap();
        assert_eq!(parsed, expected)
    }
}