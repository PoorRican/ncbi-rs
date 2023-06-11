//! Sequence feature elements
//!
//! Adapted from ["seqfeat.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqfeat/seqfeat.asn)
//! and documented by [NCBI C++ Toolkit Book](https://ncbi.github.io/cxx-toolkit/pages/ch_datamod#ch_datamod.datamodel.seqfeat)

use std::collections::HashSet;
use crate::asn::{DOI, PubMedId, Heterogen, PubDesc, SeqLiteral, DbTag,
                 IntFuzz, ObjectId, UserObject, Pub, PubSet,
                 GiimportId, SeqId, SeqLoc};

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

/// Evidence for existence of feature
pub enum SeqFeatExpEvidence {
    Experimental,
    NotExperimental,
}

/// Sequence feature generalization
pub struct SeqFeat {
    pub id: Option<FeatId>,
    pub data: SeqFeatData,
    pub partial: Option<bool>,
    pub except: Option<bool>,
    pub comment: Option<String>,
    pub product: Option<SeqLoc>,
    pub location: SeqLoc,
    pub qual: Option<Vec<GbQual>>,
    pub title: Option<String>,
    pub ext: Option<UserObject>,
    pub cit: Option<PubSet>,
    pub exp_ev: Option<SeqFeatExpEvidence>,
    pub xref: Option<HashSet<SeqFeatXref>>,
    pub dbxref: Option<HashSet<DbTag>>,
    pub pseudo: Option<bool>,
    pub except_text: Option<String>,
    pub ids: Option<HashSet<FeatId>>,
    pub exts: Option<HashSet<UserObject>>,
    pub support: Option<SeqFeatSupport>,
}

pub enum SeqFeatBond {
    Disulfide,
    Thiolester,
    XLink,
    Thioether,
    Other = 255,
}

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

/// Protein secondary structure
pub enum PSecStr {
    /// any helix
    Helix,
    /// beta sheet
    Sheet,
    /// beta or gamma turn
    Turn,
}

pub enum SeqFeatData {
    Gene(GeneRef),
    Org(OrgRef),
    CdRegion(CdRegion),
    Prot(ProtRef),
    RNA(RnaRef),
    Pub(PubDesc),
    Seq(SeqLoc),
    Imp(ImpFeat),
    Region(String),
    Bond(SeqFeatBond),
    PSecStr(PSecStr),
    NonStdResidue(String),
    Het(Heterogen),
    BioSrc(BioSource),
    Clone(CloneRef),
    Variation(VariationRef),
}

pub struct SeqFeatXref {
    pub id: Option<FeatId>,
    pub data: Option<SeqFeatData>,
}

pub struct SeqFeatSupport {
    pub experiment: Option<HashSet<ExperimentSupport>>,
    pub inference: Option<HashSet<InferenceSupport>>,
    pub model_evidence: Option<HashSet<ModelEvidenceSupport>>,
}

pub enum EvidenceCategory {
    NotSet,
    Coordinates,
    Description,
    Existence,
}

pub struct ExperimentSupport {
    pub category: Option<EvidenceCategory>,
    pub explanation: String,
    pub pmids: Option<HashSet<PubMedId>>,
    pub dois: Option<HashSet<DOI>>,
}

pub struct ProgramId {
    pub name: String,
    pub version: Option<String>,
}

pub struct EvidenceBasis {
    pub programs: Option<HashSet<ProgramId>>,
    pub accessions: Option<HashSet<SeqId>>,
}

#[derive(Default)]
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

pub struct InferenceSupport {
    pub category: Option<EvidenceCategory>,
    pub r#type: InferenceSupportType,
    pub other_type: Option<String>,
    // TODO: default to false
    pub same_species: bool,
    pub basis: EvidenceBasis,
    pub pmids: Option<HashSet<PubMedId>>,
    pub soids: Option<HashSet<DOI>>,
}

pub struct ModelEvidenceItem {
    pub id: SeqId,
    pub exon_count: Option<u64>,
    pub exon_length: Option<u64>,
    // TODO: default to false
    pub full_length: bool,
    // TODO: default to false
    pub supports_all_exon_combo: bool,
}

