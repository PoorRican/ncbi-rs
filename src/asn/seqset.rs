//! NCBI Sequence Collections
//!
//! Adapted from ["seqset.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqset/seqset.asn)
//! from the NCBI C++ Toolkit

use crate::general::{Date, DbTag, ObjectId};
use crate::parsing_utils::{parse_vec_node_to, read_node};
use crate::seq::{BioSeq, SeqAnnot, SeqDescr};
use crate::{XmlNode, XmlVecNode};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
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

impl XmlNode for BioSeqSet {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Bioseq-set")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let seq_set_element = BytesStart::new("Bioseq-set_seq-set");

        let mut set = Self::default();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_vec_node_to(&name, &seq_set_element, &mut set.seq_set, reader);
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return set.into();
                    }
                }
                _ => (),
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum SeqEntry {
    Seq(BioSeq),
    Set(BioSeqSet),
}

impl XmlNode for SeqEntry {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Seq-entry")
    }
    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let seq = BytesStart::new("Seq-entry_seq");
        let set = BytesStart::new("Seq-entry_set");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == seq.name() {
                        return Self::Seq(read_node(reader).unwrap()).into();
                    }
                    if name == set.name() {
                        return Self::Set(read_node(reader).unwrap()).into();
                    }
                }
                Event::End(e) => {
                    // correctly escape "Seq-entry"
                    if Self::is_end(&e) {
                        return None;
                    }
                }
                _ => (),
            }
        }
    }
}
impl XmlVecNode for SeqEntry {}
