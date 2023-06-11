//! Sequence Analysis Results (other than alignments)
//!
//! Adapted from ["seqres.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqres/seqres.asn)

use crate::seqloc::SeqLoc;

pub enum SeqGraphChoice {
    Real(RealGraph),
    Int(IntGraph),
    Byte(ByteGraph),
}

/// For values mapped by residue or range to sequence
pub struct SeqGraph {
    pub title: Option<String>,
    pub comment: Option<String>,
    /// region this applies to
    pub loc: SeqLoc,
    /// title for x-axis
    pub title_x: Option<String>,
    /// title for y-axis
    pub title_y: Option<String>,
    pub comp: Option<i64>,
    /// for scaling values
    ///
    /// `display = (a * value ) + b`
    pub a: Option<f64>,
    pub b: Option<f64>,

    /// number of values in graph
    pub numval: u64,

    pub graph: SeqGraphChoice,
}

pub struct Graph<T> {
    /// top of graph
    pub max: T,

    /// bottom of graph
    pub min: T,

    /// value to draw axis on
    pub axis: T,

    pub values: Vec<T>
}

pub type RealGraph = Graph<f64>;

pub type IntGraph = Graph<u64>;

/// integer from 0-255
pub type ByteGraph = Graph<u8>;