pub struct ModelEvidenceSupport {
    pub method: Option<String>,
    pub mrna: Option<HashSet<ModelEvidenceItem>>,
    pub est: Option<HashSet<ModelEvidenceItem>>,
    pub protein: Option<HashSet<ModelEvidenceItem>>,
    pub identification: Option<SeqId>,
    pub dbxref: Option<HashSet<DbTag>>,
    pub exon_count: Option<u64>,
    pub exon_length: Option<u64>,
    pub full_length: bool,                  // TODO: default false
    pub supports_all_exon_combo: bool,      // TODO: default false

}

#[derive(Default)]
pub enum CdRegionFrame {
    #[default]
    NotSet,
    One,
    Two,
    Three,
}

pub struct CdRegion {
    pub orf: Option<bool>,
    pub frame: CdRegionFrame,
    pub conflict: Option<bool>,
    pub gaps: Option<u64>,
    pub mismatch: Option<u64>,
    pub code: Option<GeneticCode>,
    pub code_break: Option<Vec<CodeBreak>>,
    pub stops: Option<u64>,
}

pub enum GeneticCodeOpt {
    Name(String),
    Id(u64),
    NcbiEaa(String),
    Ncbi8aa(Vec<u8>),
    NcbiStdAa(Vec<u8>),
    SNcbiEaa(String),
    SNcbi8aa(Vec<u8>),
    SNcbiStdAa(Vec<u8>),
}

pub type GeneticCode = HashSet<GeneticCodeOpt>;

pub enum CodeBreakAA {
    NcbiAa(u64),
    Ncbi8aa(u64),
    NcbiStdAa(u64),
}

pub struct CodeBreak {
    pub loc: SeqLoc,
    pub aa: CodeBreakAA,
}

pub type GeneticCodeTable = HashSet<GeneticCode>;

pub struct ImpFeat {
    pub bkey: String,
    pub bloc: Option<String>,
    pub bdescr: Option<String>,
}

pub struct GbQual {
    pub qual: String,
    pub val: String,
}

pub enum CloneRefPlacementMethod {
    EndSeq,
    InsertAlignment,
    STS,
    Fish,
    Fingerprint,
    EndSeqInsertAlignment,
    External = 253,
    Curated = 254,
    Other = 255,
}

pub struct CloneRef {
    pub name: String,
    pub library: Option<String>,
    // TODO: default to false
    pub concordant: bool,
    // TODO: default to false
    pub unique: bool,
    pub placement_method: Option<CloneRefPlacementMethod>,
    pub clone_seq: Option<CloneSeqSet>,
}

pub type CloneSeqSet = HashSet<CloneSeq>;

pub enum CloneSeqType {
    Insert,
    End,
    Other = 255,
}

pub enum CloneSeqConfidence {
    Multiple,
    NA,
    NoHitRep,
    NoHitNoRep,
    OtherChrm,
    Unique,
    Virtual,
    MultipleRep,
    MultipleNoRep,
    NoHit,
    Other = 255,
}

pub enum CloneSeqSupport {
    Prototype,
    Supporting,
    SupportsOther,
    NonSupporting,
}

pub struct CloneSeq {
    pub r#type: CloneSeqType,
    pub confidence: Option<CloneSeqConfidence>,
    pub location: SeqLoc,
    pub seq: Option<SeqLoc>,
    pub align_id: Option<DbTag>,
    pub support: Option<CloneSeqSupport>,
}

pub enum VariantResourceLink {
    Preserved = 1,
    Provisional = 2,
    Has3D = 4,
    SubmitterLinkout = 8,
    Clinical = 16,
    GenotypeKit = 32,
}

pub enum VariantGeneLocation {
    InGene = 1,
    NearGene5 = 2,
    NearGene3 = 4,
    Intron = 8,
    Donor = 16,
    Acceptor = 32,
    UTR5 = 64,
    UTR3 = 128,
    InStartCodon = 256,
    InStopCodon = 512,
    Intergenic = 1024,
    ConservedNonCoding = 2048,
}

pub enum VariantEffect {
    NoChange = 0,
    Synonymous = 1,
    Nonsense = 2,
    Missense = 4,
    Frameshift = 8,
    UpRegulator = 16,
    DownRegulator = 32,
    Methylation = 64,
    StopGain = 128,
    StopLose = 256,
}

