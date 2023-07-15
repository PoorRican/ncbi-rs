//! Sequence location and identifier elements
//!
//! Adapted from [seqloc.asn](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqloc/seqloc.asn)
//! from NCBI C++ Toolkit.
//!
//! See [book](https://ncbi.github.io/cxx-toolkit/pages/ch_datamod#ch_datamod._ASN1_Specification_s_8)
//! for more information on.

use crate::biblio::IdPat;
use crate::general::{Date, DbTag, IntFuzz, ObjectId};
use crate::parsing::{check_unexpected, read_attributes, read_int, read_node, read_string};
use crate::seqfeat::FeatId;
use crate::parsing::{XmlNode, XmlVecNode, XmlValue};
use quick_xml::events::{BytesStart, Event};
use quick_xml::events::attributes::Attributes;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SeqId {
    Local(ObjectId),
    /// GenInfo backbone sequence id
    GibbSq(i64),

    /// GenInfo backbone molecule type
    GibbMt(i64),

    /// GenINfo import id
    Giim(GiimportId),

    Genbank(TextseqId),
    Embl(TextseqId),
    Pir(TextseqId),
    Swissprot(TextseqId),
    Patent(PatentSeqId),
    /// left for historical reasons, `Other = ReqSeq`
    Other(TextseqId),

    /// for other databases
    General(DbTag),

    /// GenInfo integrated database
    Gi(u64),

    /// DDBJ
    Ddbj(TextseqId),

    /// PRF SEQDB
    Prf(TextseqId),

    /// PDB sequence
    Pdb(PDBSeqId),

    /// Third party annot/seq: Genbank
    Tpg(TextseqId),

    /// Third party annot/seq: EMBL
    Tpe(TextseqId),

    /// Third party annot/seq: DDBJ
    Tpd(TextseqId),

    /// internal NCBI genome pipeline
    Gpipe(TextseqId),

    #[serde(rename = "named-annot-track")]
    /// internal named annotation
    NamedAnnotTrack(TextseqId),
}

impl XmlNode for SeqId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Seq-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        // variants
        let other_element = BytesStart::new("Seq-id_other");
        let general_element = BytesStart::new("Seq-id_general");
        let gi_element = BytesStart::new("Seq-id_gi");
        let genbank_element = BytesStart::new("Seq-id_genbank");

        loop {
            if let Event::Start(e) = reader.read_event().unwrap() {
                if e.name() == other_element.name() {
                    return SeqId::Other(read_node(reader).unwrap()).into();
                }
                if e.name() == general_element.name() {
                    return SeqId::General(read_node(reader).unwrap()).into();
                } else if e.name() == gi_element.name() {
                    return SeqId::Gi(read_int(reader).unwrap()).into();
                } else if e.name() == genbank_element.name() {
                    return SeqId::Genbank(read_node(reader).unwrap()).into();
                }
            }
        }
    }
}
impl XmlVecNode for SeqId {}

pub type SeqIdSet = Vec<SeqId>;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PatentSeqId {
    /// number of sequence in patent
    pub seqid: u64,

    /// patent citation
    pub cit: IdPat,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct TextseqId {
    pub name: Option<String>,
    pub accession: Option<String>,
    pub release: Option<String>,
    pub version: Option<u64>,
}

impl XmlNode for TextseqId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Textseq-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut id = Self::default();

        let name_element = BytesStart::new("Textseq-id_name");
        let accession_element = BytesStart::new("Textseq-id_accession");
        let release_element = BytesStart::new("Textseq-id_release");
        let version_element = BytesStart::new("Textseq-id_version");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == name_element.name() {
                        id.name = read_string(reader);
                    } else if name == accession_element.name() {
                        id.accession = read_string(reader);
                    } else if name == release_element.name() {
                        id.release = read_string(reader);
                    } else if name == version_element.name() {
                        id.version = read_int(reader);
                    } else if name != Self::start_bytes().name() {
                        check_unexpected(&name, &[]);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return id.into();
                    }
                }
                _ => (),
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct GiimportId {
    pub id: i64,
    pub db: Option<String>,
    pub release: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PDBSeqId {
    pub mol: PDBMolId,
    pub rel: Option<Date>,
    pub chain_id: Option<String>,
}

/// name of mol, should be 4 chars
pub type PDBMolId = String;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
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
    Feat(FeatId),
}

impl SeqLoc {
    /// default not originally in spec
    pub fn default() -> Self {
        Self::Null
    }
}

