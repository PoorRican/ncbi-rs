//! Sequence location and identifier elements
//!
//! Adapted from [seqloc.asn](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqloc/seqloc.asn)
//! from NCBI C++ Toolkit.
//!
//! See [book](https://ncbi.github.io/cxx-toolkit/pages/ch_datamod#ch_datamod._ASN1_Specification_s_8)
//! for more information on.

use atoi::atoi;
use crate::biblio::IdPat;
use crate::general::{Date, DbTag, IntFuzz, ObjectId};
use crate::seqfeat::FeatId;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use crate::{XMLElement, XMLElementVec};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
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

    #[serde(rename="named-annot-track")]
    /// internal named annotation
    NamedAnnotTrack(TextseqId),
}

impl XMLElement for SeqId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Seq-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        // variants
        let other = BytesStart::new("Seq-id_other");
        let general = BytesStart::new("Seq-id_general");
        let gi = BytesStart::new("Seq-id_gi");

        loop {
            if let Event::Start(e) = reader.read_event().unwrap() {
                if e.name() == other.name() {
                    return SeqId::Other(TextseqId::from_reader(reader).unwrap()).into();
                }
                else if e.name() == general.name() {
                    return SeqId::General(DbTag::from_reader(reader).unwrap()).into();
                }
                else if e.name() == gi.name() {
                    if let Event::Text(text) = reader.read_event().unwrap() {
                        return SeqId::Gi(
                            atoi(text.as_ref()).expect("Can't parse &[u8] into int")
                        ).into();
                    }
                }
            }
        }
    }
}
impl XMLElementVec for SeqId {}

pub type SeqIdSet = Vec<SeqId>;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PatentSeqId {
    /// number of sequence in patent
    pub seqid: u64,

    /// patent citation
    pub cit: IdPat,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct TextseqId {
    pub name: Option<String>,
    pub accession: Option<String>,
    pub release: Option<String>,
    pub version: Option<u64>,
}

impl XMLElement for TextseqId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Textseq-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut name = None;
        let mut accession = None;
        let mut release = None;
        let mut version = None;

        let name_element = BytesStart::new("Textseq-id_name");
        let accession_element = BytesStart::new("Textseq-id_accession");
        let release_element = BytesStart::new("Textseq-id_release");
        let version_element = BytesStart::new("Textseq-id_version");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == name_element.name() {
                        if let Event::Text(text) = reader.read_event().unwrap() {
                            name = text.escape_ascii().to_string().into();
                        }
                    }
                    else if e.name() == accession_element.name() {
                        if let Event::Text(text) = reader.read_event().unwrap() {
                            accession = text.escape_ascii().to_string().into();
                        }
                    }
                    else if e.name() == release_element.name() {
                        if let Event::Text(text) = reader.read_event().unwrap() {
                            release = text.escape_ascii().to_string().into();
                        }
                    }
                    else if e.name() == version_element.name() {
                        if let Event::Text(text) = reader.read_event().unwrap() {
                            version = atoi(text.as_ref());
                        }
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        break;
                    }
                }
                _ => ()
            }
        }

        Self {
            name,
            accession,
            release,
            version
        }.into()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct GiimportId {
    pub id: i64,
    pub db: Option<String>,
    pub release: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
pub struct PDBSeqId {
    pub mol: PDBMolId,
    pub rel: Option<Date>,
    pub chain_id: Option<String>,
}

/// name of mol, should be 4 chars
pub type PDBMolId = String;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
pub struct SeqInterval {
    pub from: i64,
    pub to: i64,
    pub strand: Option<NaStrand>,
    pub id: SeqId,
    pub fuzz_from: Option<IntFuzz>,
    pub fuzz_to: Option<IntFuzz>,
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
#[serde(rename_all="kebab-case")]
pub struct PackedSeqPnt {
    pub strand: Option<NaStrand>,
    pub id: SeqId,
    pub fuzz: Option<IntFuzz>,
    pub points: Vec<i64>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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
