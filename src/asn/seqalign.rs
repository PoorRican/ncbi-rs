//! Sequence Alignment elements
//!
//! Adapted from ["seqalign.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqalign/seqalign.asn)

use crate::general::{ObjectId, UserObject};
use crate::seqloc::{NaStrand, SeqId, SeqLoc};
use crate::parsing::{read_vec_node, read_int, read_vec_int_unchecked, read_node, read_string, UnexpectedTags, read_bool_attribute};
use crate::parsing::{XmlNode, XmlVecNode};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use quick_xml::events::Event;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

pub type SeqAlignSet = Vec<SeqAlign>;

/// Internal representation of alignment type for [`SeqAlign`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer
#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
pub enum SeqAlignType {
    #[default]
    NotSet,

    Global,
    /// unbroken, but not ordered, diagonals
    Diags,
    /// mapping pieces together
    Partial,
    /// discontinuous alignment
    Disc,

    Other = 255,
}

impl XmlNode for SeqAlignType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("type")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        match reader.read_event().unwrap() {
            Event::Start(_e) => {
                let value: u8 = read_int(reader).unwrap();
                match value {
                    0 => Some(SeqAlignType::NotSet),
                    1 => Some(SeqAlignType::Global),
                    2 => Some(SeqAlignType::Diags),
                    3 => Some(SeqAlignType::Partial),
                    4 => Some(SeqAlignType::Disc),
                    255 => Some(SeqAlignType::Other),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl XmlVecNode for SeqAlignType {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SeqAlignSegs {
    DenDiag(Vec<DenseDiag>),
    DenSeg(DenseSeg),
    Std(Vec<StdSeg>),
    Packed(PackedSeg),
    Disc(SeqAlignSet),
    Spliced(SplicedSeg),
    Sparse(SparseSeg),
}

impl Default for SeqAlignSegs {
    fn default() -> Self {
        SeqAlignSegs::Std(Vec::new())
    }
}

impl XmlNode for SeqAlignSegs {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("segs")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"den-diag" => return Some(SeqAlignSegs::DenDiag(read_vec_node(reader, e.to_end()))),
                        b"den-seg" => return Some(SeqAlignSegs::DenSeg(read_node(reader).unwrap())),
                        b"std" => return Some(SeqAlignSegs::Std(read_vec_node(reader, e.to_end()))),
                        b"packed" => return Some(SeqAlignSegs::Packed(read_node(reader).unwrap())),
                        b"disc" => return Some(SeqAlignSegs::Disc(read_vec_node(reader, e.to_end()))),
                        b"spliced" => return Some(SeqAlignSegs::Spliced(read_node(reader).unwrap())),
                        b"sparse" => return Some(SeqAlignSegs::Sparse(read_node(reader).unwrap())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return None;
                }
                _ => (),
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct SeqAlign {
    #[serde(rename = "type")]
    pub r#type: SeqAlignType,
    /// dimensionality
    pub dim: Option<u64>,
    /// for whole alignment
    pub score: Option<Vec<Score>>,
    /// alignment data
    pub segs: SeqAlignSegs,
    /// regions of sequence over which
    /// alignment was computed
    pub bounds: Option<Vec<SeqLoc>>,
    /// alignment id
    pub id: Option<Vec<ObjectId>>,
    /// extra info
    pub ext: Option<Vec<UserObject>>,
}

impl XmlNode for SeqAlign {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Seq-align")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut align = SeqAlign::default();

        /*
        let type_tag = BytesStart::new("type");
        let dim_tag = BytesStart::new("dim");
        let score_tag = BytesStart::new("score");
        let segs_tag = BytesStart::new("segs");
        let bounds_tag = BytesStart::new("bounds");
        let id_tag = BytesStart::new("id");
        let ext_tag = BytesStart::new("ext");
        */

        let forbidden_tags = [BytesStart::new("unknown-tag")];
        let forbidden = UnexpectedTags(&forbidden_tags);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    match name.as_ref() {
                        b"Seq-align_type" => align.r#type = read_node(reader).unwrap(),
                        b"Seq-align_dim" => align.dim = Some(read_int(reader).unwrap()),
                        b"Seq-align_score" => align.score = Some(read_vec_node(reader, e.to_end())),
                        b"Seq-align_segs" => align.segs = read_node(reader).unwrap(),
                        b"Seq-align_bounds" => align.bounds = Some(read_vec_node(reader, e.to_end())),
                        b"Seq-align_id" => align.id = Some(read_vec_node(reader, e.to_end())),
                        b"Seq-align_ext" => align.ext = Some(read_vec_node(reader, e.to_end())),
                        _ => forbidden.check(&name),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(align);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for SeqAlign {}


fn default_num_dimensions() -> u64 {
    2
}
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
/// for (multiway) diagonals
pub struct DenseDiag {
    /// dimensionality
    #[serde(default = "default_num_dimensions")]
    pub dim: u64,
    /// sequences in order
    pub ids: Vec<SeqId>,
    /// start OFFSETS in ids order
    pub starts: Vec<u64>,
    /// len of aligned segments
    pub len: u64,
    pub strands: Option<Vec<NaStrand>>,
    pub scores: Option<Vec<Score>>,
}

impl XmlNode for DenseDiag {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("dense-diag")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut dense_diag = DenseDiag::default();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    let end = e.to_end();
                    match name.as_ref() {
                        b"dim" => dense_diag.dim = read_int(reader).unwrap(),
                        b"ids" => dense_diag.ids = read_vec_node(reader, e.to_end()),
                        b"starts" => dense_diag.starts = read_vec_int_unchecked(reader, &end),
                        b"len" => dense_diag.len = read_int(reader).unwrap(),
                        b"strands" => dense_diag.strands = Some(read_vec_node(reader, e.to_end())),
                        b"scores" => dense_diag.scores = Some(read_vec_node(reader, e.to_end())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(dense_diag);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for DenseDiag {}

/// The densest packing for sequence alignments only.
///
/// # Description
///
/// A start of -1 indicates a gap for that sequence of length lens.
///
///  id=100  AAGGCCTTTTAGAGATGATGATGATGATGA
///  id=200  AAGGCCTTTTAG.......GATGATGATGA
///  id=300  ....CCTTTTAGAGATGATGAT....ATGA
///
///  dim = 3, numseg = 6, ids = { 100, 200, 300 }
///  starts = { 0,0,-1, 4,4,0, 12,-1,8, 19,12,15, 22,15,-1, 26,19,18 }
///  lens = { 4, 8, 7, 3, 4, 4 }
///
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct DenseSeg {
    /// dimensionality
    #[serde(default = "default_num_dimensions")]
    pub dim: u64,
    /// number of segments here
    pub numseg: u64,
    /// sequences in order
    pub ids: Vec<SeqId>,
    /// start OFFSETS in ids order within segs
    pub starts: Vec<u64>,
    /// lengths in ids order within segs
    pub lens: Vec<u64>,
    pub strands: Option<Vec<NaStrand>>,
    /// score for each seg
    pub scores: Option<Vec<Score>>,
}

impl XmlNode for DenseSeg {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("dense-seg")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut dense_seg = DenseSeg::default();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    let end=e.to_end();
                    match name.as_ref() {
                        b"dim" => dense_seg.dim = read_int(reader).unwrap(),
                        b"numseg" => dense_seg.numseg = read_int(reader).unwrap(),
                        b"ids" => dense_seg.ids = read_vec_node(reader, e.to_end()),
                        b"starts" => dense_seg.starts = read_vec_int_unchecked(reader, &end),
                        b"lens" => dense_seg.lens = read_vec_int_unchecked(reader, &end),
                        b"strands" => dense_seg.strands = Some(read_vec_node(reader, e.to_end())),
                        b"scores" => dense_seg.scores = Some(read_vec_node(reader, e.to_end())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(dense_seg);
                }
                _ => (),
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// for (multiway) global or partial alignments
pub struct PackedSeg {
    // TODO: default 2
    /// dimensionality
    pub dim: u64,

    /// number of segments here
    pub numseg: u64,

    /// sequences in order
    pub ids: Vec<SeqId>,

    /// start OFFSETS in ids order for whole alignment
    pub starts: Vec<u64>,

    /// Boolean if each sequence present or absent in each segment
    pub present: Vec<u8>,

    /// length of each segment
    pub lens: Vec<u64>,

    pub strands: Option<Vec<NaStrand>>,

    /// score for each segment
    pub scores: Option<Vec<Score>>,
}

impl XmlNode for PackedSeg {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("packed-seg")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut packed_seg = PackedSeg {
            dim: 2, // Default value
            numseg: 0,
            ids: Vec::new(),
            starts: Vec::new(),
            present: Vec::new(),
            lens: Vec::new(),
            strands: None,
            scores: None,
        };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    let end=e.to_end();
                    match name.as_ref() {
                        b"dim" => packed_seg.dim = read_int(reader).unwrap(),
                        b"numseg" => packed_seg.numseg = read_int(reader).unwrap(),
                        b"ids" => packed_seg.ids = read_vec_node(reader, e.to_end()),
                        b"starts" => packed_seg.starts = read_vec_int_unchecked(reader, &end),
                        b"present" => packed_seg.present = read_vec_int_unchecked(reader, &end),
                        b"lens" => packed_seg.lens = read_vec_int_unchecked(reader, &end),
                        b"strands" => packed_seg.strands = Some(read_vec_node(reader, e.to_end())),
                        b"scores" => packed_seg.scores = Some(read_vec_node(reader, e.to_end())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(packed_seg);
                }
                _ => (),
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct StdSeg {
    // TODO: default 2
    /// dimensionality
    pub dim: u64,

    /// sequences in order
    pub ids: Option<Vec<SeqId>>,

    pub loc: Vec<SeqLoc>,

    /// score for each segment
    pub scores: Option<Vec<Score>>,
}

impl XmlNode for StdSeg {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("std-seg")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut std_seg = StdSeg {
            dim: 2, // Default value
            ids: None,
            loc: Vec::new(),
            scores: None,
        };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"dim" => std_seg.dim = read_int(reader).unwrap(),
                        b"ids" => std_seg.ids = Some(read_vec_node(reader, e.to_end())),
                        b"loc" => std_seg.loc = read_vec_node(reader, e.to_end()),
                        b"scores" => std_seg.scores = Some(read_vec_node(reader, e.to_end())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(std_seg);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for StdSeg {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SplicedSegProduct {
    Transcript,
    Protein,
}

impl XmlNode for SplicedSegProduct {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("product-type")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        match reader.read_event().unwrap() {
            Event::Start(e) => {
                let name = e.name();
                match name.as_ref() {
                    b"transcript" => Some(SplicedSegProduct::Transcript),
                    b"protein" => Some(SplicedSegProduct::Protein),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl XmlVecNode for SplicedSegProduct {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SplicedSeg {
    /// product is either protein or transcript (cDNA)
    pub product_id: Option<SeqId>,
    pub genomic_id: Option<SeqId>,

    /// should be 'plus' or 'minus'
    pub product_strand: Option<NaStrand>,
    pub genomic_strand: Option<NaStrand>,

    pub product_type: SplicedSegProduct,

    /// set of segments involved each segment corresponds
    /// to one exon.
    ///
    /// Exons are always in biological order.
    pub exons: Vec<SplicedExon>,

    /// start of poly(A) tail on the transcript
    /// For sense transcripts:
    ///     `aligned product positions` < `poly_a` <= `product_length`
    ///     `poly_a == product_length`
    ///     indicates inferred poly(A) tail at transcript's end
    /// For anti-sense transcripts:
    ///     -1 <= `poly_a` < `align product positions`
    ///     `poly_a == -1`
    ///     indicates inferred poly(a) tail at transcript's start
    pub poly_a: Option<i64>,

    /// length of the product, in bases/residues
    ///
    /// from this (or from poly_a, if present),
    /// a 3' aligned length can be extracted
    pub product_length: Option<u64>,

    /// alignment descriptors / modifiers
    ///
    /// this provides a set for extension
    pub modifiers: Option<Vec<SplicedSegModifier>>,
}

impl XmlNode for SplicedSeg {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("spliced-seg")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut spliced_seg = SplicedSeg {
            product_id: None,
            genomic_id: None,
            product_strand: None,
            genomic_strand: None,
            product_type: SplicedSegProduct::Transcript, // Default value
            exons: Vec::new(),
            poly_a: None,
            product_length: None,
            modifiers: None,
        };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"product-id" => spliced_seg.product_id = Some(read_node(reader).unwrap()),
                        b"genomic-id" => spliced_seg.genomic_id = Some(read_node(reader).unwrap()),
                        b"product-strand" => spliced_seg.product_strand = Some(read_node(reader).unwrap()),
                        b"genomic-strand" => spliced_seg.genomic_strand = Some(read_node(reader).unwrap()),
                        b"product-type" => spliced_seg.product_type = read_node(reader).unwrap(),
                        b"exons" => spliced_seg.exons = read_vec_node(reader, e.to_end()),
                        b"poly-a" => spliced_seg.poly_a = Some(read_int(reader).unwrap()),
                        b"product-length" => spliced_seg.product_length = Some(read_int(reader).unwrap()),
                        b"modifiers" => spliced_seg.modifiers = Some(read_vec_node(reader, e.to_end())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(spliced_seg);
                }
                _ => (),
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum SplicedSegModifier {
    /// start found for protein/product or genomic alignment
    StartCodonFound(bool),

    /// stop found for protein/product or genomic alignment
    StopCodonFound(bool),
}

impl XmlNode for SplicedSegModifier {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("modifier")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        match reader.read_event().unwrap() {
            Event::Start(e) => {
                let name = e.name();
                match name.as_ref() {
                    b"start-codon-found" => {
                        let value = read_bool_attribute( &e);
                        Some(SplicedSegModifier::StartCodonFound(value?))
                    }
                    b"stop-codon-found" => {
                        let value = read_bool_attribute( &e);
                        Some(SplicedSegModifier::StopCodonFound(value?))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl XmlVecNode for SplicedSegModifier {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "kebab-case")]
/// Complete or partial exon
///
/// Two consecutive [`SplicedExon`]'s may belong to one exon
pub struct SplicedExon {
    /// `product_end >= product_start`
    pub product_start: ProductPos,
    pub product_end: ProductPos,

    /// `genomic_end >= genomic_start`
    pub genomic_start: i64,
    pub genomic_end: i64,

    /// product is either protein or transcript (cDNA)
    pub product_id: Option<SeqId>,
    pub genomic_id: Option<SeqId>,

    /// should be 'plus' or 'minus'
    pub product_strand: Option<NaStrand>,

    /// represents the strand of translation
    pub genomic_strand: Option<NaStrand>,

    /// basic segments always are in biologic order
    pub parts: Option<Vec<SplicedExonChunk>>,

    /// scores for this exon
    pub scores: Option<ScoreSet>,

    /// splice sites
    pub acceptor_before_exon: Option<SpliceSite>,
    pub donor_after_exon: Option<SpliceSite>,

    /// flag: is this exon complete or partial?
    pub partial: Option<bool>,

    /// extra info
    pub ext: Option<Vec<UserObject>>,
}

impl XmlNode for SplicedExon {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("spliced-exon")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut spliced_exon = SplicedExon::default();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    let end = e.to_end();
                    match name.as_ref() {
                        b"product-start" => spliced_exon.product_start = read_node(reader).unwrap(),
                        b"product-end" => spliced_exon.product_end = read_node(reader).unwrap(),
                        b"genomic-start" => spliced_exon.genomic_start = read_int(reader).unwrap(),
                        b"genomic-end" => spliced_exon.genomic_end = read_int(reader).unwrap(),
                        b"product-id" => spliced_exon.product_id = Some(read_node(reader).unwrap()),
                        b"genomic-id" => spliced_exon.genomic_id = Some(read_node(reader).unwrap()),
                        b"product-strand" => spliced_exon.product_strand = Some(read_node(reader).unwrap()),
                        b"genomic-strand" => spliced_exon.genomic_strand = Some(read_node(reader).unwrap()),
                        b"parts" => spliced_exon.parts = Some(read_vec_node(reader, e.to_end())),
                        b"scores" => spliced_exon.scores = Some(read_vec_node(reader,end)),
                        b"acceptor-before-exon" => spliced_exon.acceptor_before_exon = Some(read_node(reader).unwrap()),
                        b"donor-after-exon" => spliced_exon.donor_after_exon = Some(read_node(reader).unwrap()),
                        b"partial" => spliced_exon.partial = read_bool_attribute(&e),
                        b"ext" => spliced_exon.ext = Some(read_vec_node(reader, e.to_end())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(spliced_exon);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for SplicedExon {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProductPos {
    NucPos(u64),
    ProtPos(ProtPos),
}

impl Default for ProductPos {
    fn default() -> Self {
        ProductPos::NucPos(0)
    }
}

impl XmlNode for ProductPos {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("product-pos")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"nuc-pos" => return Some(ProductPos::NucPos(read_int(reader).unwrap())),
                        b"prot-pos" => return Some(ProductPos::ProtPos(read_node(reader).unwrap())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return None;
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for ProductPos {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// codon based position on protein (1/3 of aminoacid)
pub struct ProtPos {
    /// standard protein position
    pub amin: u64,

    /// 0, 1, 2, or 3 as for [`CdRegion`]
    /// 0 = not set
    /// 1, 2, 3 = actual frame
    pub frame: usize,
}

impl XmlNode for ProtPos {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("prot-pos")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut prot_pos = ProtPos { amin: 0, frame: 0 };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"amin" => prot_pos.amin = read_int(reader)?,
                        b"frame" => prot_pos.frame = read_int(reader)?,
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(prot_pos);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for ProtPos {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// Piece of an exon
///
/// Each variant contains lengths given in nucleotide bases
/// (1/3 of amino acid when product is a protein)
pub enum SplicedExonChunk {
    /// both sequences represented, product and genomic sequences match
    Match(u64),

    /// both sequences represented, product and genomic sequences do not match
    Mismatch(u64),

    /// both sequences are represented, there is sufficient similarity
    /// between product and genomic sequences. Can be used to replace
    /// stretches of matches and mismatches, mostly for protein to genomic
    /// where definition of match or mismatch depends on translation table
    Diag(u64),

    /// Insertion in product sequence
    /// (ie: gap in the genomic sequence)
    ProductIns(u64),

    /// Insertion in product sequence
    /// (ie: gap in the genomic sequence)
    GenomicIns(u64),
}

impl XmlNode for SplicedExonChunk {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("spliced-exon-chunk")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"match" => return Some(SplicedExonChunk::Match(read_int(reader).unwrap())),
                        b"mismatch" => return Some(SplicedExonChunk::Mismatch(read_int(reader).unwrap())),
                        b"diag" => return Some(SplicedExonChunk::Diag(read_int(reader).unwrap())),
                        b"product-ins" => return Some(SplicedExonChunk::ProductIns(read_int(reader).unwrap())),
                        b"genomic-ins" => return Some(SplicedExonChunk::GenomicIns(read_int(reader).unwrap())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return None;
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for SplicedExonChunk {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// site involved in splice
pub struct SpliceSite {
    /// typically two bases in the introgenic region,
    /// always in IUPAC format
    pub bases: String,
}

impl XmlNode for SpliceSite {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("splice-site")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut splice_site = SpliceSite { bases: String::new() };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"bases" => splice_site.bases = read_string(reader).unwrap(),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(splice_site);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for SpliceSite {}
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// [`SparseSeg`] follows the semantics of [`DenseSeg`] and is optimized
/// for representing sparse multiple alignments.
pub struct SparseSeg {
    pub master_id: Option<SeqId>,
    /// pairwise alignments constituting this multiple alignment
    pub rows: Vec<SparseAlign>,

    /// per-row scores
    pub row_scores: Option<Vec<Score>>,

    /// index of extra items
    pub ext: Option<Vec<SparseSegExt>>,
}

impl XmlNode for SparseSeg {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("sparse-seg")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut sparse_seg = SparseSeg {
            master_id: None,
            rows: Vec::new(),
            row_scores: None,
            ext: None,
        };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"master-id" => sparse_seg.master_id = Some(read_node(reader).unwrap()),
                        b"rows" => sparse_seg.rows = read_vec_node(reader, e.to_end()),
                        b"row-scores" => sparse_seg.row_scores = Some(read_vec_node(reader, e.to_end())),
                        b"ext" => sparse_seg.ext = Some(read_vec_node(reader, e.to_end())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(sparse_seg);
                }
                _ => (),
            }
        }
    }

}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SparseAlign {
    pub first_id: SeqId,
    pub second_id: SeqId,

    /// number of segments
    pub numseg: u64,

    /// starts on the first sequence (`numseg`)
    pub first_starts: Vec<u64>,

    /// starts on the second sequence (`numseg`)
    pub second_starts: Vec<u64>,

    /// lengths of segments (`numseg`)
    pub lens: Vec<u64>,

    pub second_strands: Option<Vec<NaStrand>>,

    /// per-segment scores
    pub seg_scores: Option<Vec<Score>>,
}

impl XmlNode for SparseAlign {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("sparse-align")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut sparse_align = SparseAlign {
            first_id: SeqId::default(),
            second_id: SeqId::default(),
            numseg: 0,
            first_starts: Vec::new(),
            second_starts: Vec::new(),
            lens: Vec::new(),
            second_strands: None,
            seg_scores: None,
        };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    let end = e.to_end();
                    match name.as_ref() {
                        b"first-id" => sparse_align.first_id = read_node(reader).unwrap(),
                        b"second-id" => sparse_align.second_id = read_node(reader).unwrap(),
                        b"numseg" => sparse_align.numseg = read_int(reader).unwrap(),
                        b"first-starts" => sparse_align.first_starts = read_vec_int_unchecked(reader, &end),
                        b"second-starts" => sparse_align.second_starts = read_vec_int_unchecked(reader, &end),
                        b"lens" => sparse_align.lens = read_vec_int_unchecked(reader, &end),
                        b"second-strands" => sparse_align.second_strands = Some(read_vec_node(reader, e.to_end())),
                        b"seg-scores" => sparse_align.seg_scores = Some(read_vec_node(reader, e.to_end())),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(sparse_align);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for SparseAlign {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct SparseSegExt {
    pub index: u64,
}

impl XmlNode for SparseSegExt {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("sparse-seg-ext")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut sparse_seg_ext = SparseSegExt { index: 0 };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"index" => sparse_seg_ext.index = read_int(reader).unwrap(),
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(sparse_seg_ext);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for SparseSegExt {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ScoreValue {
    Real(f64),
    Int(i64),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Use of [`Score`] is discouraged for external ASN.1 specifications
pub struct Score {
    pub id: Option<ObjectId>,
    pub value: ScoreValue,
}

impl XmlNode for Score {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("score")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut score = Score {
            id: None,
            value: ScoreValue::Int(0), // Default value, will be overwritten
        };

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    match name.as_ref() {
                        b"id" => score.id = Some(read_node(reader).unwrap()),
                        b"value" => {
                            let value_str: String = read_string(reader).unwrap();
                            if let Ok(int_value) = value_str.parse::<i64>() {
                                score.value = ScoreValue::Int(int_value);
                            } else if let Ok(real_value) = value_str.parse::<f64>() {
                                score.value = ScoreValue::Real(real_value);
                            }
                        }
                        _ => (),
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return Some(score);
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Score {}

pub type ScoreSet = Vec<Score>;