pub enum VariantMapping {
    HasOtherSNP = 1,
    HasAssemblyConflict = 2,
    IsAssemblySpecific = 4,
}

pub enum VariantMapWeight {
    IsUniquelyPlaced = 1,
    PlacedTwiceOnSameChrom = 2,
    PlacedTypeOnDiffChrom = 3,
    ManyPlacements = 10,
}

pub enum FrequencyBasedValidation {
    IsMutation,
    Above5Pct1Plus,
    Validated,
    Above1PctAll,
    Above1Pct1Plus
}

pub enum VariantGenotype {
    InHaplotypeSet,
    HasGenotypes,
}

pub enum VariantQualityCheck {
    ContigAlleleMissing,
    WithdrawnBySubmitter,
    NonOverlappingAlleles,
    StrainSpecific,
    GenotypeConflict,
}

pub enum VariantConfidence {
    Unknown,
    LikelyArtifact,
    Other = 255,
}

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

pub enum VariantAlleleState {
    Unknown,
    Homosygous,
    Heterozygous,
    Hemizygous,
    Nullizygous,
    Other = 255,
}

pub struct VariantProperties {
    pub version: u64,
    pub resource_link: Option<VariantResourceLink>,
    pub gene_location: Option<VariantGeneLocation>,
    pub effect: Option<VariantEffect>,
    pub mapping: Option<VariantMapping>,
    pub map_weight: Option<VariantMapWeight>,
    pub frequency_based_validation: Option<FrequencyBasedValidation>,
    pub genotype: Option<VariantGenotype>,
    pub project_data: Option<HashSet<u64>>,
    pub quality_check: Option<VariantQualityCheck>,
    pub confidence: VariantConfidence,
    pub other_validation: Option<bool>,
    pub allele_origin: Option<VariantAlleleOrigin>,
    pub allele_state: Option<VariantAlleleState>,
    pub allele_frequency: Option<f64>,
    pub is_ancestral_allele: Option<bool>,
}

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

pub struct Phenotype {
    pub source: Option<String>,
    pub term: Option<String>,
    pub xref: Option<HashSet<DbTag>>,
    pub clinical_significance: Option<PhenotypeClinicalSignificance>,
}

pub enum PopulationDataFlags {
    IsDefaultPopulation = 1,
    IsMinorAllele = 2,
    IsRareAllele = 4,
}

pub struct PopulationData {
    pub population: String,
    pub genotype_frequency: Option<f64>,
    pub chromosomes_tested: Option<u64>,
    pub sample_ids: Option<HashSet<ObjectId>>,
    pub allele_frequency: Option<f64>,
    pub flags: Option<PopulationDataFlags>,
}

pub struct ExtLoc {
    pub id: ObjectId,
    pub location: SeqLoc,
}

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

pub struct VariationRefDataSet {
    pub r#type: VariationRefDataSetType,
    pub variations: HashSet<VariationRef>,
    pub name: Option<String>,
}

pub enum VariationRefData {
    Unknown,
    Note(String),
    UniparentalDisomy,
    Instance(VariationInst),
    Set(Vec<VariationRefDataSet>),
    Complex,
}

pub struct VariationFrameshift {
    pub phase: Option<i64>,
    pub x_length: Option<i64>,
}

pub struct VariationLossOfHeterozygosity {
    pub reference: Option<String>,
    pub test: Option<String>,
}

pub enum VariationConsequence {
    Unknown,
    Splicing,
    Note(String),
    Variation(VariationRef),
    Frameshift(VariationFrameshift),
    LossOfHeterozygosity(VariationLossOfHeterozygosity),
}

pub struct SomaticOriginCondition {
    pub description: Option<String>,
    pub object_id: Option<HashSet<DbTag>>,
}

pub struct VariationSomaticOrigin {
    pub source: Option<SubSource>,
    pub condition: Option<SomaticOriginCondition>,
}

pub struct VariationRef {
    pub id: Option<DbTag>,
    pub parent_id: Option<DbTag>,
    pub sample_id: Option<ObjectId>,
    pub other_ids: Option<HashSet<DbTag>>,
    pub name: Option<String>,
    pub synonyms: Option<HashSet<String>>,
    pub description: Option<String>,
    pub phenotype: Option<HashSet<Phenotype>>,
    pub method: Option<HashSet<VariantRefMethod>>,
    pub variant_prop: Option<VariantProperties>,
    pub r#pub: Option<Pub>,
    pub data: VariationRefData,
    pub consequence: Option<HashSet<VariationConsequence>>,
    pub somatic_origin: Option<HashSet<VariationSomaticOrigin>>,
}

