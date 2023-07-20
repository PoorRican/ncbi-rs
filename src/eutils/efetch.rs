//! Retrieve and handle data from NCBI
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Url;
use crate::{EntrezDb, ESearchResult, EUtil};
use crate::eutils::{BASE, SEQUENCE_DB};
use crate::parsing::XmlNode;
use crate::seqset::BioSeqSet;

type RetType = &'static str;
type RetMode = &'static str;

#[non_exhaustive]
/// Encapsulate high-level types returned by EUtils
///
/// This captures the possible return values from parsed XML. While each NCBI
/// database returns it's own type, knowledge of query is not necessary for
/// parsing.
pub enum DataType {
    BioSeqSet(BioSeqSet),
    /// placeholder for other types
    EtAl,
}

/// [Valid values of &retmode and &rettype for EFetch](https://www.ncbi.nlm.nih.gov/books/NBK25499/table/chapter4.T._valid_values_of__retmode_and/?report=objectonly)
fn check_return_type_mode(db: EntrezDb) -> (RetType, RetMode) {
    match db {
        EntrezDb::Nucleotide | EntrezDb::Protein | EntrezDb::PopSet => ("native", "xml"),
        EntrezDb::Genome => ("native", "xml"),
        EntrezDb::Gene => ("", "xml"),
        EntrezDb::HomoloGene => ("", "xml"),
        // only db's that store have parsing implementations
        _ => unimplemented!("Only fetching from sequence databases has been implemented")
    }
}

/// Retrieves biological sequence data and it's metadata from NCBI
///
/// # Example
///
/// ```
/// // accession version numbers for E. coli RL465 plasmid's I, II, III
/// use std::ptr::replace;
/// use ncbi::{DataType, EFetch, EntrezDb, EUtil};
/// let acc = [
///     "NZ_LT906558.1",
///     "NZ_LT906557.1",
///     "NZ_LT906556.1",
/// ];
/// let response = EFetch::new(EntrezDb::Nucleotide)
///                     .ids(&acc)
///                     .fetch();
///
/// if let DataType::BioSeqSet(set) = response.unwrap() {
///     println!("{:?}", set)
/// } else {
///     assert!(false)
/// }
/// ```
///
/// # See Also
///
/// [Chapter on EFetch syntax](https://www.ncbi.nlm.nih.gov/books/NBK25499/#chapter4.EFetch)
/// from the NCBI EUtils book.
pub struct EFetch {
    db: EntrezDb,
    id: Option<String>,
    ret_start: Option<usize>,
    ret_max: Option<usize>,

    /// automatically calculated by [`check_return_type_mode()`]
    ret_type: RetType,

    /// automatically calculated by [`check_return_type_mode()`]
    ret_mode: RetMode,
}

impl EFetch {
    const ENDPOINT: &'static str = "efetch.fcgi?";

    #[must_use]
    /// Builder method for `id` field
    ///
    /// # Example
    ///
    /// ```
    /// use ncbi::{EFetch, EntrezDb, EUtil};
    ///
    /// let builder = EFetch::new(EntrezDb::Genome)
    ///                 .id("1".to_string());
    /// let url = builder.build_url();
    /// assert!(url.as_str()
    ///            .contains("&id=1"))
    /// ```
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Builder method for passing a slice of &str values
    ///
    /// # Example
    ///
    /// ```
    /// use ncbi::{EFetch, EntrezDb, EUtil};
    ///
    /// let builder = EFetch::new(EntrezDb::Genome)
    ///                 .ids(&["1", "2"]);
    /// let url = builder.build_url();
    /// // url contains url-encoded, but comma-separated string
    /// assert!(url.as_str()
    ///            .contains("&id=1%2C2"))
    /// ```
    pub fn ids(self, ids: &[& str]) -> Self {
        let mut concatenated = String::new();

        let mut iter = ids.iter();
        if let Some(&first) = iter.next() {
            concatenated.push_str(first);
        }
        iter.for_each(|&i| {
            concatenated.push_str(",");
            concatenated.push_str(i);
        });

        self.id(concatenated)
    }

    /// Builder method for setting id term from an [`ESearchResult`]
    ///
    /// # Example
    ///
    /// ```
    /// use ncbi::{EFetch, EntrezDb, ESearchResult, EUtil};
    ///
    /// let db = EntrezDb::Genome;
    /// let result = ESearchResult::new( 2, 2, 0, vec![
    ///     "1".to_string(),
    ///     "2".to_string() ]);
    ///
    /// let builder = EFetch::new(db)
    ///                 .ids_from_result(result);
    /// let url = builder.build_url();
    /// // url contains url encoded, but comma-separated string
    /// assert!(url.as_str()
    ///            .contains("&id=1%2C2"))
    /// ```
    pub fn ids_from_result(self, result: ESearchResult) -> Self {
        let mut concatenated = String::new();
        let mut iter = result.iter_id_list();

        if let Some(first) = iter.next() {
            concatenated.push_str(first.as_str());
        }
        iter.for_each(|i| {
            concatenated.push_str(",");
            concatenated.push_str(i.as_str());
        });

        self.id(concatenated)
    }

