//! Sequence feature elements
//!
//! Adapted from ["seqfeat.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqfeat/seqfeat.asn)
//! and documented by [NCBI C++ Toolkit Book](https://ncbi.github.io/cxx-toolkit/pages/ch_datamod#ch_datamod.datamodel.seqfeat)
//!
/// A feature table is a collection of sequence features called [`SeqFeat`]s.
/// A [`SeqFeat`] serves to connect a sequence location ([`SeqLoc`]) with a
/// specific block of data known as a datablock. Datablocks are defined
/// objects on their own and are often used in other contexts, such as
/// publications ([`PubSet`]), references to organisms ([`OrgRef`]), or genes
/// ([`GeneRef`]). Some datablocks, like coding regions ([`CdRegion`]), only make
/// sense when considered within the context of a [`SeqLoc`]. However, each
/// datablock is designed to fulfill a specific purpose and is independent
/// of others. This means that changes or additions to one datablock do not
/// affect the others.
///
/// When a pre-existing object from another context is used as a datablock,
/// any software capable of utilizing that object can also operate on the
/// feature. For example, code that displays a publication can function with a
/// publication from a bibliographic database or one used as a sequence
/// feature without any modifications.
///
/// The [`SeqFeat`] data structure and the [`SeqLoc`] used to attach it to the
/// sequence are shared among all features. This allows for a set of operations
/// that can be performed on all features, regardless of the type of datablocks
/// attached to them. Therefore, a function designed to determine all features
/// in a specific region of a Bioseq does not need to consider the specific
/// types of features involved.
///
/// A [`SeqFeat`] is referred to as bipolar because it can have up to two
/// [`SeqLoc`]s. The [`SeqFeat`].location indicates the "source" and represents
/// the location on the DNA sequence, similar to the single location in common
/// feature table implementations. The `product` from [`SeqFeat`] represents
/// the "sink" and is typically associated with the protein sequence produced.
/// For example, a [`CdRegion`] feature would have its [`SeqFeat`].location on
/// the DNA and its [`SeqFeat`].product on the corresponding protein sequence.
/// This usage defines the process of translating a DNA sequence into a protein
/// sequence, establishing an explicit relationship between nucleic acid and
/// protein sequence databases.
///
/// Having two [`SeqLoc`]s allows for a more comprehensive representation of
/// data conflicts or exceptional biological circumstances. For instance,
/// if an author presents a DNA sequence and its protein product in a figure,
/// it is possible to enter the DNA and protein sequences independently and
/// then confirm through the [`CdRegion`] feature that the DNA indeed translates
/// to the provided protein sequence. In cases where the DNA and protein do
/// not match, it could indicate an error in the database or the original
/// paper. By setting a "conflict" flag in the CdRegion, the problem can be
/// identified early and addressed. Additionally, there may be situations
/// where a genomic sequence cannot be translated to a protein due to known
/// biological reasons, such as RNA editing or suppressor tRNAs. In such
/// cases, the [`SeqFeat`] can be marked with an "exception" flag to indicate
/// that the data is correct but may not behave as expected.

use crate::biblio::{DOI, PubMedId};
use crate::general::{DbTag, IntFuzz, ObjectId, UserObject};
use crate::r#pub::PubSet;
use crate::seq::{Heterogen, Numbering, PubDesc, SeqLiteral};
use crate::seqloc::{GiimportId, SeqId, SeqLoc};
use std::collections::BTreeSet;

#[derive(PartialEq, Debug)]
/// Feature identifiers
pub enum FeatId {
    /// GenInfo backbone
    GIBB(u64),

    /// GenInfo import
    GIIM(GiimportId),

    /// for local software use
    Local(ObjectId),

    /// for use by various databases
    General(DbTag),
}

#[derive(PartialEq, Debug)]
/// Experimental evidence for existence of feature
pub enum SeqFeatExpEvidence {
    /// any reasonable experiment check
    Experimental = 1,

    /// similarity, pattern, etc
    NotExperimental,
}

#[derive(PartialEq, Debug)]
/// Sequence feature generalization
pub struct SeqFeat {
    pub id: Option<FeatId>,

    /// the specific data
    pub data: SeqFeatData,

    /// incomplete in some way?
    pub partial: Option<bool>,

    /// something funny about this?
    pub except: Option<bool>,

    pub comment: Option<String>,

    /// product of process
    pub product: Option<SeqLoc>,

    /// feature made from
    pub location: SeqLoc,

    /// qualifiers
    pub qual: Option<Vec<GbQual>>,

    /// for user defined label
    pub title: Option<String>,

    /// user defined structure extension
    pub ext: Option<UserObject>,

    /// citations for this feature
    pub cit: Option<PubSet>,

    /// evidence for existence of feature
    pub exp_ev: Option<SeqFeatExpEvidence>,

    /// cite other relevant features
    pub xref: Option<BTreeSet<SeqFeatXref>>,

    /// support for xref to other databases
    pub dbxref: Option<BTreeSet<DbTag>>,

    /// annotated on pseudogene
    pub pseudo: Option<bool>,

    /// explain if `except=true`
    pub except_text: Option<String>,

    /// set of id's; will replace `id` field
    pub ids: Option<BTreeSet<FeatId>>,

    /// set of extensions; will replace `ext` field
    pub exts: Option<BTreeSet<UserObject>>,

    pub support: Option<SeqFeatSupport>,
}

#[derive(PartialEq, Debug)]
pub enum SeqFeatBond {
    Disulfide,
    Thiolester,
    XLink,
    Thioether,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub enum SeqFeatSite {
    Active,
    Binding,
    Cleavage,
    Inhibit,
    Modified,
    Clycosylation,
    Myristoylation,
    Mutagenized,
    MetalBinding,
    Phosphorylation,
    Acetylation,
    Amidation,
    Methylation,
    Hydroxylation,
    Sulfatation,
    OxidativeDeamination,
    PyrrolidoneCarboxylicAcid,
    GammaCarboxylglutamicAcid,
    Blocked,
    LipidBinding,
    NPBinding,
    DNABinding,
    SignalPeptide,
    TransitPeptide,
    TransmembraneRegion,
    Nitrosylation,
    Other = 255,
}

#[derive(PartialEq, Debug)]
/// Protein secondary structure
pub enum PSecStr {
    /// any helix
    Helix,

