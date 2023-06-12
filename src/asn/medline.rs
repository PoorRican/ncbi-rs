//! NCBI MEDLINE data definitions
//!
//! Adapted from ["medline.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/C_DOC/lxr/source/asn/medline.asn)

use crate::biblio::{CitArt, PubMedId};
use crate::general::Date;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all="lowercase")]
pub enum MedlineEntryStatus {
    /// record as supplied by publisher
    Publisher = 1,

    /// pre-medline record
    PreMedline,

    #[default]
    /// regular medline record
    Medline,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
/// a MEDLINE or PubMed entry
pub struct MedlineEntry {
    /// MEDLINE UID, sometimes not yet available from PubMed
    pub uid: Option<u64>,

    /// entry month
    pub em: Date,

    /// article citation
    pub cit: CitArt,

    #[serde(rename="abstract")]
    pub r#abstract: Option<String>,
    pub mesh: Option<Vec<MedlineMesh>>,
    pub substance: Option<Vec<MedlineRn>>,
    pub xref: Option<Vec<MedlineSi>>,

    /// ID Number (grants, contracts)
    pub idnum: Option<Vec<String>>,

    pub gene: Option<Vec<String>>,

    /// MEDLINE records may include the PubMedId
    pub pmid: Option<PubMedId>,

    /// may show publication types (review, etc)
    pub pub_type: Option<Vec<String>>,

    /// additional Medline field types
    pub mlfield: Option<Vec<MedlineField>>,

    pub status: MedlineEntryStatus,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MedlineMesh {
    // TODO: default false
    /// true if main point (*)
    pub mp: bool,

    ///the MeSH term
    pub term: String,

    /// qualifiers
    pub qual: Option<Vec<MedlineQual>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MedlineQual {

    /// true if main point
    pub mp: bool,           // TODO: default false

    /// the subheading
    pub subh: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum MedlineSiType {
    DDBJ = 1,
    /// Carbohydrate Structure Database
    CarbBank,
    /// EMBL Data Library
    EMBL,
    /// Hybridoma Data Bank
    HDB,
    /// GenBank
    GenBank,
    /// Human Gene Map Library
    HGML,
    /// Mendelian Inheritance in Man
    MIM,
    /// Microbial Strains Database
    MSD,
    /// Protein Data Bank (Brookhaven)
    PDB,
    /// Protein Identification Resource
    PIR,
    /// Protein Research Foundation (Japan)
    PrfSeqDb,
    /// Protein Sequence Database (Japan)
    PSD,
    /// SwissProt
    SwissProt,
    /// genome data base
    GDB,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum MedlineRnType {
    NameOnly,

    /// CAS number
    CAS,

    /// EC number
    EC,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
/// Medline substance records
pub struct MedlineRn {
    #[serde(rename="type")]
    /// type of record
    pub r#type: MedlineRnType,

    /// CAS or EC if present
    pub cit: Option<String>,

    /// name (always present)
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
/// medline cross reference records
pub struct MedlineSi {
    /// type of xref
    #[serde(rename="type")]
    pub r#type: MedlineSiType,
    pub cit: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum MedlineFieldType {
    /// look in line code
    Other,

    /// comment line
    Comment,

    /// retracted, corrected, etc
    Erratum,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MedlineField {
    #[serde(rename="type")]
    /// keyed type
    pub r#type: MedlineFieldType,

    ///the text
    pub cit: Option<String>,

    /// pointers relevant to this text
    pub ids: Option<Vec<DocRef>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum DocRefType {
    Medline = 1,
    PubMed,
    NCBIGi,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
/// reference to a document
pub struct DocRef {
    #[serde(rename="type")]
    pub r#type: DocRefType,
    pub uid: u64,
}
