//! EMBL specific data
//!
//! Adapted from ["seqblock.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqblock/seqblock.asn)
//! from the NCBI C++ Toolkit.

use crate::general::{Date, DbTag, ObjectId};
use crate::seqloc::SeqId;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Internal representation of DB code for [`EMBLDbNameCode`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer
pub enum EMBLDbNameCode {
    EMBL,
    GenBank,
    DDBJ,
    GenInfo,
    MedLine,
    SWISSPROT,
    PIR,
    PDB,
    EPD,
    ECD,
    TFD,
    FlyBase,
    ProSite,
    Enzyme,
    MIM,
    EcoSeq,
    HIV,
    Other = 255,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EMBLDbName {
    Code(EMBLDbNameCode),
    Name(String),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct EMBLXref {
    pub dbname: EMBLDbName,
    pub id: Vec<ObjectId>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
/// Internal representation of block class for [`EMBLBlockClass`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum EMBLBlockClass {
    NotSet,
    #[default]
    Standard,
    Unannotated,
    Other = 255,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Internal representation of block division for [`EMBLBlockClass`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum EMBLBlockDiv {
    Fun,
    Inv,
    Mam,
    Org,
    Pln,
    Pri,
    Pro,
    Rod,
    Syn,
    Una,
    Vrl,
    Vrt,
    Pat,
    Est,
    STS,
    Other = 255,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct EMBLBlock {
    pub class: EMBLBlockClass,
    pub div: EMBLBlockDiv,
    pub creation_date: Date,
    pub update_date: Date,
    pub extra_acc: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub xref: Option<Vec<EMBLXref>>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// internal representation of `class` for [`SPBlock`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum SPBlockClass {
    NotSet,
    /// conforms to all SWISSPROT checks
    Standard,
    /// only seq and biblio checked
    Prelim,
    Other = 255,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// SWISSPROT specific descriptions
pub struct SPBlock {
    pub class: SPBlockClass,

    /// old SWISSPROT id's
    pub extra_acc: Option<Vec<String>>,

    /// seq known to start with Met
    /// Should default to false
    pub imeth: bool,

    /// plasmid names carrying gene
    pub plasnm: Option<Vec<String>>,

    /// xref to other sequences
    pub seqref: Option<Vec<SeqId>>,

    /// xref to non-sequence db's
    pub dbref: Option<Vec<DbTag>>,

    /// keywords
    pub keywords: Option<Vec<String>>,

    /// creation date
    pub created: Option<Date>,

    /// sequence update
    pub sequpd: Option<Date>,

    /// annotation update
    pub annotupd: Option<Date>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// PIR specific descriptions
pub struct PIRBlock {
    /// had punctuation in sequence?
    pub had_punct: Option<bool>,

    pub host: Option<String>,

    /// source line
    pub source: Option<String>,

    pub summary: Option<String>,
    pub genetic: Option<String>,
    pub includes: Option<String>,
    pub placement: Option<String>,
    pub superfamily: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub cross_reference: Option<String>,
    pub date: Option<String>,

    /// seq with punctuation
    pub seq_raw: Option<String>,

    /// xref to other sequences
    pub seqref: Option<Vec<SeqId>>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GBBlock {
    pub extra_accessions: Option<Vec<String>>,
    /// source line
    pub source: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub origin: Option<String>,

    /// *OBSOLETE* old form entry date
    pub date: Option<String>,

    /// replaces date
    pub entry_date: Option<Date>,

    /// GenBank division
    pub div: Option<String>,

    /// continuation line of organism
    pub taxonomy: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// Protein Research Foundation specific definition
pub struct PRFBlock {
    pub extra_src: Option<PRFExtraSrc>,
    pub keywords: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PRFExtraSrc {
    pub host: Option<String>,
    pub part: Option<String>,
    pub state: Option<String>,
    pub strain: Option<String>,
    pub taxon: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// PDB specific descriptions
pub struct PDBBlock {
    /// deposition date: month,year
    pub deposition: Date,

    pub class: String,
    pub compound: Vec<String>,
    pub source: Vec<String>,

    /// present if NOT X-ray diffraction
    pub exp_method: Option<String>,

    /// replacement history
    pub replace: Option<PDBReplace>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PDBReplace {
    pub date: Date,

    /// entry ids replace by this one
    pub ids: Vec<String>,
}