    /// beta sheet
    Sheet,

    /// beta or gamma turn
    Turn,
}

#[derive(PartialEq, Debug)]
pub enum SeqFeatData {
    Gene(GeneRef),
    Org(OrgRef),
    CdRegion(CdRegion),
    Prot(ProtRef),
    RNA(RnaRef),

    /// publication applies to this seq
    Pub(PubDesc),

    /// to annotate origin from another seq
    Seq(SeqLoc),

    Imp(ImpFeat),

    /// named region (ie: "globin locus")
    Region(String),
    Bond(SeqFeatBond),

    Site(SeqFeatSite),

    /// restriction site (for maps really)
    RSite(RSiteRef),

    /// user defined structure
    User(UserObject),

    /// transcription initiation
    TxInit(TxInit),

    /// a numbering system
    Num(Numbering),

    /// protein secondary structure
    PSecStr(PSecStr),

    /// non-standard residue here in seq
    NonStdResidue(String),

    /// cofactor, prosthetic group, etc, bound to seq
    Het(Heterogen),

    BioSrc(BioSource),
    Clone(CloneRef),
    Variation(VariationRef),
}

#[derive(PartialEq, Debug)]
pub struct SeqFeatXref {
    pub id: Option<FeatId>,
    pub data: Option<SeqFeatData>,
}

#[derive(PartialEq, Debug)]
pub struct SeqFeatSupport {
    pub experiment: Option<BTreeSet<ExperimentSupport>>,
    pub inference: Option<BTreeSet<InferenceSupport>>,
    pub model_evidence: Option<BTreeSet<ModelEvidenceSupport>>,
}

#[derive(PartialEq, Debug)]
pub enum EvidenceCategory {
    NotSet,
    Coordinates,
    Description,
    Existence,
}

#[derive(PartialEq, Debug)]
pub struct ExperimentSupport {
    pub category: Option<EvidenceCategory>,
    pub explanation: String,
    pub pmids: Option<BTreeSet<PubMedId>>,
    pub dois: Option<BTreeSet<DOI>>,
}

#[derive(PartialEq, Debug)]
pub struct ProgramId {
    pub name: String,
    pub version: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct EvidenceBasis {
    pub programs: Option<BTreeSet<ProgramId>>,
    pub accessions: Option<BTreeSet<SeqId>>,
}

#[derive(PartialEq, Debug, Default)]
pub enum InferenceSupportType {
    #[default]
    NotSet,
    SimilarToSequence,
    SimilarToAA,
    SimilarToDNA,
    SimilarToRNA,
    SimilarTomRNA,
    SimilarToEst,
    SimilarToOtherRNA,
    Profile,
    NucleotideMotif,
    ProteinMotif,
    AbInitioPrediction,
    Alignment,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub struct InferenceSupport {
    pub category: Option<EvidenceCategory>,
    pub r#type: InferenceSupportType,
    pub other_type: Option<String>,
    // TODO: default to false
    pub same_species: bool,
    pub basis: EvidenceBasis,
    pub pmids: Option<BTreeSet<PubMedId>>,
    pub soids: Option<BTreeSet<DOI>>,
}

#[derive(PartialEq, Debug)]
pub struct ModelEvidenceItem {
    pub id: SeqId,
    pub exon_count: Option<u64>,
    pub exon_length: Option<u64>,
    // TODO: default to false
    pub full_length: bool,
    // TODO: default to false
    pub supports_all_exon_combo: bool,
}

#[derive(PartialEq, Debug)]
pub struct ModelEvidenceSupport {
    pub method: Option<String>,
    pub mrna: Option<BTreeSet<ModelEvidenceItem>>,
    pub est: Option<BTreeSet<ModelEvidenceItem>>,
    pub protein: Option<BTreeSet<ModelEvidenceItem>>,
    pub identification: Option<SeqId>,
    pub dbxref: Option<BTreeSet<DbTag>>,
    pub exon_count: Option<u64>,
    pub exon_length: Option<u64>,
    pub full_length: bool,             // TODO: default false
    pub supports_all_exon_combo: bool, // TODO: default false
}

#[derive(PartialEq, Debug, Default)]
pub enum CdRegionFrame {
    #[default]
    /// not set, code uses one
    NotSet,
    One,
    Two,
    /// reading frame
    Three,
}

#[derive(PartialEq, Debug)]
/// Instructions to translate from a nucleic acid to a peptide
pub struct CdRegion {
    /// just an ORF ?
    pub orf: Option<bool>,

    pub frame: CdRegionFrame,


    /// supposed to translate, but doesn't
    pub conflict: Option<bool>,

    /// number of gaps on conflict/except
    pub gaps: Option<u64>,

    /// number of mismatches on above
    pub mismatch: Option<u64>,

    /// genetic code used
    pub code: Option<GeneticCode>,

    /// individual exceptions
    pub code_break: Option<Vec<CodeBreak>>,

    /// number of stop codons on above
    ///
    /// ### Original Comment:
    ///     each code is 64 cells long, in the order where:
    ///     `T=0, C=1, A=2, G=3, TTT=0, TTC=1, CTA=4, ...`
    ///
    ///     NOTE: this order does NOT correspond to a [`SeqData`] encoding.
    ///     It is "natural" to codon usage instead. The value in each cell is
    ///     the AA coded for start=AA coded only if first in peptide in start
    ///     array, if codon is not a legitimate start codon, that cell will
    ///     have the "gap" symbol for that alphabet. Otherwise it will have the
    ///     AA encoded when that codon is used at the start.
    pub stops: Option<u64>,
}

#[derive(PartialEq, Debug)]
/// Storage type for genetic code data
///
/// I do not know what purpose the "start" variants [`Self::SNcbiStdAa`],
/// [`Self::SNcbi8aa`], [`Self::SNcbiEaa`] serve.
pub enum GeneticCodeOpt {
    /// name of a code
    Name(String),

