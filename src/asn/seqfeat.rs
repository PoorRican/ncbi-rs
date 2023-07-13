//! Sequence feature elements
//!
//! Adapted from ["seqfeat.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/seqfeat/seqfeat.asn)
//! and documented by [NCBI C++ Toolkit Book](https://ncbi.github.io/cxx-toolkit/pages/ch_datamod#ch_datamod.datamodel.seqfeat)
//!
//! A feature table is a collection of sequence features called [`SeqFeat`]s.
//! A [`SeqFeat`] serves to connect a sequence location ([`SeqLoc`]) with a
//! specific block of data known as a datablock. Datablocks are defined
//! objects on their own and are often used in other contexts, such as
//! publications ([`PubSet`]), references to organisms ([`OrgRef`]), or genes
//! ([`GeneRef`]). Some datablocks, like coding regions ([`CdRegion`]), only make
//! sense when considered within the context of a [`SeqLoc`]. However, each
//! datablock is designed to fulfill a specific purpose and is independent
//! of others. This means that changes or additions to one datablock do not
//! affect the others.
//!
//! When a pre-existing object from another context is used as a datablock,
//! any software capable of utilizing that object can also operate on the
//! feature. For example, code that displays a publication can function with a
//! publication from a bibliographic database or one used as a sequence
//! feature without any modifications.
//!
//! The [`SeqFeat`] data structure and the [`SeqLoc`] used to attach it to the
//! sequence are shared among all features. This allows for a set of operations
//! that can be performed on all features, regardless of the type of datablocks
//! attached to them. Therefore, a function designed to determine all features
//! in a specific region of a Bioseq does not need to consider the specific
//! types of features involved.
//!
//! A [`SeqFeat`] is referred to as bipolar because it can have up to two
//! [`SeqLoc`]s. The [`SeqFeat`].location indicates the "source" and represents
//! the location on the DNA sequence, similar to the single location in common
//! feature table implementations. The `product` from [`SeqFeat`] represents
//! the "sink" and is typically associated with the protein sequence produced.
//! For example, a [`CdRegion`] feature would have its [`SeqFeat`].location on
//! the DNA and its [`SeqFeat`].product on the corresponding protein sequence.
//! This usage defines the process of translating a DNA sequence into a protein
//! sequence, establishing an explicit relationship between nucleic acid and
//! protein sequence databases.
//!
//! Having two [`SeqLoc`]s allows for a more comprehensive representation of
//! data conflicts or exceptional biological circumstances. For instance,
//! if an author presents a DNA sequence and its protein product in a figure,
//! it is possible to enter the DNA and protein sequences independently and
//! then confirm through the [`CdRegion`] feature that the DNA indeed translates
//! to the provided protein sequence. In cases where the DNA and protein do
//! not match, it could indicate an error in the database or the original
//! paper. By setting a "conflict" flag in the CdRegion, the problem can be
//! identified early and addressed. Additionally, there may be situations
//! where a genomic sequence cannot be translated to a protein due to known
//! biological reasons, such as RNA editing or suppressor tRNAs. In such
//! cases, the [`SeqFeat`] can be marked with an "exception" flag to indicate
//! that the data is correct but may not behave as expected.

use crate::biblio::{PubMedId, DOI};
use crate::general::{DbTag, IntFuzz, ObjectId, UserObject};
use crate::parsing_utils::{parse_int_to_option, parse_node_to, parse_node_to_option, parse_string_to, parse_vec_node_to_option, read_int, read_node};
use crate::r#pub::PubSet;
use crate::seq::{Heterogen, Numbering, PubDesc, SeqLiteral};
use crate::seqloc::{GiimportId, SeqId, SeqLoc};
use crate::{XmlNode, XmlVecNode};
use bitflags::bitflags;
use enum_primitive::FromPrimitive;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
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

