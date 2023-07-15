use crate::seqset::BioSeqSet;
use crate::parsing::XmlNode;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;

const BASE: &str = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/";

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

pub fn build_search_url(db: EntrezDb, term: &str) -> String {
    let mut url_str = format!("{}esearch.fcgi?", BASE);
    url_str.push_str(&(format!("db={}", db.as_str())));
    url_str.push_str(&(format!("&term={}", term)));

    let ret = "xml";
    url_str.push_str(&(format!("&rettype={}", ret)));
    url_str.push_str(&(format!("&retmode={}", ret)));

    url_str
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

pub enum DataType {
    BioSeqSet(BioSeqSet),
    /// placeholder for other types
    EtAl,
}

pub fn parse_xml(response: &str) -> Result<DataType, ()> {
    let mut reader = Reader::from_str(response);
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

pub fn get_local_xml(path: &str) -> String {
    let file = fs::read(path);
    return file.unwrap().escape_ascii().to_string();
}

pub fn fetch_data(db: EntrezDb, id: &str, r#type: &str, mode: &str) -> DataType {
    let url = build_fetch_url(db, id, r#type, mode);
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    parse_xml(response.as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{build_fetch_url, build_search_url, get_local_xml, parse_xml, DataType, EntrezDb};
    use std::fs;

    #[test]
    fn search_url() {
        let url = build_search_url(EntrezDb::Protein, "deaminase");
    }

    #[test]
    fn test_protein() {
        let id = "2520667272";
        let url = build_fetch_url(EntrezDb::Protein, id, "native", "xml");
    }

    #[test]
    fn test_parse_xml() {
        let data = get_local_xml("tests/data/2519734237.xml");
        match parse_xml(data.as_str()).unwrap() {
            DataType::BioSeqSet(_) => (),
            _ => assert!(false),
        }
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