    /// id in dbase
    Id(u64),

    /// indexed to [`crate::asn::NCBIEaa`]
    NcbiEaa(String),

    /// indexed to [`crate::asn::NCBI8aa`]
    NCBI8aa(Vec<u8>),

    /// indexed to [`crate::asn::NCBIStdAa`]
    NCBIStdAa(Vec<u8>),

    /// start, indexed to [`crate::asn::NCBIEaa`]
    SNcbiEaa(String),

    /// start, indexed to [`crate::asn::NCBI8aa`]
    SNcbi8aa(Vec<u8>),

    /// start, indexed to [`crate::asn::NCBIStdAa`]
    SNcbiStdAa(Vec<u8>),
}

pub type GeneticCode = BTreeSet<GeneticCodeOpt>;

#[derive(PartialEq, Debug)]
/// the amino acid that is the exception
pub enum CodeBreakAA {
    /// ASCII value of [`crate::asn::NCBIEaa`] code
    NcbiAa(u64),

    /// [`crate::asn::NCBI8aa`] code
    Ncbi8aa(u64),

    /// [`crate::asn::NCBIStdAa`] code
    NcbiStdAa(u64),
}

#[derive(PartialEq, Debug)]
/// specific codon exceptions
pub struct CodeBreak {
    /// location of exception
    pub loc: SeqLoc,

    /// the amino acid that is the exception
    pub aa: CodeBreakAA,
}

/// table of genetic codes
pub type GeneticCodeTable = BTreeSet<GeneticCode>;

#[derive(PartialEq, Debug)]
/// Features imported from other databases
pub struct ImpFeat {
    pub key: String,

    /// original location string
    pub loc: Option<String>,

    /// text description
    pub descr: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct GbQual {
    pub qual: String,
    pub val: String,
}

#[derive(PartialEq, Debug)]
pub enum CloneRefPlacementMethod {
    /// clone placed by end sequence
    EndSeq,

    /// clone placed by insert alignment
    InsertAlignment,

    /// clone placed by STS
    STS,

    Fish,
    Fingerprint,

    /// combined end-seq and insert align
    EndSeqInsertAlignment,

    /// placement provided externally
    External = 253,

    /// human placed or approved
    Curated = 254,

    Other = 255,
}

#[derive(PartialEq, Debug)]
/// Specification of clone features
pub struct CloneRef {
    /// official clone symbol
    pub name: String,

    /// library name
    pub library: Option<String>,


    pub concordant: bool,       // TODO: default to false
    pub unique: bool,           // TODO: default to false
    pub placement_method: Option<CloneRefPlacementMethod>,
    pub clone_seq: Option<CloneSeqSet>,
}

pub type CloneSeqSet = BTreeSet<CloneSeq>;

#[derive(PartialEq, Debug)]
pub enum CloneSeqType {
    Insert,
    End,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub enum CloneSeqConfidence {
    /// multiple hits
    Multiple,

    /// unspecified
    NA,

    /// no hits, end flagged repetitive
    NoHitRep,

    /// no hits, end not flagged repetitive
    NoHitNoRep,

    /// hit on different chromosome
    OtherChrm,

    Unique,

    /// virtual (hasn't been sequenced)
    Virtual,

    /// multiple hits, end flagged repetitive
    MultipleRep,

    /// multiple hits, end not flagged repetitive
    MultipleNoRep,

    /// no hits
    NoHit,

    Other = 255,
}

#[derive(PartialEq, Debug)]
pub enum CloneSeqSupport {
    /// sequence used to place clone
    Prototype,

    /// sequence supports placement
    Supporting,

    /// supports a different placement
    SupportsOther,

    /// dose not support any placement
    NonSupporting,
}

#[derive(PartialEq, Debug)]
pub struct CloneSeq {
    pub r#type: CloneSeqType,
    pub confidence: Option<CloneSeqConfidence>,

    /// location on sequence
    pub location: SeqLoc,

    /// clone sequence location
    pub seq: Option<SeqLoc>,

    /// internal alignment identifier
    pub align_id: Option<DbTag>,
    pub support: Option<CloneSeqSupport>,
}

#[derive(PartialEq, Debug)]
pub enum VariantResourceLink {
    /// Clinical, Pubmed, Cited
    Preserved = 1,

    /// Provisional third party annotations
    Provisional = 2,

    /// has 3D structure in SNP3D table
    Has3D = 4,

    /// SNP -> SubSNP -> Batch link_out
    SubmitterLinkout = 8,

    /// Clinical if LSDB, OMIM, TPA, Diagnostic
    Clinical = 16,

    /// Marker exists on high density genotyping kit
    GenotypeKit = 32,
}

#[derive(PartialEq, Debug)]
pub enum VariantGeneLocation {
    /// sequence intervals covered by a gene ID but not
    /// having an aligned transcript
    InGene = 1,

    /// within 2kb of the 5' end of a gene feature
    NearGene5 = 2,

    /// within 0.5kb of the 3' end of a gene feature
    NearGene3 = 4,

    /// in intron
    Intron = 8,

    /// in donor splice-site
    Donor = 16,

    /// in acceptor splice-site
    Acceptor = 32,

    /// in 5' UTR
    UTR5 = 64,

    /// in 3' UTR
    UTR3 = 128,

    /// the variant is observed in a start codon
    InStartCodon = 256,

    /// the variant is observed in a stop codon
    InStopCodon = 512,

    /// variant located between genes
    Intergenic = 1024,

    /// variant is located in a conserved non-coding region
    ConservedNonCoding = 2048,
}

#[derive(PartialEq, Debug)]
pub enum VariantEffect {
    /// known to cause no functional changes.
    /// since value is 0, it does not combine with any other bit, explicitly
    /// implying there are no consequences from SNP
    NoChange = 0,

