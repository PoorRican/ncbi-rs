//! NCBI MEDLINE data definitions
//!
//! Adapted from ["medline.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/C_DOC/lxr/source/asn/medline.asn)

use crate::biblio::{CitArt, PubMedId};
use crate::general::Date;

#[derive(PartialEq, Debug, Default)]
pub enum MedlineEntryStatus {
    /// record as supplied by publisher
    Publisher = 1,

    /// pre-medline record
    PreMedline,

    #[default]
    /// regular medline record
    Medline,
}

#[derive(PartialEq, Debug)]
/// a MEDLINE or PubMed entry
pub struct MedlineEntry {
    /// MEDLINE UID, sometimes not yet available from PubMed
    pub uid: Option<u64>,

    /// entry month
    pub em: Date,

    /// article citation
    pub cit: CitArt,

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

#[derive(PartialEq, Debug)]
pub struct MedlineMesh {
    // TODO: default false
    /// true if main point (*)
    pub mp: bool,

    ///the MeSH term
    pub term: String,

    /// qualifiers
    pub qual: Option<Vec<MedlineQual>>,
}

#[derive(PartialEq, Debug)]
pub struct MedlineQual {
    // TODO: default false
    /// true if main point
    pub mp: bool,

    /// the subheading
    pub subh: String,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum MedlineRnType {
    NameOnly,

    /// CAS number
    CAS,

    /// EC number
    EC,
}

#[derive(PartialEq, Debug)]
/// Medline substance records
pub struct MedlineRn {
    /// type of record
    pub r#type: MedlineRnType,

    /// CAS or EC if present
    pub cit: Option<String>,

    /// name (always present)
    pub name: String,
}

#[derive(PartialEq, Debug)]
/// medline cross reference records
pub struct MedlineSi {
    /// type of xref
    pub r#type: MedlineSiType,
    pub cit: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum MedlineFieldType {
    /// look in line code
    Other,

    /// comment line
    Comment,

    /// retracted, corrected, etc
    Erratum,
}

#[derive(PartialEq, Debug)]
pub struct MedlineField {
    /// keyed type
    pub r#type: MedlineFieldType,

    ///the text
    pub cit: Option<String>,

    /// pointers relevant to this text
    pub ids: Option<Vec<DocRef>>,
}

#[derive(PartialEq, Debug)]
pub enum DocRefType {
    Medline = 1,
    PubMed,
    NCBIGi,
}

#[derive(PartialEq, Debug)]
/// reference to a document
pub struct DocRef {
    pub r#type: DocRefType,
    pub uid: u64,
}
