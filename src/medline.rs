//! NCBI MEDLINE data definitions
//!
//! Adapted from ["medline.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/C_DOC/lxr/source/asn/medline.asn)

use std::collections::HashSet;
use crate::biblio::{CitArt, PubMedId};
use crate::general::Date;

#[derive(Default)]
pub enum MedlineEntryStatus {
    /// record as supplied by publisher
    Publisher = 1,

    /// pre-medline record
    PreMedline,

    #[default]
    /// regular medline record
    Medline
}

/// a MEDLINE or PubMed entry
pub struct MedlineEntry {
    /// MEDLINE UID, sometimes not yet available from PubMed
    pub uid: Option<u64>,

    /// entry month
    pub em: Date,

    /// article citation
    pub cit: CitArt,

    pub r#abstract: Option<String>,
    pub mesh: Option<HashSet<MedlineMesh>>,
    pub substance: Option<HashSet<MedlineRn>>,
    pub xref: Option<HashSet<MedlineSi>>,

    /// ID Number (grants, contracts)
    pub idnum: Option<HashSet<String>>,

    pub gene: Option<HashSet<String>>,

    /// MEDLINE records may include the PubMedId
    pub pmid: Option<PubMedId>,

    /// may show publication types (review, etc)
    pub pub_type: Option<HashSet<String>>,

    /// additional Medline field types
    pub mlfield: Option<HashSet<MedlineField>>,

    pub status: MedlineEntryStatus
}

pub struct MedlineMesh {
    // TODO: default false
    /// true if main point (*)
    pub mp: bool,

    ///the MeSH term
    pub term: String,

    /// qualifiers
    pub qual: Option<HashSet<MedlineQual>>
}

pub struct MedlineQual {
    // TODO: default false
    /// true if main point
    pub mp: bool,

    /// the subheading
    pub subh: String,
}

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
    GDB
}

pub enum MedlineRnType {
    NameOnly,

    /// CAS number
    CAS,

    /// EC number
    EC
}

/// Medline substance records
pub struct MedlineRn {
    /// type of record
    pub r#type: MedlineRnType,

    /// CAS or EC if present
    pub cit: Option<String>,

    /// name (always present)
    pub name: String
}

/// medline cross reference records
pub struct MedlineSi {
    /// type of xref
    pub r#type: MedlineSiType,
    pub cit: Option<String>
}

pub enum MedlineFieldType {
    /// look in line code
    Other,

    /// comment line
    Comment,

    /// retracted, corrected, etc
    Erratum
}

pub struct MedlineField {
    /// keyed type
    pub r#type: MedlineFieldType,

    ///the text
    pub cit: Option<String>,

    /// pointers relevant to this text
    pub ids: Option<Vec<DocRef>>
}

pub enum DocRefType {
    Medline = 1,
    PubMed,
    NCBIGi
}

/// reference to a document
pub struct DocRef {
    pub r#type: DocRefType,
    pub uid: u64,
}






