    /// Builder method for "retstart"
    ///
    /// # Example
    /// ```
    /// use ncbi::{EFetch, EntrezDb, EUtil};
    ///
    /// let builder = EFetch::new(EntrezDb::Genome)
    ///                 .id("".to_string())
    ///                 .start(777);
    /// let url = builder.build_url();
    /// assert!(url.as_str()
    ///            .contains("&retstart=777"))
    /// ```
    pub fn start(mut self, ret_start: usize) -> Self {
        self.ret_start = Some(ret_start);
        self
    }


    /// Builder method for "retmax"
    ///
    /// # Example
    /// ```
    /// use ncbi::{EFetch, EntrezDb, EUtil};
    ///
    /// let builder = EFetch::new(EntrezDb::Genome)
    ///                 .id("".to_string())
    ///                 .max(777);
    /// let url = builder.build_url();
    /// assert!(url.as_str()
    ///            .contains("&retmax=777"))
    /// ```
    pub fn max(mut self, ret_max: usize) -> Self {
        self.ret_max = Some(ret_max);
        self
    }

    pub fn fetch(&self) -> Option<DataType> {
        let response = self.get();
        let mut reader = Reader::from_str(response.as_str());

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == BioSeqSet::start_bytes().name() {
                        let set = BioSeqSet::from_reader(&mut reader).unwrap();
                        return Some(DataType::BioSeqSet(set));
                    }
                }
                Event::Eof => break,
                _ => (),
            }
        }
        return None;
    }
}

impl EUtil for EFetch {
    fn new(db: EntrezDb) -> Self {
        let (ret_type, ret_mode) = check_return_type_mode(db);
        Self {
            db,
            id: None,
            ret_start: None,
            ret_max: None,
            ret_type,
            ret_mode
        }
    }

    fn build_url(&self) -> Url {
        let mut url = Url::parse(BASE).unwrap();
        url = url.join(Self::ENDPOINT).unwrap();

        let mut queries = url.query_pairs_mut();
        queries.append_pair("db", self.db.as_str())
            .append_pair("id", self.id.as_ref().expect("No id given!").as_str())
            .append_pair("sort", "relevance")
            .append_pair("retmode", self.ret_mode)
            .append_pair("rettype", self.ret_type);

        if SEQUENCE_DB.contains(&self.db) {
            queries.append_pair("idtype", "acc");
        }

        // optional query refinements
        if let Some(ret_start) = self.ret_start {
            queries.append_pair("retstart", format!("{}", ret_start).as_str());
        }
        if let Some(ret_max) = self.ret_max {
            queries.append_pair("retmax", format!("{}", ret_max).as_str());
        }
        drop(queries);

        url
    }
}

#[cfg(test)]
mod tests {
    use crate::{EFetch, EntrezDb, EUtil};
    use crate::eutils::efetch::check_return_type_mode;

    #[test]
    fn test_check_return_type_mode() {
        assert_eq!(check_return_type_mode(EntrezDb::Nucleotide), ("native", "xml"));
        assert_eq!(check_return_type_mode(EntrezDb::Protein), ("native", "xml"));
        assert_eq!(check_return_type_mode(EntrezDb::PopSet), ("native", "xml"));

        assert_eq!(check_return_type_mode(EntrezDb::Genome), ("native", "xml"));

        assert_eq!(check_return_type_mode(EntrezDb::Gene), ("", "xml"));
        assert_eq!(check_return_type_mode(EntrezDb::HomoloGene), ("", "xml"));
    }

    #[test]
    /// Assert that "&idtype=acc" query is added to URL
    fn test_sequence_db_build_url() {
        let builder = EFetch::new(EntrezDb::Nucleotide)
            .id("Test".to_string());
        let url = builder.build_url();
        let url = url.as_str();

        let expected = "&idtype=acc";
        assert!(url.contains(expected));
    }

    #[test]
    #[should_panic]
    fn panics_wo_id() {
        let builder = EFetch::new(EntrezDb::Nucleotide);
        builder.build_url();
    }

    #[test]
    fn test_id_builder() {
        let builder = EFetch::new(EntrezDb::Nucleotide)
            .id("accession.version".to_string());
        let url = builder.build_url();
        let url = url.as_str();

        let expected = "&id=accession.version";
        assert!(url.contains(expected));
    }


    #[test]
    fn test_ids_builder() {
        let builder = EFetch::new(EntrezDb::Nucleotide)
            .ids(&["accession.version", "secondId"] );
        let url = builder.build_url();
        let url = url.as_str();

        let expected = "&id=accession.version%2CsecondId";
        assert!(url.contains(expected));
    }

}