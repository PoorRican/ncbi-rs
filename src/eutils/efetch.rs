use reqwest::Url;
use crate::{EntrezDb, EUtil};
use crate::eutils::{BASE, SEQUENCE_DB};
use crate::seqset::BioSeqSet;

type RetType = &'static str;
type RetMode = &'static str;


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

/// # See Also
///
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
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

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

    pub fn start(mut self, ret_start: usize) -> Self {
        self.ret_start = Some(ret_start);
        self
    }

    pub fn max(mut self, ret_max: usize) -> Self {
        self.ret_max = Some(ret_max);
        self
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