pub enum DeltaSeq {
    Literal(SeqLiteral),
    Loc(SeqLoc),
    This,
}

#[derive(Default)]
pub enum DeltaAction {
    #[default]
    Morph,
    Offset,
    DelAt,
    InsBefore
}

pub struct DeltaItem {
    pub seq: Option<DeltaSeq>,
    pub multiplier: Option<i64>,
    pub multiplier_fuzz: Option<IntFuzz>,
    pub action: DeltaAction,
}

pub enum VariationInstType {
    Unknown,
    Identity,
    Inv,
    SNV,
    MNP,
    DelIns,
    Del,
    Ins,
    Microsatellite,
    Transposon,
    CNV,
    DirectCopy,
    RevDirectCopy,
    EvertedCopy,
    Translocation,
    ProtMissense,
    ProtNonsense,
    ProtNeutral,
    ProtSilent,
    Other = 255,
}

pub enum VariationInstObservation {
    Asserted,
    Reference,
    Variant,
}

pub struct VariationInst {
    pub r#type: VariationInstType,
    pub delta: Vec<DeltaItem>,
    pub observation: Option<VariationInstObservation>,
}

pub enum RSiteRef {
    Str(String),
    DB(DbTag),
}

#[allow(non_camel_case_types)]
pub enum RnaRefType {
    Unknown,
    PreMsg,
    mRNA,
    tRNA,
    rRNA,
    snRNA,
    scRNA,
    snoRNA,
    ncRNA,
    tmRNA,
    MiscRNA,
    Other = 255,
}

#[allow(non_camel_case_types)]
pub enum RnaRefExt {
    Name(String),
    tRNA(TRnaExt),
    Gen(RnaGen,)
}

pub struct RnaRef {
    pub r#type: RnaRefType,
    pub pseudo: Option<bool>,
    pub ext: Option<RnaRefExt>,
}

pub enum TRnaExtAa {
    IUPACAa(u64),
    NCBIEaa(u64),
    NCBI8aa(u64),
    NCBIStdAa(u64),
}

pub struct TRnaExt {
    pub aa: TRnaExtAa,
    pub codon: Option<HashSet<u64>>,
    pub anticodon: Option<SeqLoc>,
}

pub struct RnaGen {
    pub class: Option<String>,
    pub product: Option<String>,
    pub quals: Option<RnaQualSet>,
}

pub struct RnaQual {
    pub qual: String,
    pub val: String,
}

pub type RnaQualSet = Vec<RnaQual>;

pub struct GeneRef {
    pub locus: Option<String>,
    pub allele: Option<String>,
    pub desc: Option<String>,
    pub maploc: Option<String>,
    pub pseudo: bool,                   // TODO: default false
    pub db: Option<HashSet<DbTag>>,
    pub syn: Option<HashSet<String>>,
    pub locus_tag: Option<String>,
    pub formal_name: Option<GeneNomenclature>,
}

pub enum GeneNomenclatureStatus {
    Unknown,
    Official,
    Interim,
}

pub struct GeneNomenclature {
    pub status: GeneNomenclatureStatus,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub source: Option<DbTag>,
}

pub struct OrgRef {
    pub taxname: Option<String>,
    pub common: Option<String>,
    pub r#mod: Option<HashSet<String>>,
    pub db: Option<HashSet<DbTag>>,
    pub syn: Option<HashSet<String>>,
    pub orgname: Option<OrgName>,
}

pub enum OrgNameChoice {
    Binomial(BinomialOrgName),
    Virus(String),
    Hybrid(MultiOrgName),
    NamedHybrid(BinomialOrgName),
    Partial(PartialOrgName),
}

