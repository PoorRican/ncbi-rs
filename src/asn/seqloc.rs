//! Sequence location and identifier elements
//!
//! Adapted from [seqloc.asn](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqloc/seqloc.asn)
//! from NCBI C++ Toolkit.
//!
//! See [book](https://ncbi.github.io/cxx-toolkit/pages/ch_datamod#ch_datamod._ASN1_Specification_s_8)
//! for more information on.

use std::collections::HashSet;
use crate::asn::{IdPat, Date, IntFuzz, ObjectId, FeatId};

pub enum SeqId {
    Local(ObjectId),
    GibbSq(i64),
    GibbMt(i64),
    Giim(GiimportId),
    Genbank(TextseqId),
    Embl(TextseqId),
    Pir(TextseqId),
    Swissprot(TextseqId),
    Patent(PatentSeqId),
    Other(TextseqId)
}

pub type SeqIdSet = HashSet<SeqId>;

pub struct PatentSeqId {
    /// number of sequence in patent
    pub seqid: u64,

    /// patent citation
    pub cit: IdPat
}

pub struct TextseqId {
    pub name: Option<String>,
    pub accession: Option<String>,
    pub release: Option<String>,
    pub version: Option<u64>,
}

pub struct GiimportId {
    pub id: i64,
    pub db: Option<String>,
    pub release: Option<String>,
}

pub struct PDBSeqId {
    pub mol: PDBMolId,
    pub rel: Option<Date>,
    pub chain_id: Option<String>,
}

/// name of mol, should be 4 chars
pub type PDBMolId = String;

/// Defines a location on a [`BioSeq`].
///
/// Class hierarchy makes it possible to use the same type in multiple contexts.
///
/// # Variants
/// - `Null`: indicates a region of unknown length for which no data exists.
///           Such a location may be used in a segmented sequence for the region
///           between two sequenced fragments about which nothing, not even length,
///           is known.
/// - Other [`SeqLoc`] types, (except [`SeqLoc::Feat`]) contain a [`SeqId`]. This
///   means that data objects describing information about [`BioSeq`]'s can be
///   created and exchanged independently from the [`BioSeq`] itself. This
///   encourages the development and exchange of structured knowledge about
///   sequence data from many directions and is an essential goal of the data
///   model.
pub enum SeqLoc {
    /// not placed
    Null,
    /// to NULL one [`SeqId`] in a collection
    Empty(SeqId),
    /// Whole sequence
    Whole(SeqId),
    /// from/to
    Int(SeqInterval),
    PackedInt(PackedSeqInt),
    Pnt(SeqPoint),
    PackedPnt(PackedSeqPnt),
    Mix(SeqLocMix),
    /// equivalent sets of locations
    Equiv(SeqLocEquiv),
    Bond(SeqBond),
    /// indirect, through a [`SeqFeat`]
    Feat(FeatId)
}

pub struct SeqInterval {
    pub from: i64,
    pub to: i64,
    pub strand: Option<NaStrand>,
    pub id: SeqId,
    pub fuzz_from: Option<IntFuzz>,
    pub fuzz_to: Option<IntFuzz>,
}

pub type PackedSeqInt = HashSet<SeqInterval>;

pub struct SeqPoint {
    pub point: i64,
    pub strand: Option<NaStrand>,
    pub id: SeqId,
    pub fuzz: Option<IntFuzz>,
}

pub struct PackedSeqPnt {
    pub strand: Option<NaStrand>,
    pub id: SeqId,
    pub fuzz: Option<IntFuzz>,
    pub points: Vec<i64>,
}

/// Strand of nucleic acid
pub enum NaStrand {
    Unknown,
    Plus,
    Minus,
    /// in forward orientation
    Both,
    /// in reverse orientation
    BothRev,
    Other = 255,
}

/// bond between residues
pub struct SeqBond {
    /// connection to at least one residue
    pub a: SeqPoint,

    /// other end may not be available
    pub b: Option<SeqPoint>
}

/// this will hold anything
pub type SeqLocMix = Vec<SeqLoc>;
/// set of equivalent locations
pub type SeqLocEquiv = HashSet<SeqLoc>;
