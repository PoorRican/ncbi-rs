use quick_xml::events::Event;
use quick_xml::Reader;
use crate::seq::BioSeq;
use crate::seqset::BioSeqSet;
use crate::XMLElement;

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
    Taxonomy
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
            Self::Taxonomy => "taxonomy"
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
}

pub fn parse_response(response: &str) -> Result<DataType, ()> {
    let mut reader = Reader::from_str(response);
    loop {
        match reader.read_event().unwrap() {
            Event::Start(e) => {
                if e.name() == BioSeqSet::start_bytes().name() {
                    let set = BioSeqSet::from_reader(&mut reader);
                    return Ok(DataType::BioSeqSet(set));
                }
            }
            Event::End(e) => {
            }
            Event::Empty(e) => {}
            Event::Text(e) => {}
            Event::Eof => break,
            _ => ()
        }
    }
    return Err(())
}