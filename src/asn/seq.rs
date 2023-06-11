//! NCBI Sequence Elements
//!
//! As per [NCBI C++ Toolkit Docs](https://ncbi.github.io/cxx-toolkit/pages/ch_datamod)
//!
//! Adapted from ["seq.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seq/seq.asn)

use crate::asn::{
    BioSource, Date, DbTag, EMBLBlock, GBBlock, IntFuzz, ModelEvidenceSupport, ObjectId, OrgRef,
    PDBBlock, PIRBlock, PRFBlock, PubEquiv, SPBlock, SeqAlign, SeqFeat, SeqGraph, SeqId, SeqLoc,
    SeqTable, UserObject,
};
use std::collections::HashSet;

/// Single continuous biological sequence.
/// It can be nucleic acid or protein. It can be fully instantiated (ie: data
/// for every residue) or only partially instantiated (eg: we know a frag is
/// 10kb long, but we only have data over 1kb)
pub struct BioSeq {
    /// equivalent identifiers
    pub id: Vec<SeqId>,
    /// descriptors
    pub descr: Option<SeqDescr>,
    /// the sequence data
    pub inst: SeqInst,
    pub annot: Option<Vec<SeqAnnot>>,
}

pub type SeqDescr = HashSet<SeqDesc>;

/// # Note
/// `MolType`, `Modif`, `Method`, and `Org` are consolidated and expanded
/// in [`OrgRef`]`, [`BioSource`], and [`MolInfo`] in this specification.
/// They will be removed in later specifications. Do not use them in the future.
/// Instead expect the new structures.
pub enum SeqDesc {
    #[deprecated]
    MolType(GIBBMol),
    /// type of molecule

    #[deprecated]
    /// modifiers
    Modif(GIBBMod),

    #[deprecated]
    /// sequencing method
    Method(GIBBMethod),

    /// name for this sequence
    Name(String),
    /// title for this sequence
    Title(String),

    #[deprecated]
    /// if all from one organism
    Org(OrgRef),

    /// a more extensive comment
    Comment(String),
    /// a numbering system
    Num(Numbering),
    /// map location of this sequence
    MapLoc(DbTag),
    /// PIR specific info
    PIR(PIRBlock),
    /// GenBank specific info
    Genbank(GBBlock),
    /// reference to a publication
    Pub(PubDesc),
    /// overall region (globin locus)
    Region(String),
    /// user defined object
    User(UserObject),
    /// SWISSPROT specific info
    SP(SPBlock),
    /// EMBL specific information
    DbXref(DbTag),
    /// xref to other databases
    Embl(EMBLBlock),
    /// date entry first created/released
    CreateDate(Date),
    UpdateDate(Date),
    /// PRF specific information
    PRF(PRFBlock),
    /// PDB specific information
    PDB(PDBBlock),
    /// Cofactor, etc associated but not bound
    Het(Heterogen),
    /// source of materials, includes [`OrgRef`]
    Source(BioSource),
    /// info on the molecule and techniques
    MolInfo(MolInfo),
    /// model evidence for XM records
    ModelEv(ModelEvidenceSupport),
}

#[allow(non_camel_case_types)]
#[derive(Default)]
/// Represents type of biomolecule
///
/// # Notes
/// Non-camelcase types look cleaner for names with "RNA"/"DNA"
pub enum BioMol {
    #[default]
    Unknown,
    Genomic,
    PreRNA,
    /// precursor RNA of any sort
    mRNA,
    rRNA,
    tRNA,
    snRNA,
    scRNA,
    Peptide,
    OtherGenetic,
    /// other genetic material
    Genomic_mRNA,
    /// reported a mix of genomic dna and cdna sequence
    cRNA,
    /// viral RNA genome copy intermediate
    snoRNA,
    /// small nucleolar RNA
    TranscribedRNA,
    /// transcribed RNA other than existing classes
    ncRNA,
    tmRNA,
    Other = 255,
}

