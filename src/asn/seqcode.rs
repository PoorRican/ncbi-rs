//! Code and conversion tables for NCBI sequence codes
//!
//! Adapted from ["seqcode.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqcode/seqcode.asn)
//!
//! [`SeqMapTable`] and [`SeqCodeTable`] REQUIRE that codes start with 0 and
//! increase continuously. So IUPAC codes, which are upper case letters will
//! always have 65 0 cells before the code begins. This allows all codes to do
//! indexed lookups.
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Sequence representations
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum SeqCodeType {
    /// IUPAC 1 letter nuc acid code
    IUPACNa = 1,
    /// IUPAC 1 letter amino acid code
    IUPACAa,
    /// 2 bit nucleic acid code
    NCBI2Na,
    /// 4 bit nucleic acid code
    NCBI8Na,
    /// 8 bit extended nucleic acid codes
    NCBIPna,
    /// Nucleic acid probabilities
    NCBI8Aa,
    /// 8 bit extended amino acid codes
    NCBIEaa,
    /// amino acid probabilities
    NCBIPaa,
    /// 3 letter amino acid codes.
    ///
    /// For display only. Parallels [`SeqCodeType::NCBIEaa`]
    IUPACAa3,
    /// consecutive codes for std aa's, 0-25
    NCBIStdAa,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// for tables of sequence mappings
pub struct SeqMapTable {
    /// code to map from
    pub from: SeqCodeType,
    /// code to map to
    pub to: SeqCodeType,
    /// number of rows in table
    pub num: u64,
    /// index offset of first element
    pub start_at: u64,
    /// table of values, in from-to order
    pub table: Vec<u64>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// internal representation of map index
pub struct SeqCodeTableCell {
    /// the printed symbol or letter
    pub symbol: String,
    /// an explanatory name or string
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// for names of coded values
pub struct SeqCodeTable {
    /// name of code
    pub code: SeqCodeType,
    /// number of rows in table
    pub num: u64,
    /// symbol is ALWAYS 1 letter ?
    pub one_letter: bool,
    /// index offset of first element
    pub start_at: u64,
    pub table: Vec<Vec<SeqCodeTableCell>>,
    /// pointers to complement nuc acid
    pub comps: Option<Vec<u64>>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// for distribution
pub struct SeqCodeSet {
    pub codes: Option<Vec<SeqCodeTable>>,
    pub maps: Option<Vec<SeqMapTable>>,
}