impl XmlNode for FeatId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Feat-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variant tags
        let gibb_tag = BytesStart::new("Feat-id_gibb");
        let giim_tag = BytesStart::new("Feat-id_giim");
        let local_tag = BytesStart::new("Feat-id_local");
        let general_tag = BytesStart::new("Feat-id_general");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == local_tag.name() {
                        return Self::Local(read_node(reader).unwrap()).into();
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

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Experimental evidence for existence of feature
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum SeqFeatExpEvidence {
    /// any reasonable experiment check
    Experimental = 1,

    /// similarity, pattern, etc
    NotExperimental,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
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
    pub xref: Option<Vec<SeqFeatXref>>,

    /// support for xref to other databases
    pub dbxref: Option<Vec<DbTag>>,

    /// annotated on pseudogene
    pub pseudo: Option<bool>,

    /// explain if `except=true`
    pub except_text: Option<String>,

    /// set of id's; will replace `id` field
    pub ids: Option<Vec<FeatId>>,

    /// set of extensions; will replace `ext` field
    pub exts: Option<Vec<UserObject>>,

    pub support: Option<SeqFeatSupport>,
}

impl SeqFeat {
    /// not originally in spec
    pub fn default() -> Self {
        Self::new(SeqFeatData::User(UserObject::default()))
    }

    pub fn new(data: SeqFeatData) -> Self {
        Self {
            id: None,
            data,
            partial: None,
            except: None,
            comment: None,
            product: None,
            location: SeqLoc::default(),
            qual: None,
            title: None,
            ext: None,
            cit: None,
            exp_ev: None,
            xref: None,
            dbxref: None,
            pseudo: None,
            except_text: None,
            ids: None,
            exts: None,
            support: None,
        }
    }
}

impl XmlNode for SeqFeat {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Seq-feat")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut feat = Self::default();

        // attribute tags
        let id_tag = BytesStart::new("Seq-feat_id");
        let data_tag = BytesStart::new("Seq-feat_data");
        let partial_tag = BytesStart::new("Seq-feat_partial");
        let except_tag = BytesStart::new("Seq-feat_except");
        let comment_tag = BytesStart::new("Seq-feat_comment");
        let product_tag = BytesStart::new("Seq-feat_product");
        let location_tag = BytesStart::new("Seq-feat_location");
        let qual_tag = BytesStart::new("Seq-feat_qual");
        let title_tag = BytesStart::new("Seq-feat_title");
        let ext_tag = BytesStart::new("Seq-feat_ext");
        let cit_tag = BytesStart::new("Seq-feat_cit");
        let exp_ev_tag = BytesStart::new("Seq-feat_exp_ev");
        let xref_tag = BytesStart::new("Seq-feat_xref");
        let dbxref_tag = BytesStart::new("Seq-feat_db_xref");
        let pseudo_tag = BytesStart::new("Seq-feat_pseudo");
        let except_text_tag = BytesStart::new("Seq-feat_except_text");
        let ids_tag = BytesStart::new("Seq-feat_ids");
        let exts_tag = BytesStart::new("Seq-feat_exts");
        let support_tag = BytesStart::new("Seq-feat_support");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    parse_node_to_option(&name, &id_tag, &mut feat.id, reader);
                    parse_node_to(&name, &data_tag, &mut feat.data, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return feat.into()
                    }
                }
                _ => ()
            }
        }
    }
}
impl XmlVecNode for SeqFeat {}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Internal representation of chemical bond for [`SeqFeatData`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum SeqFeatBond {
    Disulfide = 1,
    Thiolester,
    XLink,
    Thioether,
    Other = 255,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Internal representation of site biochemical modification for [`SeqFeatData`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum SeqFeatSite {
    Active = 1,
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
    NpBinding,
    DnaBinding,
    SignalPeptide,
    TransitPeptide,
    TransmembraneRegion,
    Nitrosylation,
    Other = 255,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Internal representation of protein secondary structure for [`SeqFeatData`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum PSecStr {
    /// any helix
    Helix = 1,

    /// beta sheet
    Sheet,

    /// beta or gamma turn
    Turn,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
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

    #[serde(rename = "psec-str")]
    /// protein secondary structure
    PSecStr(PSecStr),

    #[serde(rename = "non-std-residue")]
    /// non-standard residue here in seq
    NonStdResidue(String),

    /// cofactor, prosthetic group, etc, bound to seq
    Het(Heterogen),

    BioSrc(BioSource),
    Clone(CloneRef),
    Variation(VariationRef),
}

impl XmlNode for SeqFeatData {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("SeqFeatData")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variant tags
        let gene_tag = BytesStart::new("SeqFeatData_gene");
        let _org_tag = BytesStart::new("SeqFeatData_org");
        let cdregion_tag = BytesStart::new("SeqFeatData_cdregion");
        let _prot_tag = BytesStart::new("SeqFeatData_prot");
        let _rna_tag = BytesStart::new("SeqFeatData_rna");
        let _pub_tag = BytesStart::new("SeqFeatData_pub");
        let _seq_tag = BytesStart::new("SeqFeatData_seq");
        let _imp_tag = BytesStart::new("SeqFeatData_imp");
        let _region_tag = BytesStart::new("SeqFeatData_region");
        let _bond_tag = BytesStart::new("SeqFeatData_bond");
        let _site_tag = BytesStart::new("SeqFeatData_site");
        let _rsite_tag = BytesStart::new("SeqFeatData_rsite");
        let _user_tag = BytesStart::new("SeqFeatData_user");
        let _txinit_tag = BytesStart::new("SeqFeatData_txinit");
        let _num_tag = BytesStart::new("SeqFeatData_num");
        let _psec_str_tag = BytesStart::new("SeqFeatData_psec-str");
        let _non_std_residue_tag = BytesStart::new("SeqFeatData_non-std-residue");
        let _het_tag = BytesStart::new("SeqFeatData_het");
        let _biosrc_tag = BytesStart::new("SeqFeatData_biosrc");
        let _clone_tag = BytesStart::new("SeqFeatData_clone");
        let _variation_tag = BytesStart::new("SeqFeatData_variation");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == gene_tag.name() {
                        return Self::Gene(read_node(reader).unwrap()).into()
                    }
                    if name == cdregion_tag.name() {
                        return Self::CdRegion(read_node(reader).unwrap()).into()
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
pub struct SeqFeatXref {
    pub id: Option<FeatId>,
    pub data: Option<SeqFeatData>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SeqFeatSupport {
    pub experiment: Option<Vec<ExperimentSupport>>,
    pub inference: Option<Vec<InferenceSupport>>,
    pub model_evidence: Option<Vec<ModelEvidenceSupport>>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Discrete types for types of experimental evidence
///
/// # Note
///
/// Original implementation lists this as `INTEGER`, therefore it is assumed that
/// serialized representation is an integer.
pub enum EvidenceCategory {
    NotSet,
    Coordinates,
    Description,
    Existence,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ExperimentSupport {
    pub category: Option<EvidenceCategory>,
    pub explanation: String,
    pub pmids: Option<Vec<PubMedId>>,
    pub dois: Option<Vec<DOI>>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ProgramId {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct EvidenceBasis {
    pub programs: Option<Vec<ProgramId>>,
    pub accessions: Option<Vec<SeqId>>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
/// Internal representation of inference support type for [`InferenceSupport`]
///
/// # Note
///
/// Original implementation lists this as `INTEGER`, therefore it is assumed that
/// serialized representation is an integer.
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct InferenceSupport {
    pub category: Option<EvidenceCategory>,
    #[serde(rename = "type")]
    pub r#type: InferenceSupportType,
    pub other_type: Option<String>,
    // TODO: default to false
    pub same_species: bool,
    pub basis: EvidenceBasis,
    pub pmids: Option<Vec<PubMedId>>,
    pub dois: Option<Vec<DOI>>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ModelEvidenceItem {
    pub id: SeqId,
    pub exon_count: Option<u64>,
    pub exon_length: Option<u64>,
    // TODO: default to false
    pub full_length: bool,
    // TODO: default to false
    pub supports_all_exon_combo: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ModelEvidenceSupport {
    pub method: Option<String>,
    pub mrna: Option<Vec<ModelEvidenceItem>>,
    pub est: Option<Vec<ModelEvidenceItem>>,
    pub protein: Option<Vec<ModelEvidenceItem>>,
    pub identification: Option<SeqId>,
    pub dbxref: Option<Vec<DbTag>>,
    pub exon_count: Option<u64>,
    pub exon_length: Option<u64>,
    pub full_length: bool,             // TODO: default false
    pub supports_all_exon_combo: bool, // TODO: default false
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
/// Internal representation of reading frame for [`CdRegion`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer.
pub enum CdRegionFrame {
    #[default]
    /// not set, code uses one
    NotSet,
    One,
    Two,
    /// reading frame
    Three,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "kebab-case")]
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

impl XmlNode for CdRegion {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Cdregion")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut cdregion = Self::default();

        // field tags
        let _orf_tag = BytesStart::new("Cdregion_orf");
        let _frame_tag = BytesStart::new("Cdregion_frame");
        let _conflict_tag = BytesStart::new("Cdregion_conflict");
        let gaps_tag = BytesStart::new("Cdregion_gaps");
        let mismatch_tag = BytesStart::new("Cdregion_mismatch");
        let code_tag = BytesStart::new("Cdregion_code");
        let _code_break_tag = BytesStart::new("Cdregion_code-break");
        let stops_tag = BytesStart::new("Cdregion_stops");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_vec_node_to_option(&name, &code_tag, &mut cdregion.code, reader);
                    parse_int_to_option(&name, &gaps_tag, &mut cdregion.gaps, reader);
                    parse_int_to_option(&name, &mismatch_tag, &mut cdregion.mismatch, reader);
                    parse_int_to_option(&name, &stops_tag, &mut cdregion.stops, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return cdregion.into()
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
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

impl XmlNode for GeneticCodeOpt {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Genetic-code_E")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variant tags
        let id_tag = BytesStart::new("Genetic-code_E_id");
        let name_tag = BytesStart::new("Genetic-code_E_name");
        let ncbieaa_tag = BytesStart::new("Genetic-code_E_ncbieaa");
        let ncbi8aa_tag = BytesStart::new("Genetic-code_E_ncbi8aa");
        let ncbistdaa_tag = BytesStart::new("Genetic-code_E_ncbistdaa");
        let sncbieaa_tag = BytesStart::new("Genetic-code_E_sncbieaa");
        let sncbi8aa_tag = BytesStart::new("Genetic-code_E_sncbi8aa");
        let sncbistdaa_tag = BytesStart::new("Genetic-code_E_sncbistdaa");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == id_tag.name() {
                        return Self::Id(read_int(reader).unwrap()).into()
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
impl XmlVecNode for GeneticCodeOpt {}

pub type GeneticCode = Vec<GeneticCodeOpt>;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
/// the amino acid that is the exception
pub enum CodeBreakAA {
    /// ASCII value of [`crate::asn::NCBIEaa`] code
    NcbiAa(u64),

    /// [`crate::asn::NCBI8aa`] code
    Ncbi8aa(u64),

    /// [`crate::asn::NCBIStdAa`] code
    NcbiStdAa(u64),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// specific codon exceptions
pub struct CodeBreak {
    /// location of exception
    pub loc: SeqLoc,

    /// the amino acid that is the exception
    pub aa: CodeBreakAA,
}

/// table of genetic codes
pub type GeneticCodeTable = Vec<GeneticCode>;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Features imported from other databases
pub struct ImpFeat {
    pub key: String,

    /// original location string
    pub loc: Option<String>,

    /// text description
    pub descr: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct GbQual {
    pub qual: String,
    pub val: String,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Internal representation of placement method for [`CloneRef`]
///
/// # Note
///
/// Original implementation lists this as `INTEGER`, therefore it is assumed that
/// serialized representation is an integer.
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// Specification of clone features
pub struct CloneRef {
    /// official clone symbol
    pub name: String,

    /// library name
    pub library: Option<String>,

    pub concordant: bool, // TODO: default to false
    pub unique: bool,     // TODO: default to false
    pub placement_method: Option<CloneRefPlacementMethod>,
    pub clone_seq: Option<CloneSeqSet>,
}

pub type CloneSeqSet = Vec<CloneSeq>;

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Internal representation of clone sequence type for [`CloneSeq`]
///
/// # Note
///
/// Original implementation lists this as `INTEGER`, therefore it is assumed that
/// serialized representation is an integer.
pub enum CloneSeqType {
    Insert,
    End,
    Other = 255,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Internal representation of clone confidence for [`CloneSeq`]
///
/// # Note
///
/// Original implementation lists this as `INTEGER`, therefore it is assumed that
/// serialized representation is an integer.
pub enum CloneSeqConfidence {
    /// multiple hits
    Multiple,

    /// unspecified
    Na,

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

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
///
/// # Note
///
/// Original implementation lists this as `INTEGER`, therefore it is assumed that
/// serialized representation is an integer.
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct CloneSeq {
    #[serde(rename = "type")]
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

bitflags! {
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct VariantResourceLink: u8 {
    /// Clinical, Pubmed, Cited
    const Preserved = 1;

    /// Provisional third party annotations
    const Provisional = 2;

    /// has 3D structure in SNP3D table
    const Has3D = 4;

    /// SNP -> SubSNP -> Batch link_out
    const SubmitterLinkout = 8;

    /// Clinical if LSDB, OMIM, TPA, Diagnostic
    const Clinical = 16;

    /// Marker exists on high density genotyping kit
    const GenotypeKit = 32;
}}

bitflags! {
    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    pub struct VariantGeneLocation: u32 {
        /// sequence intervals covered by a gene ID but not
        /// having an aligned transcript
        const InGene = 1;

        /// within 2kb of the 5' end of a gene feature
        const NearGene5 = 2;

        /// within 0.5kb of the 3' end of a gene feature
        const NearGene3 = 4;

        /// in intron
        const Intron = 8;

        /// in donor splice-site
        const Donor = 16;

        /// in acceptor splice-site
        const Acceptor = 32;

        /// in 5' UTR
        const UTR5 = 64;

        /// in 3' UTR
        const UTR3 = 128;

        /// the variant is observed in a start codon
        const InStartCodon = 256;

        /// the variant is observed in a stop codon
        const InStopCodon = 512;

        /// variant located between genes
        const Intergenic = 1024;

        /// variant is located in a conserved non-coding region
        const ConservedNoncoding = 2048;
    }
}

bitflags! {
    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    pub struct VariantEffect: u16 {
        /// known to cause no functional changes.
        /// since value is 0, it does not combine with any other bit, explicitly
        /// implying there are no consequences from SNP
        const NoChange = 0;

        /// one allele in the set does not change the encoded amino aced
        const Synonymous = 1;

        /// one allele in the set changes to STOP codon
        const Nonsense = 2;

        /// one allele in the set changes protein peptide
        const Missense = 4;

        /// one allele in the set changes all downstream amino acids
        const Frameshift = 8;

        /// the variant causes increased transcription
        const UpRegulator = 16;

        /// the variant causes decreased transcription
        const DownRegulator = 32;

        const Methylation = 64;

        /// reference codon is not stop codon, but the SNP variant allele changes
        /// the codon to a termination codon
        const StopGain = 128;

        /// reverse of [`Self::StopGain`]: reference codon is a stop codon, but
        /// a SNP variant allele changes the codon to a non-termination codon.
        const StopLoss = 256;
    }
}

bitflags! {
    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    pub struct VariantMapping: u8 {
        /// another SNP has the same mapped positions on reference assembly
        const HasOtherSnp = 1;

        /// weight 1 or 2 SNPs that map to different chromosomes on
        /// different assemblies
        const HasAssemblyConflict = 2;

        /// Only maps to 1 assembly
        const IsAssemblySpecific = 4;
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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

bitflags! {
    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    pub struct FrequencyBasedValidation: u8 {
        /// low frequency variation that is cited in journal or other reputable
        /// source.
        const IsMutation = 1;

        /// >5% minor allele freq in each and all populations
        const Above5pctAll = 2;

        /// >5% minor allele freq in 1+ populations
        const Above5pct1plus = 4;

        /// bit is set if the variant has a minor allele observed in two or
        /// more separate chromosomes.
        const Validated = 8;

        /// >1% minor allele frequency in each and all populations
        const Above1pctAll = 16;

        /// >1% minor allele frequency in 1+ populations
        const Above1pct1plus = 32;
    }

}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VariantGenotype {
    /// exists in a haplotype tagging set
    InHaplotypeSet,

    /// SNP has individual genotype
    HasGenotypes,
}

bitflags! {
    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    pub struct VariantQualityCheck: u8 {
        /// reference sequence allele at the mapped position is not present in
        /// the SNP allele list, adjust for orientation
        const ContigAlleleMissing = 1;

        /// one member SS is withdrawn by submitter
        const WithdrawnBySubmitter = 2;

        /// RS set has 2+ alleles from different submissions and these sets
        /// share no alleles in common
        const NonOverlappingAlleles = 4;

        /// strain specific fixed difference
        const StrainSpecific = 8;

        /// has genotype conflict
        const GenotypeConflict = 16;
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VariantConfidence {
    Unknown,
    LikelyArtifact,
    Other = 255,
}

bitflags! {
    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    /// origin of this allele, if known
    ///
    /// # See also
    /// - [`bitflags`](https://docs.rs/bitflags/latest/bitflags/macro.bitflags.html)
    ///   to view how multiple values may be held by this object
    pub struct VariantAlleleOrigin: u32 {
        const Unknown = 0;
        const Germline = 1;
        const Somatic = 2;
        const Inherited = 4;
        const Paternal = 8;
        const Maternal = 16;
        const DeNovo = 32;
        const Biparental = 64;
        const Uniparental = 128;
        const NotTested = 256;
        const TestedInconclusive = 512;
        const NotReported = 1024;

        /// stopper - 2^31
        const Other = 1073741824;
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
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

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Phenotype {
    pub source: Option<String>,
    pub term: Option<String>,
    pub xref: Option<Vec<DbTag>>,

    /// does this variant have known clinical significance?
    pub clinical_significance: Option<PhenotypeClinicalSignificance>,
}

bitflags! {
    #[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
    /// This field is an explicit bitflag
    pub struct PopulationDataFlags: u8 {
        const IsDefaultPopulation = 1;
        const IsMinorAllele = 2;
        const IsRareAllele = 4;
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PopulationData {
    /// assayed population (eg: HAPMAP-CEU)
    pub population: String,
    pub genotype_frequency: Option<f64>,
    pub chromosomes_tested: Option<u64>,
    pub sample_ids: Option<Vec<ObjectId>>,
    pub allele_frequency: Option<f64>,
    pub flags: Option<PopulationDataFlags>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ExtLoc {
    pub id: ObjectId,
    pub location: SeqLoc,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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
    Pcr,
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

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct VariationRefDataSet {
    #[serde(rename = "type")]
    pub r#type: VariationRefDataSetType,
    pub variations: Vec<VariationRef>,
    pub name: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
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

    Variations(Vec<VariationRef>),

    /// variant is a complex and un-described change at the location
    ///
    /// This type of variant is known to occur in dbVar submissions
    Complex,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// see http://www.hgvs.org/mutnomen/recs-prot.html
pub struct VariationFrameshift {
    pub phase: Option<i64>,
    pub x_length: Option<i64>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct VariationLossOfHeterozygosity {
    /// in germline comparison, it will be reference genome assembly
    /// (default) or referenece/normal population. In somatic mutation,
    /// it will be a name of the normal tissue
    pub reference: Option<String>,

    /// name of the testing subject type or the testing issue
    pub test: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SomaticOriginCondition {
    pub description: Option<String>,

    /// reference to BioTerm / other descriptive database
    pub object_id: Option<Vec<DbTag>>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct VariationSomaticOrigin {
    /// description of the somatic origin itself
    pub source: Option<SubSource>,

    /// condition related to this origin's type
    pub condition: Option<SomaticOriginCondition>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
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
    pub other_ids: Option<Vec<DbTag>>,

    /// names and synonyms
    /// some variants have well-known canonical names and possible
    /// accepted synonyms
    pub name: Option<String>,
    pub synonyms: Option<Vec<String>>,

    /// tag for comment and descriptions
    pub description: Option<String>,

    /// phenotype
    pub phenotype: Option<Vec<Phenotype>>,

    /// sequencing / acquisition method
    pub method: Option<Vec<VariantRefMethod>>,

    /// variant properties bitflags
    pub variant_prop: Option<VariantProperties>,

    pub data: VariationRefData,
    pub consequence: Option<Vec<VariationConsequence>>,
    pub somatic_origin: Option<Vec<VariationSomaticOrigin>>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum DeltaSeq {
    Literal(SeqLiteral),
    Loc(SeqLoc),

    /// same location as [`VariationRef`] itself
    This,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DeltaItem {
    pub seq: Option<DeltaSeq>,

    /// multiplier allows representing a tandem, eg: ATATAT as AT*3
    ///
    /// This allows describing CNV/SSR where delta=self  with a
    /// multiplier which specifies the count of the repeat unit.
    pub multiplier: Option<i64>, // assumed 1 if not specified
    pub multiplier_fuzz: Option<IntFuzz>,

    pub action: DeltaAction,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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
    Snv,

    /// `delta=[morph of length >1]`
    Mnp,

    #[serde(rename = "delins")]
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
    Cnv,

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

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// Used to label items in a [`VariationRef`] package
pub enum VariationInstObservation {
    /// represents the asserted base at a position
    Asserted = 1,

    /// represents the reference base at the position
    Reference = 2,

    /// represent the observed variant at a given position
    Variant = 4,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct VariationInst {
    #[serde(rename = "type")]
    pub r#type: VariationInstType,

    /// sequence that replaces the location, in biological order
    pub delta: Vec<DeltaItem>,

    /// used to label items in a [`VariationRef`] package
    ///
    /// This field is explicitly a bitflag
    pub observation: Option<VariationInstObservation>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RSiteRef {
    /// may be unparsable
    Str(String),

    /// pointer to a restriction site database
    DB(DbTag),
}

#[allow(non_camel_case_types)]
#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RnaRefExt {
    /// for naming "other" type
    Name(String),

    /// for tRNA's
    tRNA(TRnaExt),

    /// generic fields for `ncRNA`, `tmRNA` and `miscRNA`
    Gen(RnaGen),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RnaRef {
    #[serde(rename = "type")]
    pub r#type: RnaRefType,
    pub pseudo: Option<bool>,
    pub ext: Option<RnaRefExt>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TRnaExtAa {
    IUPACAa(u64),
    NCBIEaa(u64),
    NCBI8aa(u64),
    NCBIStdAa(u64),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// tRNA feature extensions
pub struct TRnaExt {
    /// transported amino acid
    pub aa: TRnaExtAa,

    /// codon(s) as in [`GeneticCode`]
    pub codon: Option<Vec<u64>>,

    /// location of anticodon
    pub anticodon: Option<SeqLoc>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RnaGen {
    /// for ncRNA's, the class of non-coding RNA
    ///
    /// examples: antisense RNA, guide RNA, snRNA
    pub class: Option<String>,

    pub product: Option<String>,

    /// eg: `tag_peptide` qualifier for tmRNA's
    pub quals: Option<RnaQualSet>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct RnaQual {
    pub qual: String,
    pub val: String,
}

pub type RnaQualSet = Vec<RnaQual>;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "kebab-case")]
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
    pub pseudo: bool,

    /// ids in other dbases
    pub db: Option<Vec<DbTag>>,

    /// synonyms for locus
    pub syn: Option<Vec<String>>,

    /// systematic gene name (eg: MI0001, ORF0069)
    pub locus_tag: Option<String>,

    pub formal_name: Option<GeneNomenclature>,
}

impl XmlNode for GeneRef {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Gene-ref")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut gene = Self::default();

        // field tags
        let locus_tag = BytesStart::new("Gene-ref_locus");
        let allele_tag = BytesStart::new("Gene-ref_allele");
        let desc_tag = BytesStart::new("Gene-ref_desc");
        let maploc_tag = BytesStart::new("Gene-ref_maploc");
        let pseudo_tag = BytesStart::new("Gene-ref_pseudo");
        let db_tag = BytesStart::new("Gene-ref_db");
        let syn_tag = BytesStart::new("Gene-ref_syn");
        let locus_tag_tag = BytesStart::new("Gene-ref_locus-tag");
        let form_name_tag = BytesStart::new("Gene-ref_formal-name");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_string_to(&name, &locus_tag, &mut gene.locus, reader);
                    parse_string_to(&name, &allele_tag, &mut gene.allele, reader);
                    parse_string_to(&name, &desc_tag, &mut gene.desc, reader);
                    parse_string_to(&name, &maploc_tag, &mut gene.maploc, reader);
                    parse_vec_node_to_option(&name, &db_tag, &mut gene.db, reader);
                    parse_string_to(&name, &locus_tag_tag, &mut gene.locus_tag, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return gene.into()
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum GeneNomenclatureStatus {
    Unknown,
    Official,
    Interim,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct GeneNomenclature {
    pub status: GeneNomenclatureStatus,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub source: Option<DbTag>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
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
    pub r#mod: Option<Vec<String>>,

    /// ids in taxonomic or culture databases
    pub db: Option<Vec<DbTag>>,

    /// synonyms for `taxname` or common
    pub syn: Option<Vec<String>>,

    pub orgname: Option<OrgName>,
}

impl XmlNode for OrgRef {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Org-ref")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut org_ref = OrgRef::default();

        let taxname_element = BytesStart::new("Org-ref_taxname");
        let db_element = BytesStart::new("Org-ref_db");
        let orgname_element = BytesStart::new("Org-ref_orgname");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_string_to(&name, &taxname_element, &mut org_ref.taxname, reader);
                    parse_node_to_option(&name, &orgname_element, &mut org_ref.orgname, reader);
                    parse_vec_node_to_option(&name, &db_element, &mut org_ref.db, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return org_ref.into();
                    }
                }
                _ => (),
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
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

impl XmlNode for OrgNameChoice {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("OrgName_name")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        // variants
        let binomial_element = BytesStart::new("OrgName_name_binomial");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == binomial_element.name() {
                        return Self::Binomial(read_node(reader).unwrap()).into();
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return None;
                    }
                }
                _ => (),
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct OrgName {
    pub name: Option<OrgNameChoice>,

    /// attribution of name
    pub attrib: Option<String>,

    #[serde(rename = "mod")]
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

impl XmlNode for OrgName {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("OrgName")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut org_name = OrgName::default();

        let name_element = BytesStart::new("OrgName_name");
        let attrib_element = BytesStart::new("OrgName_attrib");
        let mod_element = BytesStart::new("OrgName_mod");
        let lineage_element = BytesStart::new("OrgName_lineage");
        let gcode_element = BytesStart::new("OrgName_gcode");
        let div_element = BytesStart::new("OrgName_div");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_string_to(&name, &div_element, &mut org_name.div, reader);
                    parse_string_to(&name, &attrib_element, &mut org_name.attrib, reader);
                    parse_string_to(&name, &lineage_element, &mut org_name.lineage, reader);
                    parse_int_to_option(&name, &gcode_element, &mut org_name.gcode, reader);
                    parse_node_to_option(&name, &name_element, &mut org_name.name, reader);
                    parse_vec_node_to_option(&name, &mod_element, &mut org_name.r#mod, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return org_name.into();
                    }
                }
                _ => (),
            }
        }
    }
}

enum_from_primitive! {
    #[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
    #[repr(u8)]
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
}

/// default not defined in original spec
impl Default for OrgModSubType {
    fn default() -> Self {
        Self::Other
    }
}

impl XmlNode for OrgModSubType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("OrgMod_subtype")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_u8(read_int(reader).unwrap())
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct OrgMod {
    pub subtype: OrgModSubType,
    pub subname: String,

    /// attribution/source of name
    pub attrib: Option<String>,
}

impl XmlNode for OrgMod {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("OrgMod")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut r#mod = Self::default();

        let subtype_element = BytesStart::new("OrgMod_subtype");
        let subname_element = BytesStart::new("OrgMod_subname");
        let attrib_element = BytesStart::new("OrgMod_attrib");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_node_to(&name, &subtype_element, &mut r#mod.subtype, reader);
                    parse_string_to(&name, &subname_element, &mut r#mod.subname, reader);
                    parse_string_to(&name, &attrib_element, &mut r#mod.attrib, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return r#mod.into();
                    }
                }
                _ => (),
            }
        }
    }
}
impl XmlVecNode for OrgMod {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct BinomialOrgName {
    /// required
    pub genus: String,

    /// species required if subspecies used
    pub species: Option<String>,
    pub subspecies: Option<String>,
}

impl XmlNode for BinomialOrgName {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("BinomialOrgName")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut binomial = BinomialOrgName::default();

        let genus_element = BytesStart::new("BinomialOrgName_genus");
        let species_element = BytesStart::new("BinomialOrgName_species");
        let subspecies_element = BytesStart::new("BinomialOrgName_subspecies");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_string_to(&name, &genus_element, &mut binomial.genus, reader);
                    parse_string_to(&name, &species_element, &mut binomial.species, reader);
                    parse_string_to(&name, &subspecies_element, &mut binomial.subspecies, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return binomial.into();
                    }
                }
                _ => (),
            }
        }
    }
}

/// the first value will be used to assign division
pub type MultiOrgName = Vec<OrgName>;

/// used when genus not known
pub type PartialOrgName = Vec<TaxElement>;

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum TaxElementFixedLevel {
    /// level must be set in string
    Other,

    Family,
    Order,
    Class,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct TaxElement {
    pub fixed_level: TaxElementFixedLevel,
    pub level: Option<String>,
    pub name: String,
}

enum_from_primitive! {
    #[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
    #[repr(u8)]
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
        InsertionSeq,
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
}

impl XmlNode for BioSourceGenome {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("BioSource_genome")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        Self::from_u8(read_int(reader).unwrap())
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "kebab-case")]
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

impl XmlNode for BioSource {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("BioSource")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut source = Self::default();

        let genome_element = BytesStart::new("BioSource_genome");
        let _origin_element = BytesStart::new("BioSource_origin");
        let org_element = BytesStart::new("BioSource_org");
        let subtype_element = BytesStart::new("BioSource_subtype");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_node_to(&name, &genome_element, &mut source.genome, reader);
                    parse_node_to(&name, &org_element, &mut source.org, reader);
                    parse_vec_node_to_option(&name, &subtype_element, &mut source.subtype, reader);
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return source.into();
                    }
                }
                _ => (),
            }
        }
    }
}

pub type PCRReationSet = Vec<PCRReaction>;
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PCRReaction {
    pub forward: Option<PCRPrimerSet>,
    pub reverse: Option<PCRPrimerSet>,
}

pub type PCRPrimerSet = Vec<PCRPrimer>;
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PCRPrimer {
    pub seq: Option<PCRPrimerSeq>,
    pub name: Option<PCRPrimerName>,
}
pub type PCRPrimerSeq = String;
pub type PCRPrimerName = String;

enum_from_primitive! {
    #[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
    #[repr(u8)]
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
}

/// default not in original spec
impl Default for SubSourceSubType {
    fn default() -> Self {
        Self::Other
    }
}

impl XmlNode for SubSourceSubType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("SubSource_subtype")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        Self::from_u8(read_int(reader).unwrap())
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct SubSource {
    pub subtype: SubSourceSubType,
    pub name: String,

    /// attribution/source of this name
    pub attrib: Option<String>,
}

impl XmlNode for SubSource {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("SubSource")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut source = Self::default();

        let subtype_element = BytesStart::new("SubSource_subtype");
        let name_element = BytesStart::new("SubSource_name");
        let attrib_element = BytesStart::new("SubSource_attrib");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let qname = e.name();

                    parse_node_to(&qname, &subtype_element, &mut source.subtype, reader);
                    parse_string_to(&qname, &name_element, &mut source.name, reader);
                    parse_string_to(&qname, &attrib_element, &mut source.attrib, reader);
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return source.into();
                    }
                }
                _ => (),
            }
        }
    }
}
impl XmlVecNode for SubSource {}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
pub enum ProtRefProcessingStatus {
    #[default]
    NotSet,

    PreProtein,
    Mature,
    SignalPeptide,
    TransitPeptide,
    ProPeptide,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
/// Reference to a protein name
pub struct ProtRef {
    /// protein name
    pub name: Option<Vec<String>>,

    /// description (instead of name)
    pub desc: Option<String>,

    /// E.C. number(s)
    pub ec: Option<Vec<String>>,

    /// activities
    pub activity: Option<Vec<String>>,

    /// id's in other dbases
    pub db: Option<Vec<DbTag>>,

    /// processing status
    pub processed: ProtRefProcessingStatus,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
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
    pub mapping_precise: bool, // TODO: default false

    /// does [`SeqLoc`] reflect mapping
    pub location_accurate: bool, // TODO: default false

    pub inittype: InitType,
    pub evidence: Option<Vec<TxEvidence>>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
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

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct TxEvidence {
    pub exp_code: TxEvidenceExpCode,
    pub expression_system: TxEvidenceExpressionSystem,
    pub low_prec_data: bool, // TODO: default false
    pub from_homolog: bool,  // TODO: default false
}
