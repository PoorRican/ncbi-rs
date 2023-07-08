//! NCBI Sequence Collections
//!
//! Adapted from ["seqset.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqset/seqset.asn)
//! from the NCBI C++ Toolkit

use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use crate::general::{Date, DbTag, ObjectId};
use crate::seq::{BioSeq, SeqAnnot, SeqDescr};
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::XMLElement;

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

impl XMLElement for BioSeqSet {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Bioseq-set")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let seq_set_element = BytesStart::new("Bioseq-set_seq-set");

        let mut id = None;
        let mut coll = None;
        let mut level = None;
        let mut class = BioSeqSetClass::default();
        let mut release = None;
        let mut date = None;
        let mut descr = None;
        let mut annot = None;
        let mut seq_set= Vec::new();

        println!("Starting to parse BioSeqSet");
        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == seq_set_element.name() {
                        seq_set = SeqEntry::vec_from_reader(reader, seq_set_element.to_end());
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        break;
                    }
                }
                Event::Eof => break,
                _ => ()
            }
        }

        Self {
            id,
            coll,
            level,
            class,
            release,
            date,
            descr,
            seq_set,
            annot,
        }.into()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum SeqEntry {
    Seq(BioSeq),
    Set(BioSeqSet),
}

impl XMLElement for SeqEntry {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Seq-entry")
    }
    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let seq = BytesStart::new("Seq-entry_seq");
        let set = BytesStart::new("Seq-entry_set");

        let mut entry = None;

        println!("Beginning to parse Seq-entry");
        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    println!("Got here");
                    if e.name() == seq.name() {
                        entry = Self::Seq(BioSeq::from_reader(reader).unwrap()).into();
                    }
                    else if e.name() == set.name() {
                        entry = Self::Set(BioSeqSet::from_reader(reader).unwrap()).into();
                    }
                }
                Event::End(e) => {
                    // correctly escape "Seq-entry"
                    if Self::start_bytes().to_end().name() == e.name() {
                        break;
                    }
                }
                _ => (),
            }
        }
        println!("Finished parsing Seq-entry");

        entry
    }
}