#[allow(non_camel_case_types)]
#[derive(Default)]
pub enum MolTech {
    #[default]
    Unknown,
    /// standard sequencing
    Standard,
    /// Expressed Sequence Tag
    EST,
    /// Sequence Tagged Site
    STS,
    /// One-pass genomic sequence
    Survey,
    /// from genetic mapping techniques
    GeneMap,
    /// from physical mapping techniques
    PhysMap,
    /// derived from other data, not a primary entity
    Derived,
    /// conceptual translation
    ConceptTrans,
    /// peptide was sequenced
    SeqPept,
    /// concept transl. w/ partial pept. seq.
    Both,
    /// sequenced peptide, ordered by overlap
    SeqPeptOverlap,
    /// sequenced peptide, ordered by homology
    SeqPeptHomol,
    /// conceptual translation. supplied by author
    ConceptTransA,
    /// unordered High Throughput sequence contig
    HTGS1,
    /// ordered High Throughput sequence contig
    HTGS2,
    /// finished High Throughput sequence
    HTGS3,
    /// full length insert cDNA
    FLI_cDNA,
    /// single genomic reads for coordination
    HTGS0,
    /// high throughput cDNA,
    HTC,
    /// whole genome shotgun sequencing
    WGS,
    /// barcode of life project
    Barcode,
    /// composite of WGS and HTGS
    CompositeWH,
    /// transcriptome shotgun assembly
    TSA,
    /// targeted locus sets/studies
    Targeted,
    /// use `tech_exp` from [`MolInfo`]
    Other = 255,
}

#[derive(Default)]
/// Capture sequence completeness.
///
/// Completeness is not indicated in most records. For genomes, assume
/// the sequences are incomplete unless specifically marked as complete.
/// For mRNAs, assume the ends are not known exactly unless marked as having
/// the left or right end.
pub enum MolCompleteness {
    #[default]
    Unknown,
    Complete,
    Partial,
    NoLeft,
    NoRight,
    NoEnds,
    HasLeft,
    HasRight,
    Other = 255,
}

pub struct MolInfo {
    pub bio_mol: BioMol,
    pub tech: MolTech,
    /// explanation if `tech` not enough
    pub tech_exp: Option<String>,
    pub completeness: MolCompleteness,
    pub gb_mol_type: Option<String>,
}

#[allow(non_camel_case_types)]
/// GenInfo Backbone molecule types
///
/// Captures type of molecule represented
pub enum GIBBMol {
    Unknown,
    Genomic,
    /// Precursor RNA of any sort really
    PreRNA,
    mRNA,
    rRNA,
    tRNA,
    snRNA,
    scRNA,
    Peptide,
    /// other genetic material
    OtherGenetic,
    /// reported a mix of genomic and cDNA sequence
    Genomic_mRNA,
    Other = 255,
}

/// GenInfo Backbone Modifiers
pub enum GIBBMod {
    DNA,
    RNA,
    ExtraChrom,
    Plasmid,
    Mitochondrial,
    Chloroplast,
    Kinetoplast,
    Cyanelle,
    Synthetic,
    Recombinant,
    Partial,
    Complete,
    /// subject of mutagenesis
    Mutagen,
    /// natural mutant
    NatMut,
    Transposon,
    InsertionSeq,
    /// missing left end (5' for na, NH2 for aa)
    NoLeft,
    /// missing right end (3' for na, COOH for aa)
    NoRight,
    MacroNuclear,
    ProViral,
    /// expressed sequence tag
    EST,
    /// sequenced tagged site
    STS,
    /// one pass survey sequence
    Survey,
    Chromoplast,
    /// is a genetic map
    GeneMap,
    /// is an ordered restriction map
    RestMap,
    /// is a physical map (not ordered restriction map)
    PhysMap,
    Other = 255,
}

/// Sequencing method
pub enum GIBBMethod {
    /// Conceptual translation
    ConceptTrans,
    /// Peptide was sequenced
    SeqPept,
    /// concept transl. w/ partial pept. seq.
    Both,
    /// sequenced peptide, ordered by overlap
    SeqPeptOverlap,
    /// sequenced peptide, ordered by homology
    SeqPeptHomol,
    /// conceptual transl. supplied by author.
    ConceptTransA,
    Other = 255,
}

/// Any display numbering system
pub enum Numbering {
    /// continuous numbering
    Cont(NumCont),
    /// enumerated names for residues
    Enum(NumEnum),
    /// by reference to another sequence
    Ref(NumRef),
    /// supports mapping to a float system
    Real(NumReal),
}

#[derive(Default)]
/// continuous display numbering system
pub struct NumCont {
    /// number assigned to first residue
    /// TODO: should default to `1`
    pub ref_num: u64,

    /// 0-indexed?
    /// TODO: should default to `false`
    pub has_zero: bool,

