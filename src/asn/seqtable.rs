//! Interface to table readers
//!
//! Adapted from ["seqtable.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/C_DOC/lxr/source/asn/seqtable.asn)

use crate::seqloc::{SeqId, SeqLoc, SeqInterval};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// known column data types
pub enum ColumnInfoFieldId {
    // position types
    Location,
    LocationId,
    LocationGi,
    LocationFrom,
    LocationTo,
    LocationStrand,
    LocationFuzzFromLim,
    LocationFuzzToLim,

    Product,
    ProductId,
    ProductGi,
    ProductFrom,
    ProductTo,
    ProductStrand,
    ProductFuzzFromLim,
    ProductFuzzToLim,

    // main feature fields
    IdLocal,
    XrefIdLocal,
    Partial,
    Comment,
    Title,
    Ext,
    Qual,
    DbXref,

    // various data fields
    DataImpKey,
    DataRegion,
    DataCdregionFrame,

    // extra fields, see also special values for str below
    ExtType,
    QualQual,
    QualVal,
    DbxrefDb,
    DbxrefTag,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
/// Unsure on how this object is used
///
/// Nor do I know of any examples of the use of `field_name`. It *could*
/// however be implemented by parsing str, then matching possible str values.
pub struct SeqTableColumnInfo {
    pub title: Option<String>,
    /// identification of column data in the objects described by the table
    pub field_id: Option<ColumnInfoFieldId>,
    /// # Original Comment:
    ///     any column can be identified by ASN.1 text locator string
    ///        -- with omitted object type.
    ///        -- examples:
    ///        --   "data.gene.locus" for SeqFeat.data.gene.locus
    ///        --   "data.imp.key" for SeqFeat.data.imp.key
    ///        --   "qual.qual"
    ///        --    - SeqFeat.qual is SEQUENCE so several columns are allowed
    ///        --      see also "Q.xxx" special value for shorter qual representation
    ///        --   "ext.type.str"
    ///        --   "ext.data.label.str"
    ///        --   "ext.data.data.int"
    ///        --      see also "E.xxx" special value for shorter ext representation
    ///        -- special values start with capital letter:
    ///        --   "E.xxx" - ext.data.label.str = xxx, ext.data.data = data
    ///        --    - SeqFeat.ext.data is SEQUENCE so several columns are allowed
    ///        --   "Q.xxx" - qual.qual = xxx, qual.val = data
    ///        --    - SeqFeat.qual is SEQUENCE so several columns are allowed
    ///        --   "D.xxx" - dbxref.id = xxx, dbxref.tag = data
    ///        --    - SeqFeat.dbxref is SET so several columns are allowed
    pub field_name: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct CommonStringTable {
    /// set of possible values
    pub strings: Vec<String>,

    /// indexes of values in array 'strings' for each data row
    pub indexes: Vec<usize>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct CommonBytesTable {
    /// set of possible values
    pub bytes: Vec<Vec<u8>>,

    /// indexes of values in array 'bytes' for each data row
    pub indexes: Vec<usize>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Not to sure what the purpose of this class is.
///
/// Also unsure about original comments on scaling.
pub struct ScaledIntMultiData {
    /// output data[i] = data[i] * mul + add
    pub mul: i32,
    pub add: i32,
    pub data: SeqTableMultiData,

    /// Unsure on functionality.
    ///
    /// # Original comment:
    ///     min/max scaled value
    ///     should be set if scaled values may not fit in 32-bit signed integer
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Pretty sure that this class is meant for using double's
pub struct ScaledRealMultiData {
    // output data[i] = data[i] * mul + add
    pub mul: f64,
    pub add: f64,
    pub data: Box<SeqTableMultiData>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Seems to be an artifact from C++ implementation in ASN.1 spec
///
/// Unsure on utility.
///
/// # Original comment:
///     Class for serializing bm::bvector<>     //[ likely a c++ construct ]
///     see include/util/bitset/bm.h
///     Since bvector<> serialization doesn't keep size we have to add it explicitly
pub struct BVectorData {
    pub size: u64,
    pub data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
pub enum SeqTableMultiData {
    /// a set of 4-byte integers, one per row
    Int(Vec<u32>),

    /// as set of reals, one per row
    Real(Vec<f64>),

    /// a set of byte arrays, one per row
    String(Vec<String>),

    /// a set of string with small set of possible values
    Bytes(Vec<Vec<u8>>),

    /// a set of string with small set of possible values
    CommonString(CommonStringTable),

    /// a set of byte arrays with small set of possible values
    CommonBytes(CommonBytesTable),

    /// a set of bits, one per row
    /// most-significant bit in each octet comes first
    Bit(Vec<u8>),

    /// a set of locations, one per row
    Loc(Vec<SeqLoc>),
    Id(Vec<SeqId>),
    Interval(Vec<SeqInterval>),

    /// delta encoded data (int/bit -> int)
    IntDelta(Box<SeqTableMultiData>),

    /// scaled data (int/bit -> int)
    IntScaled(Box<ScaledIntMultiData>),

    /// scaled data (int/bit -> real)
    RealScaled(ScaledRealMultiData),

    /// # Original comment:
    ///     a set of bit, represented as a serialized bvector
    ///     see include/util/bitset/bm.h
    BitVector(BVectorData),

    #[serde(rename="int1")]
    /// a set of signed 1-byte integers encoded as sequential octets
    Int1(Vec<u8>),

    #[serde(rename="int2")]
    /// a set of signed 2-byte integers
    Int2(Vec<u16>),

    #[serde(rename="int3")]
    /// a set of signed 8-byte integers
    Int8(Vec<u64>),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum SeqTableSingleData {
    Int(u64),
    Real(f64),
    String(String),
    Bytes(Vec<u8>),
    Bit(bool),
    Loc(SeqLoc),
    Id(SeqId),
    Interval(SeqInterval),
    Int8(u8),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
pub enum SeqTableSparseIndex {
    /// Indexes of rows with values
    Indexes(Vec<u64>),

    /// Bitset of rows with values, set bit means the row has value.
    /// Most-significant bit in each octet comes first
    BitSet(Vec<u8>),

    /// indexes of rows with values, delta-encoded
    IndexesDelta(Vec<u64>),

    /// # Original comment:
    ///     Bitset of rows with values, as serialized bvector<>,
    ///     see include/util/bitset/bm.h
    BitSetBvector(BVectorData),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
pub struct SeqTableColumn {
    /// column description or reference to previously defined info
    pub header: SeqTableColumnInfo,

    /// row data
    pub data: Option<SeqTableMultiData>,

    /// in case not all rows contain data this field will contain sparse info
    pub sparse: Option<SeqTableSparseIndex>,

    /// default value for sparse table, or if row data is too short
    pub default: Option<SeqTableSingleData>,

    /// single value for indexes not listed in sparse table
    pub sparse_other: Option<SeqTableSingleData>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
pub struct SeqTable {
    /// type of features in this table
    ///
    /// Original comment:
    ///
    ///     ... equal to Seq-feat.data variant index
    pub feat_type: usize,

    /// subtype of features in this table, ...
    ///
    /// # Original comment:
    ///
    ///     ... defined in header SeqFeatData.hpp
    pub feat_subtype: Option<usize>,

    /// number of rows
    pub num_rows: u64,

    /// data in columns
    pub columns: Vec<SeqTableColumn>,
}