impl XmlNode for SeqLoc {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Seq-loc")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variant tags
        let null_variant = BytesStart::new("Seq-loc_null");
        let int_variant = BytesStart::new("Seq-loc_int");
        let empty_variant = BytesStart::new("Seq-loc_empty");
        let whole_variant = BytesStart::new("Seq-loc_whole");
        let packed_int_variant = BytesStart::new("Seq-loc_packed-int");
        let pnt_variant = BytesStart::new("Seq-loc_pnt");
        let packed_pnt_variant = BytesStart::new("Seq-loc_packed_pnt");
        let mix_variant = BytesStart::new("Seq-loc_mix");
        let equiv_variant = BytesStart::new("Seq-loc_equiv");
        let bond_variant = BytesStart::new("Seq-loc_bond");
        let feat_variant = BytesStart::new("Seq-loc_feat");

        let forbidden = [
            null_variant,
            empty_variant,
            packed_int_variant,
            pnt_variant,
            packed_pnt_variant,
            mix_variant,
            equiv_variant,
            bond_variant,
            feat_variant
        ];

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == int_variant.name() {
                        return Self::Int(read_node(reader).unwrap()).into()
                    } else if name == whole_variant.name() {
                        return Self::Whole(read_node(reader).unwrap()).into()
                    } else if name != Self::start_bytes().name() {
                        check_unexpected(&name, &forbidden);
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return None
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SeqInterval {
    pub from: i64,
    pub to: i64,
    pub strand: Option<NaStrand>,
    pub id: SeqId,
    pub fuzz_from: Option<IntFuzz>,
    pub fuzz_to: Option<IntFuzz>,
}

impl Default for SeqInterval {
    fn default() -> Self {
        Self {
            from: 0,
            to: 0,
            strand: None,
            id: SeqId::Other(TextseqId::default()),
            fuzz_from: None,
            fuzz_to: None
        }
    }
}

impl XmlNode for SeqInterval {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Seq-interval")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut interval = SeqInterval::default();

        // elements
        let from_element = BytesStart::new("Seq-interval_from");
        let to_element = BytesStart::new("Seq-interval_to");
        // this tag is skipped, and `Empty` tag for `NaStrand` is used instead
        let _strand_element = BytesStart::new("Seq-interval_strand");
        let id_element = BytesStart::new("Seq-interval_id");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == from_element.name() {
                        interval.from = read_int(reader).unwrap();
                    } else if name == to_element.name() {
                        interval.to = read_int(reader).unwrap();
                    } else if name == id_element.name() {
                        interval.id = read_node(reader).unwrap();
                    }
                }
                Event::Empty(e) => {
                    if e.name() == NaStrand::start_bytes().name() {
                        interval.strand = read_attributes(&e);
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return interval.into()
                    }
                }
                _ => ()
            }
        }
    }
}

pub type PackedSeqInt = Vec<SeqInterval>;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct SeqPoint {
    pub point: i64,
    pub strand: Option<NaStrand>,
    pub id: SeqId,
    pub fuzz: Option<IntFuzz>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PackedSeqPnt {
    pub strand: Option<NaStrand>,
    pub id: SeqId,
    pub fuzz: Option<IntFuzz>,
    pub points: Vec<i64>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
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

impl XmlValue for NaStrand {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Na-strand")
    }

    fn from_attributes(attributes: Attributes) -> Option<Self> {
        let value = BytesStart::new("value");
        for attribute in attributes {
            if let Ok(attr) = attribute {
                if attr.key == value.name() {
                    let _inner = attr.unescape_value().unwrap().to_string();
                    let inner = _inner.get(2.._inner.len()-2).unwrap();
                    if inner == "unknown" {
                        return Self::Unknown.into()
                    }
                    if inner == "plus" {
                        return Self::Plus.into()
                    }
                    if inner == "minus" {
                        return Self::Minus.into()
                    }
                    if inner == "both" {
                        return Self::Both.into()
                    }
                    if inner == "both-rev" {
                        return Self::BothRev.into()
                    }
                }
            }
        }
        return None
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// bond between residues
pub struct SeqBond {
    /// connection to at least one residue
    pub a: SeqPoint,

    /// other end may not be available
    pub b: Option<SeqPoint>,
}

/// this will hold anything
pub type SeqLocMix = Vec<SeqLoc>;
/// set of equivalent locations
pub type SeqLocEquiv = Vec<SeqLoc>;