    /// Ascending numbers
    /// TODO: should default to `true`
    pub ascending: bool,
}

/// any tags to residues
pub struct NumEnum {
    /// number of tags to follow
    pub num: u64,
    /// the tags
    pub names: Vec<String>,
}

/// type of reference
pub enum NumRefType {
    NotSet,
    /// by segmented or const seq sources
    Sources,
    /// by alignments given below
    Aligns,
}

/// Number by reference to other sequences
pub struct NumRef {
    /// type of reference
    pub r#type: NumRefType,
    /// alignments to pass for [`NumRefType::Aligns`]
    pub aligns: Option<SeqAlign>,
}

/// Mapping to floating point system
/// from an integer system used by [`BioSeq`]
/// `position = (a * int_position) + b`
pub struct NumReal {
    pub a: f64,
    pub b: f64,
    pub units: Option<String>,
}

#[derive(Default)]
/// type of reference in a GenBank record
pub enum PubDescRefType {
    #[default]
    /// refers to sequence
    Seq,
    /// refers to unspecified features
    Sites,
    /// refers to specified features
    Feats,
    /// nothing specified (EMBL)
    NoTarget,
}

pub struct PubDesc {
    pub r#pub: PubEquiv,
    pub name: Option<String>,
    pub fig: Option<String>,
    /// numbering from paper
    pub num: Option<Numbering>,
    /// numbering problem with paper
    pub num_exc: Option<bool>,
    /// poly A tail indicated in figure
    pub poly_a: Option<bool>,
    /// map location reported in paper
    pub map_loc: Option<String>,
    /// original sequence from paper
    pub seq_raw: Option<String>,
    /// this seq aligned with others in paper
    pub align_group: Option<i64>,
    /// any comment on this pub in context
    pub comment: Option<String>,
    /// type of reference in a GenBank record
    pub ref_type: PubDescRefType,
}

/// Cofactor, prosthetic group, inhibitor, etc
pub type Heterogen = String;

/// Representation class for [`SeqInst`]
///
/// Stored by [`SeqInst`] and is independent of [`Mol`]
///
/// Variants involve the particular data structure used to represent the knowledge
/// about the molecule, no matter which part of the molecule type branch it may be in.
/// The [`Repr`] element indicates the type of representation used. The aim of such a
/// set of representation classes is to supper the information to express different
/// views of sequence based objects, from chromosomes to restriction fragments,
/// from genetic maps to proteins, within a single overall model.
///
/// # Variants
/// - `Virtual`: used to describe a sequence about which we may have
///              information on the molecule itself but no sequence yet.
/// - `Raw`: used for what we traditionally consider a sequence. Molecule type,
///          strandedness, length, and sequence are known. In this case, [`SeqInst.seq_data`]
///          contains sequence data.
/// - `Seg`: A **segmented** representation is very analogous to a virtual representation.
///          It exists through references to other [`BioSeq`]'s, so there is
///          molecular information, but no `seq_data`. Only data is contained
///          by reference to other [`BioSeq`]'s in [`SeqInst::ext`] to hold an
///          array of [`SeqLoc`]. That is, the extension is an ordered series
///          of locations on *other* [`BioSeq`] objects. If one needed to
///          retrieve the base at the first position in the segmented [`BioSeq`],
///          one would go to the first [`SeqLoc`] in the extension, and return the
///          appropriate base from the [`BioSeq`] it points to.
/// - `Const`: A **constructed** [`BioSeq`] is used to describe an assembly or
///            merge of other [`BioSeq`]'s. It is analogous to the raw representation.
///            It is really meant for tracking higher level merging.
/// - `Map`: A **map** is akin to a virtual [`BioSeq`]. In the case where molecular
///          information is known, but there is no complete sequence data, [`SeqInst::ext`]
///          is a sequence of [`SeqFeat`] objects. For a genetic map, this
///          feature table contains [`GeneRef`] features. An ordered restriction
///          map would have a feature table containing [`RsiteRef`] features.
///          The feature table is part of [`SeqInst`] because for a map, it is
///          an essential part of instantiating the map [`BioSeq`], not merely
///          annotation on a known sequence.
pub enum Repr {
    /// empty
    NotSet,
    /// no seq data
    Virtual,
    /// continuous sequence
    Raw,
    /// segmented sequence
    Seg,
    /// constructed sequence
    Const,
    /// reference to another sequence
    Ref,
    /// consensus sequence or pattern
    Consen,
    /// ordered map of any kind
    Map,
    /// sequence made by changes (delta) to others
    Delta,
    Other = 255,
}

