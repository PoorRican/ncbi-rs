

use crate::seqset::BioSeqSet;
use crate::entrezgene::EntrezgeneSet;
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

//FIXME: Please a comment what this is about
#[derive(Debug)]
pub enum DataType {
    BioSeqSet(BioSeqSet),
    EntrezgeneSet(EntrezgeneSet),
    /// placeholder for other types
    EtAl,
}

pub fn parse_xml(response: &str) -> Result<DataType, String> {
    let mut reader = Reader::from_str(response);
    reader.trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag_name = e.name().into_inner(); // Extract the inner byte slice
                if let Ok(tag_str) = std::str::from_utf8(tag_name) {
                    println!("Found XML tag: {}", tag_str); // Debugging output
                } else {
                    println!("Found XML tag (invalid UTF-8): {:?}", tag_name);
                }

                if tag_name == b"Bioseq-set" {
                    println!("Matched Bioseq-Set, attempting to parse...");
                    return BioSeqSet::from_reader(&mut reader)
                        .map(|set| DataType::BioSeqSet(set))
                        .ok_or("Failed to parse BioSeqSet.".to_string());
                }
                if tag_name == b"Entrezgene-Set" {
                    println!("Matched Entrezgene-Set, attempting to parse...");
                    return EntrezgeneSet::from_reader(&mut reader)
                        .map(|set| DataType::EntrezgeneSet(set))
                        .ok_or("Failed to parse EntrezgeneSet.".to_string());
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(format!("XML parsing error: {:?}", e));
            }
            _ => (),
        }
        buf.clear();
    }

    Err("No recognizable XML root tag found.".to_string())
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

    #[test]
    fn search_url() {
        let _url = build_search_url(EntrezDb::Protein, "deaminase");
    }

    #[test]
    fn test_protein() {
        let id = "2520667272";
        let _url = build_fetch_url(EntrezDb::Protein, id, "native", "xml");
    }

    #[test]
    fn test_parse_xml() {
        let data = get_local_xml("tests/data/2519734237.xml");
        match parse_xml(data.as_str()).unwrap() {
            DataType::BioSeqSet(_) => (),
            _ => assert!(false),
        }
        let data = get_local_xml("tests/data/tp73.genbank.xml");
        let result = parse_xml(data.as_str());
        println!("Parse result: {:?}", result);
        match result {
            Ok(DataType::EntrezgeneSet(_)) => (),
            Ok(_) => assert!(false, "Parsed unexpected data type."),
            Err(e) => panic!("Error while parsing XML: {:?}", e),
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
