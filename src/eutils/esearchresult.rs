use std::slice::Iter;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::parsing::{read_int, read_vec_str_unchecked, XmlNode};

#[derive(Default, Debug, Clone, PartialEq)]
/// Encapsulates data returned from [`ESearch`]
///
/// This is an intermediate data format that contains ID's to be retrieved
/// via [`EFetch`] or any of the other Entrez tools.
///
/// The following elements have not been implemented because they deal with
/// how the users query is translated:
/// - `TranslationSet`
/// - `TranslationStack`
/// - `QueryTranslation`
///
/// These might be useful to parse in the future because it might nail down
/// query anomalies for those who aren't previously familiar with NCBI's query
/// syntax.
pub struct ESearchResult {
    /// total number of results from this query
    count: u64,


    /// maximum number of values to return
    ret_max: u64,

    /// index of result in respect to `Self::count`
    ret_start: u64,

    /// id's should be stored as a String to account for accession.version strings
    id_list: Vec<String>,
}

impl ESearchResult {
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn ret_max(&self) -> u64 {
        self.ret_max
    }

    pub fn ret_start(&self) -> u64 {
        self.ret_start
    }

    /// Returns an iter of `id_list`
    pub fn id_list_iter(&self) -> Iter<String> {
        self.id_list.iter()
    }
}

impl XmlNode for ESearchResult {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("eSearchResult")
    }

    /// Unknown tags are explicitly not caught by [`UnexpectedTags`] as mentioned
    /// above. The priority is to parse the id list.
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
                        result.id_list = read_vec_str_unchecked(reader, &id_list_tag.to_end())
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

    const DATA1: &str = "tests/data/deaminase_genome_search.xml";

    #[test]
    fn read_from_xml() {
        let xml = get_local_xml(DATA1).unwrap();
        let mut reader = Reader::from_str(xml.as_str());

        let expected = ESearchResult {
            count: 2,
            ret_start: 0,
            ret_max: 2,
            id_list: vec!["11294".to_string(), "1387".to_string()]
        };

        let parsed = ESearchResult::from_reader(&mut reader).unwrap();
        assert_eq!(parsed, expected)
    }
}