/// molecule class in living organism
///  > cdna = rna
pub enum Mol {
    NotSet,
    DNA,
    RNA,
    AA,
    /// just a nucleic acid
    NA,
    Other = 255,
}

#[derive(Default)]
/// Topology of biomolecule
pub enum Topology {
    NotSet,
    #[default]
    Linear,
    Circular,
    Tandem,
    Other = 255,
}

/// Strandedness in living organism
pub enum Strand {
    NotSet,
    /// single strand
    SS,
    /// double strand
    DS,
    Mixed,
    Other = 255,
}

/// Instances of sequences
///
/// Represents things like: is DNA, RNA, or protein? Is it circular or linear?
/// Double-stranded or single-stranded? How long is it?
pub struct SeqInst {
    /// representation class
    pub repr: Repr,

    /// molecule class in living organism
    pub mol: Mol,

    /// length of sequence in residues
    pub length: Option<u64>,

    /// length of uncertainty
    pub fuzz: Option<IntFuzz>,

    /// topology of molecule
    pub topology: Topology,

    /// strandedness in living organism
    pub strand: Strand,

    /// the sequence
    pub seq_data: Option<SeqData>,

    /// extensions for special types
    pub ext: Option<SeqExt>,

    /// sequence history
    pub hist: Option<SeqHist>,
}

// Sequence extensions for representing more complex types

pub enum SeqExt {
    /// segmented sequences
    Seg(SegExt),

    /// hot link to another sequence (a view)
    Ref(RefExt),

    /// ordered map of markers
    Map(MapExt),

    Delta(DeltaExt),
}

pub type SegExt = Vec<SeqLoc>;
pub type RefExt = SeqLoc;
pub type MapExt = Vec<SeqFeat>;
pub type DeltaExt = Vec<DeltaSeq>;

pub enum DeltaSeq {
    /// point to a sequence
    Loc(SeqLoc),

    /// a piece of sequence
    Literal(SeqLiteral),
}

pub struct SeqLiteral {
    /// must give a length in residues
    pub length: u64,

    /// could be unsure
    pub full: Option<IntFuzz>,

    /// may have the data
    pub seq_data: Option<SeqData>,
}

/// internal structure for storing sequence history deletion status
pub enum SeqHistDeleted {
    Bool(bool),
    Date(Date),
}

/// Sequence history record
/// assembly: records how seq was assembled from others
pub struct SeqHist {
    pub assembly: Option<HashSet<SeqAlign>>,
    pub replaces: Option<SeqHistRec>,
    pub replaced_by: Option<SeqHistRec>,
    pub deleted: Option<SeqHistDeleted>,
}

pub struct SeqHistRec {
    pub date: Option<Date>,
    pub ids: HashSet<SeqId>,
}

/// Sequence representations
pub enum SeqData {
    /// IUPAC 1 letter nuc acid code
    Ina(IUPACna),

    /// IUPAC 1 letter amino acid code
    Iaa(IUPACaa),

    /// 2 bit nucleic acid code
    N2na(NCBI2na),

    /// 4 bit nucleic acid code
    N4na(NCBI4na),

    /// 8 bit extended nucleic acid code
    N8na(NCBI8na),

    /// nucleic acid probabilities
    NPna(NCBIpna),

    /// 8 bit extended amino acid codes
    N8aa(NCBI8aa),

    /// extended ASCII 1 letter aa codes
    NEaa(NCBIeaa),

    /// amino acid probabilities
    NPaa(NCBIpaa),

    /// consecutive codes for std aa's
    NStdAAs(NCBIstdaa),

    /// gap types
    Gap(SeqGap),
}

/// internal structure for `type` field in [`SeqGap`]
pub enum SeqGapType {
    Unknown,
    #[deprecated]
    /// used only for AGP 1.1
    Fragment,
    #[deprecated]
    /// used only for AGP 1.1
    Clone,
    ShortArm,
    Heterochromatin,
    Centromere,
    Telomere,
    Repeat,
    Contig,
    Scaffold,
    Contamination,
    Other = 255,
}

pub enum SeqGapLinkage {
    Unlinked,
    Linked,
    Other = 255,
}