pub struct OrgName {
    pub name: Option<OrgNameChoice>,
    pub attrib: Option<String>,
    pub r#mod: Option<Vec<OrgMod>>,
    pub lineage: Option<String>,
    pub gcode: Option<u64>,
    pub mgcode: Option<u64>,
    pub div: Option<String>,
    pub pgcode: Option<u64>,
}

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
    Dosage,
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
    GbAcronym,
    GbAnamorph,
    GbSynonym,
    CultureCollection,
    BioMaterial,
    MetagenomeSource,
    TypeMaterial,
    Nomenclature,
    OldLineage = 253,
    OldName = 254,
    Other = 255,
}

pub struct OrgMod {
    pub subtype: OrgModSubType,
    pub subname: String,
    pub attrib: Option<String>,
}

pub struct BinomialOrgName {
    pub genus: String,
    pub species: Option<String>,
    pub subspecies: Option<String>,
}

pub type MultiOrgName = Vec<OrgName>;

pub type PartialOrgName = Vec<TaxElement>;

pub enum TaxElementFixedLevel {
    Other,
    Family,
    Order,
    Class,
}

pub struct TaxElement {
    pub fixed_level: TaxElementFixedLevel,
    pub level: Option<String>,
    pub name: String,
}

#[derive(Default)]
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
    PlasmidInPlastid
}

#[derive(Default)]
pub enum BioSourceOrigin {
    #[default]
    Unknown,
    Natural,
    NatMut,
    Mut,
    Artificial,
    Synthetic,
    Other = 255,
}

pub struct BioSource {
    pub genome: BioSourceGenome,
    pub origin: BioSourceOrigin,
    pub org: OrgRef,
    pub subtype: Option<Vec<SubSource>>,
    pub is_focus: Option<()>,
    pub pcr_primers: Option<PCRReationSet>,
}

pub type PCRReationSet = HashSet<PCRReaction>;
pub struct PCRReaction {
    pub forward: Option<PCRPrimerSet>,
    pub reverse: Option<PCRPrimerSet>,
}

pub type PCRPrimerSet = HashSet<PCRPrimer>;
pub struct PCRPrimer {
    pub seq: Option<PCRPrimerSeq>,
    pub name: Option<PCRPrimerName>,
}
pub type PCRPrimerSeq = String;
pub type PCRPrimerName = String;

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
    LatLon,
    CollectionDate,
    CollectedBy,
    IdentifiedBy,
    FwdPrimerSeq,
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

pub struct SubSource {
    pub subtype: SubSourceSubType,
    pub name: String,
    pub attrib: Option<String>,
}

#[derive(Default)]
pub enum ProtRefProcessingStatus {
    #[default]
    NotSet,
    PreProtein,
    Mature,
    SignalPeptide,
    TransitPeptide,
    ProPeptide,
}

pub struct ProtRef {
    pub name: Option<HashSet<String>>,
    pub desc: Option<String>,
    pub ec: Option<HashSet<String>>,
    pub activity: Option<HashSet<String>>,
    pub db: Option<HashSet<DbTag>>,
    pub processed: ProtRefProcessingStatus,
}

pub enum Txsystem {
    Unknown,
    Pol1,
    Pol2,
    Pol3,
    Bacterial,
    Viral,
    Rna,
    Organelle,
    Other = 255,
}

pub enum InitType {
    Unknown,
    Single,
    Multiple,
    Region
}

pub struct Txinit {
    pub name: String,
    pub syn: Option<Vec<String>>,
    pub gene: Option<Vec<GeneRef>>,
    pub protein: Option<Vec<ProtRef>>,
    pub rna: Option<Vec<String>>,
    pub expression: Option<String>,
    pub txdescr: Option<String>,
    pub txorg: Option<OrgRef>,
    pub mapping_precise: bool,          // TODO: default false
    pub location_accurate: bool,        // TODO: default false
    pub inittype: InitType,
    pub evidence: Option<HashSet<TxEvidence>>,
}

pub enum TxEvidenceExpCode {
    Unknown,
    RnaSeq,
    RnaSize,
    NpMap,
    NpSize,
    PeSeq,
    CDnaSeq,
    PeMap,
    PeSize,
    PseudoSeq,
    RevPeMap,
    Other = 255,
}

#[derive(Default)]
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

pub struct TxEvidence {
    pub exp_code: TxEvidenceExpCode,
    pub expression_system: TxEvidenceExpressionSystem,
    pub low_prec_data: bool,            // TODO: default false
    pub from_homolog: bool,             // TODO: default false
}
























