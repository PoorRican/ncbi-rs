//! Helper functions that deal with Entrez eUtils

mod esearchresult;
mod esearch;
mod eutil;

/// reexport modules
pub use eutil::EUtil;
pub use esearch::ESearch;
pub use esearchresult::ESearchResult;

use crate::seqset::BioSeqSet;
use crate::parsing::XmlNode;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{fs, io};

const BASE: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";

#[derive(Copy, Clone, Debug, PartialEq)]
/// # See Also
///
/// [Entrez Unique Identifiers table](https://www.ncbi.nlm.nih.gov/books/NBK25497/table/chapter2.T._entrez_unique_identifiers_ui/)
pub enum EntrezDb {
    BioProject,
    BioSample,
    Books,
    ConservedDomains,
    DbGaP,
    DbVar,
    Gene,
    Genome,
    GeoDatasets,
    GeoProfiles,
    HomoloGene,
    MeSH,
    NlmCatalog,
    Nucleotide,
    PopSet,
    Probe,
    Protein,
    ProteinClusters,
    PubChemBioAssay,
    PubChemCompound,
    PubChemSubstance,
    PubMed,
    PubMedCentral,
    Snp,
    Sra,
    Structure,
    Taxonomy,
}
impl EntrezDb {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BioProject => "bioproject",
            Self::BioSample => "biosample",
            Self::Books => "books",
            Self::ConservedDomains => "cdd",
            Self::DbGaP => "gap",
            Self::DbVar => "dbvar",
            Self::Gene => "gene",
            Self::Genome => "genome",
            Self::GeoDatasets => "gds",
            Self::GeoProfiles => "geoprofiles",
            Self::HomoloGene => "homologene",
            Self::MeSH => "mesh",
            Self::NlmCatalog => "nlmcatalog",
            Self::Nucleotide => "nuccore",
            Self::PopSet => "popset",
            Self::Probe => "probe",
            Self::Protein => "protein",
            Self::ProteinClusters => "proteinclusters",
            Self::PubChemBioAssay => "pcassay",
            Self::PubChemCompound => "pccompound",
            Self::PubChemSubstance => "pcsubstance",
            Self::PubMed => "pubmed",
            Self::PubMedCentral => "pmc",
            Self::Snp => "snp",
            Self::Sra => "sra",
            Self::Structure => "structure",
            Self::Taxonomy => "taxonomy",
        }
    }
}

/// View [EFetch documentation](https://www.ncbi.nlm.nih.gov/books/NBK25499/table/chapter4.T._valid_values_of__retmode_and/?report=objectonly)
/// for a valid list of `retmode` and `rettype` values
pub fn build_fetch_url(db: EntrezDb, id: &str, r#type: &str, mode: &str) -> String {
    let mut url_str = format!("{}efetch.fcgi?", BASE);
    url_str.push_str(&(format!("db={}", db.as_str())));
    url_str.push_str(&(format!("&id={}", id)));

    url_str.push_str(&(format!("&rettype={}", r#type)));
    url_str.push_str(&(format!("&retmode={}", mode)));

    url_str
}

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

/// Parse an XML string
///
/// Since XML strings will only be found with high-level objects,
/// these are the only values that will be returned (ie: Entrez will never return a plain
/// `ObjectId` or single `SeqFeat`).
///
/// # Parameters
///
/// - `xml`: XML data as str
///
/// # Returns
///
/// `Ok` if [`DataType`] can be returned. Otherwise an `Err(())` is returned
pub fn parse_xml(xml: &str) -> Result<DataType, ()> {
    let mut reader = Reader::from_str(xml);
    loop {
        match reader.read_event().unwrap() {
            Event::Start(e) => {
                if e.name() == BioSeqSet::start_bytes().name() {
                    let set = BioSeqSet::from_reader(&mut reader).unwrap();
                    return Ok(DataType::BioSeqSet(set));
                }
            }
            Event::Eof => break,
            _ => (),
        }
    }
    return Err(());
}

/// Fetch local XML data from filesystem
///
/// # Parameters
///
/// - `path`: Path to local XML file
///
/// # Returns
///
/// `Ok` with XML data as `String` if read successfully; otherwise, returns the
/// problematic io::Error
pub fn get_local_xml(path: &str) -> Result<String, io::Error> {
    let file = fs::read(path);
    Ok(file?
        .escape_ascii()
        .to_string())
}

pub fn fetch(db: EntrezDb, id: &str, r#type: &str, mode: &str) -> DataType {
    let url = build_fetch_url(db, id, r#type, mode);
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    parse_xml(response.as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use std::ops::Not;
    use crate::{build_fetch_url, get_local_xml, parse_xml, DataType, EntrezDb};

    #[test]
    fn test_protein() {
        let id = "2520667272";
        let _url = build_fetch_url(EntrezDb::Protein, id, "native", "xml");
    }

    #[test]
    fn test_parse_xml() {
        let data = get_local_xml("tests/data/2519734237.xml").unwrap();
        match parse_xml(data.as_str()).unwrap() {
            DataType::BioSeqSet(_) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_local_xml() {
        // Valid
        let data = get_local_xml("tests/data/2519734237.xml");
        assert!(data.is_ok());
        assert!(data.unwrap().is_empty().not());

        // Invalid
        let data = get_local_xml("tests/data/2.xml");
        assert!(data.is_err());

    }


    #[test]
    fn test_article_set() {
        let id = "37332098";
        let db = EntrezDb::PubMed;

        let url = build_fetch_url(db, id, "xml", "xml");
        let _ = reqwest::blocking::get(url).unwrap().text().unwrap();
        //let expected = from_str(text.as_str()).unwrap();
        //assert!(expected.is_empty().not())
    }
}