pub struct SeqGap {
    pub r#type: SeqGapType,
    pub linkage: Option<SeqGapLinkage>,
    pub linkage_evidence: Option<HashSet<LinkageEvidence>>,
}

/// internal representation for `type` in [`LinkageEvidence`]
pub enum LinkageEvidenceType {
    PairedEnds,
    AlignGenus,
    AlignXGenus,
    AlignTrans,
    WithinClone,
    CloneContig,
    Map,
    Strobe,
    Unspecified,
    PCR,
    ProximityLigation,
    Other = 255,
}

pub struct LinkageEvidence {
    pub r#type: LinkageEvidenceType,
}

/// IUPAC 1 letter codes, no spaces
pub type IUPACna = String;

/// IUPAC 1 letter codes, no spaces
pub type IUPACaa = String;

/// 00=A, 01=C, 10=G, 11=T
pub type NCBI2na = Vec<u8>;

/// 1 bit for each agct
/// 0001=A, 0010=C, 0100=G, 1000=T/U
/// 0101/Purine, 1010=Pyrimidine, etc
pub type NCBI4na = Vec<u8>;

/// For modified nucleic acids
pub type NCBI8na = Vec<u8>;

/// 5 octets/base, prob for a,c,g,t,n
/// Probabilities are coded 0-255 = 0.0-1.0
pub type NCBIpna = Vec<u8>;

/// For modified amino acids
pub type NCBI8aa = Vec<u8>;

/// ASC extended 1 letter aa codes
/// IUPAC Codes + U=selenocysteine
pub type NCBIeaa = String;

/// 25/octets/aa, prob for IUPAC aa's in order:
/// A-Y, B, Z, X, (ter), anything
///
/// Probabilities are coded 0-255 = 0.0-1.0
pub type NCBIpaa = Vec<u8>;

/// Codes 0-25, 1 per byte
pub type NCBIstdaa = Vec<u8>;

/// This is a replica of [`TextSeqId`]
/// This is specific for annotations, and exists to maintain a semantic difference
/// between ID's assigned to annotations and ID's assigned to sequences.
pub struct TextAnnotId {
    pub name: Option<String>,
    pub accession: Option<String>,
    pub release: Option<String>,
    pub version: Option<u64>,
}

pub enum AnnotId {
    Local(ObjectId),
    NCBI(u64),
    General(DbTag),
    Other(TextAnnotId),
}

pub type AnnotDescr = Vec<AnnotDesc>;

pub enum AnnotDesc {
    /// a short name for this collection
    Name(String),
    /// a title for this collection
    Title(String),
    /// a more extensive comment
    Comment(String),
    /// a reference to the publication
    Pub(PubDesc),
    /// user defined object
    User(UserObject),
    /// date entry first created/released
    CreateDate(Date),
    /// date of last update
    UpdateDate(Date),
    /// source sequence from which annot came
    Src(SeqId),
    /// definition of the SeqAligns
    Align(AlignDef),
    /// all contents cover this region
    Region(SeqLoc),
}

/// Class of align [`SeqAnnot`]
pub enum AlignType {
    /// set of alignments to the same sequence
    Ref,
    /// set of alternate alignments of the same seqs
    Alt,
    /// set of aligned blocks in the same seqs
    Blocks,
    Other = 255,
}

pub struct AlignDef {
    pub align_type: AlignType,
    /// used for the one ref [`SeqId`] for now
    pub ids: Option<HashSet<SeqId>>,
}

pub enum SeqAnnotDB {
    GenBank,
    EMBL,
    DDBJ,
    PIR,
    SP,
    BBone,
    PDB,
    Other = 255,
}

/// Internal representation for `data` choice in [`SeqAnnot`]
pub enum SeqAnnotData {
    FTable(HashSet<SeqFeat>),
    Align(HashSet<SeqAlign>),
    Graph(HashSet<SeqGraph>),
    /// used for communication between tools
    IDS(HashSet<SeqId>),
    /// used for communication between tools
    Locs(HashSet<SeqLoc>),
    /// features in table form
    SeqTable(SeqTable),
}

pub struct SeqAnnot {
    pub id: Option<HashSet<AnnotId>>,
    pub db: Option<SeqAnnotDB>,
    /// source if `db` [`SeqAnnotDB::Other`]
    pub name: Option<String>,
    /// used only for standalone [`SeqAnnot`]'s
    pub desc: Option<AnnotDescr>,
    pub data: SeqAnnotData,
}