    /// one allele in the set does not change the encoded amino aced
    Synonymous = 1,

    /// one allele in the set changes to STOP codon
    Nonsense = 2,

    /// one allele in the set changes protein peptide
    Missense = 4,

    /// one allele in the set changes all downstream amino acids
    Frameshift = 8,

    /// the variant causes increased transcription
    UpRegulator = 16,

    /// the variant causes decreased transcription
    DownRegulator = 32,

    Methylation = 64,

    /// reference codon is not stop codon, but the SNP variant allele changes
    /// the codon to a termination codon
    StopGain = 128,

    /// reverse of [`Self::StopGain`]: reference codon is a stop codon, but
    /// a SNP variant allele changes the codon to a non-termination codon.
    StopLose = 256,
}

#[derive(PartialEq, Debug)]
pub enum VariantMapping {
    /// another SNP has the same mapped positions on reference assembly
    HasOtherSNP = 1,

    /// weight 1 or 2 SNPs that map to different chromosomes on
    /// different assemblies
    HasAssemblyConflict = 2,

    /// Only maps to 1 assembly
    IsAssemblySpecific = 4,
}

#[derive(PartialEq, Debug)]
/// captures specificity of placement
///
/// # Note
///
/// This is *NOT* a bitfield
pub enum VariantMapWeight {
    IsUniquelyPlaced = 1,
    PlacedTwiceOnSameChrom = 2,
    PlacedTypeOnDiffChrom = 3,
    ManyPlacements = 10,
}

#[derive(PartialEq, Debug)]
pub enum FrequencyBasedValidation {
    /// low frequency variation that is cited in journal or other reputable
    /// source.
    IsMutation = 1,

    /// >5% minor allele freq in each and all populations
    Above5Pct1Plus = 2,

    /// bit is set if the variant has a minor allele observed in two or
    /// more separate chromosomes.
    Validated = 4,

    /// >1% minor allele frequency in each and all populations
    Above1PctAll = 8,

    /// >1% minor allele frequency in 1+ populations
    Above1Pct1Plus = 32,
}

#[derive(PartialEq, Debug)]
pub enum VariantGenotype {
    /// exists in a haplotype tagging set
    InHaplotypeSet,

    /// SNP has individual genotype
    HasGenotypes,
}

#[derive(PartialEq, Debug)]
pub enum VariantQualityCheck {
    /// reference sequence allele at the mapped position is not present in
    /// the SNP allele list, adjust for orientation
    ContigAlleleMissing = 1,

    /// one member SS is withdrawn by submitter
    WithdrawnBySubmitter = 2,

    /// RS set has 2+ alleles from different submissions and these sets
    /// share no alleles in common
    NonOverlappingAlleles = 4,

    /// strain specific fixed difference
    StrainSpecific = 8,

    /// has genotype conflict
    GenotypeConflict = 16,
}

#[derive(PartialEq, Debug)]
pub enum VariantConfidence {
    Unknown,
    LikelyArtifact,
    Other = 255,
}

#[derive(PartialEq, Debug)]
/// origin of this allele, if known
///
/// Note is a bitflag and more than one state can be represented at once
pub enum VariantAlleleOrigin {
    Unknown = 0,
    Germline = 1,
    Somatic = 2,
    Inherited = 4,
    Paternal = 8,
    Maternal = 16,
    DeNovo = 32,
    Biparental = 64,
    Uniparental = 128,
    NotTested = 256,
    TestedInconclusive = 512,
    NotReported = 1024,
    Other = 10732741824,
}

#[derive(PartialEq, Debug)]
/// observed allele state, if known
///
/// NOTE this field is not a bitflag
pub enum VariantAlleleState {
    Unknown,
    Homosygous,
    Heterozygous,
    Hemizygous,
    Nullizygous,
    Other = 255,
}

#[derive(PartialEq, Debug)]
/// Specification of variation features
///
/// The intention is to provide information to clients that reflect internal
/// information produced during the mapping of SNPs.
///
/// # Note
/// The most of these values are bitflags, an integer that represents
/// a bitwise OR (simple sum) of the possible values, unless otherwise stated
///
/// # Original comment
///
/// This comment comes from deprecated values from [`VariationRef`]
/// for fields that were moved here, specifically `allele_frequency`, and
/// `quality_codes`:
///
///     The case of multiple alleles for a SNP would be described by
///     parent-features of type `VariationSet.diff_alleles`, where the child
///     features of type `VariationINst`, all at the same location, would
///     describe individual alleles.
///
pub struct VariantProperties {
    pub version: u64,
    pub resource_link: Option<VariantResourceLink>,
    pub gene_location: Option<VariantGeneLocation>,
    pub effect: Option<VariantEffect>,
    pub mapping: Option<VariantMapping>,

    /// specificity of placement
    pub map_weight: Option<VariantMapWeight>,

    pub frequency_based_validation: Option<FrequencyBasedValidation>,
    pub genotype: Option<VariantGenotype>,
    pub quality_check: Option<VariantQualityCheck>,
    pub confidence: VariantConfidence,

    /// has this variant been validated?
    ///
    /// While a boolean flag offers no subtle distinctions of validation
    /// methods, occasionally it is only known as a single bool value.
    ///
    /// ### Note
    /// This flag is redundant and should be omitted if more comprehensive
    /// validation information is present
    pub other_validation: Option<bool>,

    /// origin of allele, if known
    pub allele_origin: Option<VariantAlleleOrigin>,

    /// observed allele state, if known
    pub allele_state: Option<VariantAlleleState>,

    /// minor allele frequency of the default population
    pub allele_frequency: Option<f64>,

