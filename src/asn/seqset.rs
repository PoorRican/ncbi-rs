//! NCBI Sequence Collections
//!
//! Adapted from ["seqset.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqset/seqset.asn)
//! from the NCBI C++ Toolkit

use crate::general::{Date, DbTag, ObjectId};
use crate::seq::{BioSeq, SeqAnnot, SeqDescr};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
/// internal representation of `class` field for [`BioSeqSet`]
pub enum BioSeqSetClass {
    #[default]
    NotSet,

    /// nuc acid and coded proteins
    NucProt,

    /// segmented sequence + parts
    SegSet,

    /// constructed sequence + parts
    ConSet,

    /// parts for [`BioSeqSetClass::SetSet`] or [`BioSeqSetClass::ConSet`]
    Parts,

    /// GenInfo backbone
    Gibb,

    /// GenInfo
    Gi,

    /// converted GenBank
    Genbank,

    /// converted PIR
    Pir,

    /// all the seqs from a single publication
    PubSet,

    /// a set of equivalent maps or seqs
    Equiv,

    /// converted SWISSPROT
    Swissprot,

    /// a complete PDB entry
    PdbEntry,

    /// set of mutations
    MutSet,

    /// population study
    PopSet,

    /// phylogenetic study
    PhySet,

    /// ecological sample study
    EcoSet,

    /// genomic products, chrom+mRNA+protein
    GenProdSet,

    /// whole genome shotgun project
    WgsSet,

    /// named annotation set
    NamedAnnot,

    /// with instantiated mRNA+protein
    NamedAnnotProd,

    /// set from a single read
    ReadSet,

    /// paired sequences within a read-set
    PairedEndReads,

    /// viral segments or mitochondrial mini-circles
    SmallGenomeSet,

    Other = 255,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
/// just a collection
pub struct BioSeqSet {
    pub id: Option<ObjectId>,

    /// to identify a collection
    pub coll: Option<DbTag>,

    /// nesting level
    pub level: Option<u64>,

    pub class: BioSeqSetClass,
    pub release: Option<String>,
    pub date: Option<Date>,
    pub descr: Option<SeqDescr>,
    pub seq_set: Vec<SeqEntry>,
    pub annot: Option<Vec<SeqAnnot>>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum SeqEntry {
    Seq(BioSeq),
    Set(BioSeqSet),
}