    /// is this variant the ancestral allele
    pub is_ancestral_allele: Option<bool>,
}

#[derive(PartialEq, Debug)]
/// does this variant have known clinical significance?
pub enum PhenotypeClinicalSignificance {
    Unknown,
    Untested,
    NonPathogenic,
    ProbableNonPathogenic,
    ProbablePathogenic,
    Pathogenic,
    DrugResponse,
    Histocompatibility,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub struct Phenotype {
    pub source: Option<String>,
    pub term: Option<String>,
    pub xref: Option<BTreeSet<DbTag>>,

    /// does this variant have known clinical significance?
    pub clinical_significance: Option<PhenotypeClinicalSignificance>,
}

#[derive(PartialEq, Debug)]
/// This field is an explicit bitflag
pub enum PopulationDataFlags {
    IsDefaultPopulation = 1,
    IsMinorAllele = 2,
    IsRareAllele = 4,
}

#[derive(PartialEq, Debug)]
pub struct PopulationData {
    /// assayed population (eg: HAPMAP-CEU)
    pub population: String,
    pub genotype_frequency: Option<f64>,
    pub chromosomes_tested: Option<u64>,
    pub sample_ids: Option<BTreeSet<ObjectId>>,
    pub allele_frequency: Option<f64>,
    pub flags: Option<PopulationDataFlags>,
}

#[derive(PartialEq, Debug)]
pub struct ExtLoc {
    pub id: ObjectId,
    pub location: SeqLoc,
}

#[derive(PartialEq, Debug)]
pub enum VariantRefMethod {
    Unknown,
    BacAcgh,
    Computational,
    Curated,
    DigitalArray,
    ExpressionArray,
    Fish,
    FlankingSequence,
    Maph,
    McdAnalysis,
    Mlpa,
    OeaAssembly,
    OligoAcgh,
    PairedEnd,
    PCR,
    Qpcr,
    ReadDepth,
    Roma,
    RtPcr,
    Sage,
    SequenceAlignment,
    Sequencing,
    SnpArray,
    Southern,
    Western,
    OpticalMapping,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub enum VariationRefDataSetType {
    Unknown,
    Compound,
    Products,
    Haplotype,
    Genotype,
    Mosaic,
    Individual,
    Population,
    Alleles,
    Package,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub struct VariationRefDataSet {
    pub r#type: VariationRefDataSetType,
    pub variations: BTreeSet<VariationRef>,
    pub name: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum VariationRefData {
    Unknown,

    /// free-form
    Note(String),

    UniparentalDisomy,

    /// actual sequence-edit at feat.location
    Instance(VariationInst),

    /// set of related variations
    ///
    /// location of the set equals to the union of member locations
    Set(Vec<VariationRefDataSet>),

    Variations(BTreeSet<VariationRef>),

    /// variant is a complex and un-described change at the location
    ///
    /// This type of variant is known to occur in dbVar submissions
    Complex,
}

#[derive(PartialEq, Debug)]
/// see http://www.hgvs.org/mutnomen/recs-prot.html
pub struct VariationFrameshift {
    pub phase: Option<i64>,
    pub x_length: Option<i64>,
}

#[derive(PartialEq, Debug)]
pub struct VariationLossOfHeterozygosity {
    /// in germline comparison, it will be reference genome assembly
    /// (default) or referenece/normal population. In somatic mutation,
    /// it will be a name of the normal tissue
    pub reference: Option<String>,

    /// name of the testing subject type or the testing issue
    pub test: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum VariationConsequence {
    Unknown,

    /// some effect on splicing
    Splicing,

    /// free-form
    Note(String),

    /// describe resulting variation in the product, eg: missense,
    /// nonsense, silent, neutral, etc in a protein that arises from
    /// THIS variation.
    Variation(VariationRef),

    /// see http://www.hgvs.org/mutnomen/recs-prot.html
    Frameshift(VariationFrameshift),
    LossOfHeterozygosity(VariationLossOfHeterozygosity),
}

#[derive(PartialEq, Debug)]
pub struct SomaticOriginCondition {
    pub description: Option<String>,

    /// reference to BioTerm / other descriptive database
    pub object_id: Option<BTreeSet<DbTag>>,
}

#[derive(PartialEq, Debug)]
pub struct VariationSomaticOrigin {
    /// description of the somatic origin itself
    pub source: Option<SubSource>,

    /// condition related to this origin's type
    pub condition: Option<SomaticOriginCondition>,
}

#[derive(PartialEq, Debug)]
/// reference to SNP
///
/// This object relates three IDs:
/// - our current object's ID
/// - the ID of this object's parent, if it exists
/// - the sample ID that this item originates from
pub struct VariationRef {
    /// ID's (ie: SN rsid / ssid, dbVar nsv/nssv)
    ///
    /// expected values include:
    /// - 'dbSNP|rs12334'
    /// - 'dbSNP|ss12345'
    /// - 'dbVar|nsv1'
    pub id: Option<DbTag>,
    pub parent_id: Option<DbTag>,
    pub sample_id: Option<ObjectId>,
    pub other_ids: Option<BTreeSet<DbTag>>,

    /// names and synonyms
    /// some variants have well-known canonical names and possible
    /// accepted synonyms
    pub name: Option<String>,
    pub synonyms: Option<BTreeSet<String>>,

    /// tag for comment and descriptions
    pub description: Option<String>,

    /// phenotype
    pub phenotype: Option<BTreeSet<Phenotype>>,

    /// sequencing / acquisition method
    pub method: Option<BTreeSet<VariantRefMethod>>,

    /// variant properties bitflags
    pub variant_prop: Option<VariantProperties>,

    pub data: VariationRefData,
    pub consequence: Option<BTreeSet<VariationConsequence>>,
    pub somatic_origin: Option<BTreeSet<VariationSomaticOrigin>>,
}

#[derive(PartialEq, Debug)]
pub enum DeltaSeq {
    Literal(SeqLiteral),
    Loc(SeqLoc),

    /// same location as [`VariationRef`] itself
    This,
}

#[derive(PartialEq, Debug, Default)]
pub enum DeltaAction {
    #[default]
    /// replace len(seq) positions starting with location.start with seq
    Morph,

    /// go downstream by distance specified by multiplier (upstream if < 0)
    /// in genomic context
    Offset,

    /// excise sequence at location
    ///
    /// if multiplier is specified, delete len(location)*multiplier
    /// positions downstream
    DelAt,

    ///  insert seq before the location.start
    InsBefore,
}

#[derive(PartialEq, Debug)]
pub struct DeltaItem {
    pub seq: Option<DeltaSeq>,

    /// multiplier allows representing a tandem, eg: ATATAT as AT*3
    ///
    /// This allows describing CNV/SSR where delta=self  with a
    /// multiplier which specifies the count of the repeat unit.
    pub multiplier: Option<i64>,        // assumed 1 if not specified
    pub multiplier_fuzz: Option<IntFuzz>,

    pub action: DeltaAction,
}

#[derive(PartialEq, Debug)]
pub enum VariationInstType {
    /// `delta=None`
    Unknown,

    /// `delta=None`
    Identity,

    /// `delta=[del, ins.seq=RevComp(variation-location)]`
    Inv,

    /// `delta=[morph of length 1]`
    ///
    /// NOTE: this is snV not snP; the latter requires frequency-based
    /// validation to be established in [`VariantProperties`]. The strict
    /// definition of SNP is an SNV with an established population frequency
    /// of at least 1% in at least 1 population.
    SNV,

    /// `delta=[morph of length >1]`
    MNP,

    /// `delta=[del, ins]`
    DelIns,

    /// `delta=[del]`
    Del,

    /// `delta=[ins]`
    Ins,

    /// `delta=[del, ins.seq=repeat-unit with fuzzy multiplier]`
    ///
    /// `variation_location` is the microsat expansion on the sequence
    Microsatellite,

    /// `delta=[del, ins.seq= known donor or 'this']`
    ///
    /// `variation_location` is equiv of transposon locations
    Transposon,

    /// `delta=[del, ins= 'this' with fuzzy multiplier]`
    CNV,

    /// `delta=[ins.seq= upstream location on the same strand]`
    DirectCopy,

    /// `delta=[ins.seq= downstream location on the same strand]`
    RevDirectCopy,

    /// `delta=[ins.seq= upstream location on the same opposite strand]`
    InvertedCopy,

    /// `delta=[ins.seq= downstream location on the same opposite strand]`
    EvertedCopy,

    /// delta = like [`Self::DelIns`]
    Translocation,

    /// `delta=[morph of length 1]`
    ProtMissense,

    /// `delta=[del]`
    ///
    /// `variation_location` is the tail of the protein being truncated
    ProtNonsense,

    /// `delta=[morph of length 1]`
    ProtNeutral,

    /// `delta=[morph of length 1, same AA as at variation-location]`
    ProtSilent,

    /// delta=any
    ProtOther,

    /// delta=any
    Other = 255,
}

#[derive(PartialEq, Debug)]
/// Used to label items in a [`VariationRef`] package
pub enum VariationInstObservation {
    /// represents the asserted base at a position
    Asserted = 1,

    /// represents the reference base at the position
    Reference = 2,

    /// represent the observed variant at a given position
    Variant = 4,
}

#[derive(PartialEq, Debug)]
pub struct VariationInst {
    pub r#type: VariationInstType,

    /// sequence that replaces the location, in biological order
    pub delta: Vec<DeltaItem>,

    /// used to label items in a [`VariationRef`] package
    ///
    /// This field is explicitly a bitflag
    pub observation: Option<VariationInstObservation>,
}

#[derive(PartialEq, Debug)]
pub enum RSiteRef {
    /// may be unparsable
    Str(String),

    /// pointer to a restriction site database
    DB(DbTag),
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
/// Represents RNA feature types
pub enum RnaRefType {
    Unknown,
    PreMsg,
    mRNA,
    tRNA,
    rRNA,

    /// ### Original Comment:
    ///     will become ncRNA, with RNAGen.class = snRNA
    snRNA,

    /// ### Original Comment:
    ///     will become ncRNA, with RNAGen.class = snRNA
    scRNA,

    /// ### Original Comment:
    ///     will become ncRNA, with RNAGen.class = snRNA
    snoRNA,

    /// non-coding RNA; subsumes `snRNA`, `scRNA` and `snoRNA`
    ncRNA,
    tmRNA,
    MiscRNA,
    Other = 255,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum RnaRefExt {
    /// for naming "other" type
    Name(String),

    /// for tRNA's
    tRNA(TRnaExt),

    /// generic fields for `ncRNA`, `tmRNA` and `miscRNA`
    Gen(RnaGen),
}

#[derive(PartialEq, Debug)]
pub struct RnaRef {
    pub r#type: RnaRefType,
    pub pseudo: Option<bool>,
    pub ext: Option<RnaRefExt>,
}

#[derive(PartialEq, Debug)]
pub enum TRnaExtAa {
    IUPACAa(u64),
    NCBIEaa(u64),
    NCBI8aa(u64),
    NCBIStdAa(u64),
}

#[derive(PartialEq, Debug)]
/// tRNA feature extensions
pub struct TRnaExt {
    /// transported amino acid
    pub aa: TRnaExtAa,

    /// codon(s) as in [`GeneticCode`]
    pub codon: Option<BTreeSet<u64>>,

    /// location of anticodon
    pub anticodon: Option<SeqLoc>,
}

#[derive(PartialEq, Debug)]
pub struct RnaGen {
    /// for ncRNA's, the class of non-coding RNA
    ///
    /// examples: antisense RNA, guide RNA, snRNA
    pub class: Option<String>,

    pub product: Option<String>,

    /// eg: `tag_peptide` qualifier for tmRNA's
    pub quals: Option<RnaQualSet>,
}

#[derive(PartialEq, Debug)]
pub struct RnaQual {
    pub qual: String,
    pub val: String,
}

pub type RnaQualSet = Vec<RnaQual>;

#[derive(PartialEq, Debug)]
pub struct GeneRef {
    /// official gene symbol
    pub locus: Option<String>,

    /// official allele designation
    pub allele: Option<String>,

    /// descriptive name
    pub desc: Option<String>,

    /// descriptive map location
    pub maploc: Option<String>,

    /// pseudogene
    pub pseudo: bool, // TODO: default false

    /// ids in other dbases
    pub db: Option<BTreeSet<DbTag>>,

    /// synonyms for locus
    pub syn: Option<BTreeSet<String>>,

    /// systematic gene name (eg: MI0001, ORF0069)
    pub locus_tag: Option<String>,

    pub formal_name: Option<GeneNomenclature>,
}

#[derive(PartialEq, Debug)]
pub enum GeneNomenclatureStatus {
    Unknown,
    Official,
    Interim,
}

#[derive(PartialEq, Debug)]
pub struct GeneNomenclature {
    pub status: GeneNomenclatureStatus,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub source: Option<DbTag>,
}

#[derive(PartialEq, Debug)]
/// Reference to an organism
///
/// Defines only the organism. Lower levels of detail for biological molecules
/// are provided by [`BioSource`]
pub struct OrgRef {
    /// preferred formal name
    pub taxname: Option<String>,

    /// common name
    pub common: Option<String>,

    /// unstructured modifiers
    pub r#mod: Option<BTreeSet<String>>,

    /// ids in taxonomic or culture databases
    pub db: Option<BTreeSet<DbTag>>,

    /// synonyms for `taxname` or common
    pub syn: Option<BTreeSet<String>>,

    pub orgname: Option<OrgName>,
}

#[derive(PartialEq, Debug)]
pub enum OrgNameChoice {
    /// genus/species type name
    Binomial(BinomialOrgName),

    /// virus names are different
    Virus(String),

    /// hybrid between organisms
    Hybrid(MultiOrgName),

    /// some hybrids have genus x species name
    NamedHybrid(BinomialOrgName),

    /// when genus not known
    Partial(PartialOrgName),
}

#[derive(PartialEq, Debug)]
pub struct OrgName {
    pub name: Option<OrgNameChoice>,

    /// attribution of name
    pub attrib: Option<String>,

    pub r#mod: Option<Vec<OrgMod>>,

    /// lineage with semicolon separators
    pub lineage: Option<String>,

    /// genetic code
    ///
    /// See Also:
    /// - [`CdRegion`]
    pub gcode: Option<u64>,

    /// mitochondrial genetic code
    pub mgcode: Option<u64>,

    /// GenBank division code
    pub div: Option<String>,

    /// plastid genetic code
    pub pgcode: Option<u64>,
}

#[derive(PartialEq, Debug)]
pub enum OrgModSubType {
    Strain = 2,
    SubStrain,
    Type,
    SubType,
    Variety,
    Serotype,
    Serogroup,
    Serovar,
    Cultivar,
    Pathovar,
    Chemovar,
    Biovar,
    Biotype,
    Group,
    SubGroup,
    Isolate,
    Common,
    Acronym,

    /// chromosome dosage of hybrid
    Dosage,

    /// natural host of this specimen
    NatHost,

    SubSpecies,
    SpecimenVoucher,
    Authority,
    Forma,
    FormaSpecialis,
    Ecotype,
    Synonym,
    Anamorph,
    Breed,

    /// used by taxonomy database
    GbAcronym,

    /// used by taxonomy database
    GbAnamorph,

    /// used by taxonomy database
    GbSynonym,

    CultureCollection,
    BioMaterial,
    MetagenomeSource,
    TypeMaterial,

    /// code of nomenclature in subname (B,P,V,Z or combination)
    Nomenclature,

    OldLineage = 253,
    OldName = 254,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub struct OrgMod {
    pub subtype: OrgModSubType,
    pub subname: String,

    /// attribution/source of name
    pub attrib: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct BinomialOrgName {
    /// required
    pub genus: String,

    /// species required if subspecies used
    pub species: Option<String>,
    pub subspecies: Option<String>,
}

/// the first value will be used to assign division
pub type MultiOrgName = Vec<OrgName>;

/// used when genus not known
pub type PartialOrgName = Vec<TaxElement>;

#[derive(PartialEq, Debug)]
pub enum TaxElementFixedLevel {
    /// level must be set in string
    Other,

    Family,
    Order,
    Class,
}

#[derive(PartialEq, Debug)]
pub struct TaxElement {
    pub fixed_level: TaxElementFixedLevel,
    pub level: Option<String>,
    pub name: String,
}

#[derive(PartialEq, Debug, Default)]
/// biological context from which a molecule came from
pub enum BioSourceGenome {
    #[default]
    Unknown,
    Genomic,
    Chloroplast,
    Chromoplast,
    Kinetoplast,
    Mitochondrion,
    Plastid,
    Macronuclear,
    Extrachrom,
    Plasmid,
    Transposon,
    INsertionSeq,
    Cyanelle,
    Proviral,
    Virion,
    Nucleomorph,
    Apicoplast,
    Leucoplast,
    Proplastid,
    EndogenousVirus,
    Hydrogenosome,
    Chromosome,
    PlasmidInMitochondrion,
    PlasmidInPlastid,
}

#[derive(PartialEq, Debug, Default)]
pub enum BioSourceOrigin {
    #[default]
    Unknown,

    /// normal biological entity
    Natural,

    /// naturally occurring mutant
    NatMut,

    /// artificially mutagenized
    Mut,

    /// artificially engineered
    Artificial,

    /// purely synthetic
    Synthetic,

    Other = 255,
}

#[derive(PartialEq, Debug)]
pub struct BioSource {
    /// biological context
    pub genome: BioSourceGenome,

    pub origin: BioSourceOrigin,
    pub org: OrgRef,
    pub subtype: Option<Vec<SubSource>>,

    /// to distinguish biological focus
    ///
    /// # Implementation Notes
    ///
    /// `Option<()>` has been chosen to conform to ASN.1 spec
    pub is_focus: Option<()>,
    pub pcr_primers: Option<PCRReationSet>,
}

pub type PCRReationSet = BTreeSet<PCRReaction>;
#[derive(PartialEq, Debug)]
pub struct PCRReaction {
    pub forward: Option<PCRPrimerSet>,
    pub reverse: Option<PCRPrimerSet>,
}

pub type PCRPrimerSet = BTreeSet<PCRPrimer>;
#[derive(PartialEq, Debug)]
pub struct PCRPrimer {
    pub seq: Option<PCRPrimerSeq>,
    pub name: Option<PCRPrimerName>,
}
pub type PCRPrimerSeq = String;
pub type PCRPrimerName = String;

#[derive(PartialEq, Debug)]
pub enum SubSourceSubType {
    Chromosome = 1,
    Map,
    Clone,
    Subclone,
    Haplotype,
    Genotype,
    Sex,
    CellLine,
    CellType,
    TissueType,
    CloneLib,
    DevStage,
    Frequency,
    Germline,
    Rearranged,
    LabHost,
    PopVariant,
    TissueLib,
    PlasmidName,
    TransposonName,
    InsertionSeqName,
    PlastidName,
    Country,
    Segment,
    EndogenousVirusName,
    Transgenic,
    EnvironmentalSample,
    IsolationSource,

    /// +/- decimal degrees
    LatLon,

    /// DD-MMM-YYYY format
    CollectionDate,

    /// name of person who collected sample
    CollectedBy,

    /// name of person who identified sample
    IdentifiedBy,

    /// sequence (possibly more than one; semicolon-separated)
    FwdPrimerSeq,

    /// sequence (possibly more than one; semicolon-separated)
    RevPrimerSeq,

    FwdPrimerName,
    RevPrimerName,
    Metagenomic,
    MatingType,
    LinkageGroup,
    Haplogroup,
    WholeReplicon,
    Phenotype,
    Altitude,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub struct SubSource {
    pub subtype: SubSourceSubType,
    pub name: String,

    /// attribution/source of this name
    pub attrib: Option<String>,
}

#[derive(PartialEq, Debug, Default)]
pub enum ProtRefProcessingStatus {
    #[default]
    NotSet,
    PreProtein,
    Mature,
    SignalPeptide,
    TransitPeptide,
    ProPeptide,
}

#[derive(PartialEq, Debug)]
/// Reference to a protein name
pub struct ProtRef {
    /// protein name
    pub name: Option<BTreeSet<String>>,

    /// description (instead of name)
    pub desc: Option<String>,

    /// E.C. number(s)
    pub ec: Option<BTreeSet<String>>,

    /// activities
    pub activity: Option<BTreeSet<String>>,

    /// id's in other dbases
    pub db: Option<BTreeSet<DbTag>>,

    /// processing status
    pub processed: ProtRefProcessingStatus,
}

#[derive(PartialEq, Debug)]
pub enum TxSystem {
    Unknown,

    /// eukaryotic Pol I
    Pol1,

    /// eukaryotic Pol II
    Pol2,

    /// eukaryotic Pol III
    Pol3,
    Bacterial,
    Viral,

    /// RNA replicase
    Rna,

    Organelle,
    Other = 255,
}

#[derive(PartialEq, Debug)]
/// Represents type of transcription initiation site (TIS)
pub enum InitType {
    Unknown,

    /// transcript initiated from a single sites
    Single,

    /// transcript initiated from multiple sites
    Multiple,

    /// transcript is initiated from a specific region
    Region,
}

#[derive(PartialEq, Debug)]
/// Transcription Initiation Site feature data block
pub struct TxInit {
    /// descriptive name of initiation site
    pub name: String,

    /// synonyms
    pub syn: Option<Vec<String>>,

    /// gene(s) transcribed
    pub gene: Option<Vec<GeneRef>>,

    /// protein(s) produced
    pub protein: Option<Vec<ProtRef>>,

    /// rna(s) produced
    pub rna: Option<Vec<String>>,

    /// tissue/time of expression
    pub expression: Option<String>,

    /// transcription apparatus used at this site
    pub txsystem: TxSystem,

    /// modifiers to [`TxSystem`]
    pub txdescr: Option<String>,

    /// organism supplying transcription apparatus
    pub txorg: Option<OrgRef>,

    /// mapping precise or approx
    pub mapping_precise: bool,   // TODO: default false

    /// does [`SeqLoc`] reflect mapping
    pub location_accurate: bool, // TODO: default false

    pub inittype: InitType,
    pub evidence: Option<BTreeSet<TxEvidence>>,
}

#[derive(PartialEq, Debug)]
pub enum TxEvidenceExpCode {
    Unknown,

    /// direct RNA sequencing
    RnaSeq,

    /// RNA length measurement
    RnaSize,

    /// nuclease protection mapping with homologous sequence ladder
    NpMap,

    /// nuclease protected fragment length measurement
    NpSize,

    /// di-deoxy RNA sequencing
    PeSeq,

    /// full-length cDNA sequencing
    CDnaSeq,

    /// primer extension mapping with homologous sequence ladder
    PeMap,

    /// primer extension product length measurement
    PeSize,

    /// full-length processed pseudogene sequencing
    PseudoSeq,

    /// length measurement of a reverse direction primer-extension product
    /// (blocked by RNA 5' end) by comparison with homologous sequence
    /// ladder (J. Mol. Biol. 199, 587)
    RevPeMap,

    Other = 255,
}

#[derive(PartialEq, Debug, Default)]
pub enum TxEvidenceExpressionSystem {
    Unknown,
    #[default]
    Physiological,
    InVitro,
    Oocyte,
    Transfection,
    Transgenic,
    Other = 255,
}

#[derive(PartialEq, Debug)]
pub struct TxEvidence {
    pub exp_code: TxEvidenceExpCode,
    pub expression_system: TxEvidenceExpressionSystem,
    pub low_prec_data: bool, // TODO: default false
    pub from_homolog: bool,  // TODO: default false
}












