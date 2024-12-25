//! https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/entrezgene/entrezgene.asn

use crate::general::{Date, DbTag};
use crate::parsing::{read_node, read_vec_node, XmlNode, XmlVecNode};
use crate::r#pub::Pub;
use crate::seqfeat::{BioSource, GeneRef, ProtRef, RnaRef};
use crate::seqloc::SeqLoc;
use quick_xml::events::{BytesStart};
use quick_xml::Reader;

#[derive(Default)]
pub enum EntrezgeneType {
    #[default]
    Unknown,
    tRNA,
    rRNA,
    snRNA,
    scRNA,
    snoRNA,
    ProteinCoding,
    Pseudo,
    miscRNA,
    ncRNA,
    BiologicalRegion,
    Other,
}

pub struct EntrezGene {
    // TODO: look at reference XML for how summary, location, et al are created
    track_info: Option<GeneTrack>,
    r#type: EntrezgeneType,
    source: BioSource,
    gene: GeneRef,
    prot: ProtRef,
    rna: RnaRef,
    summary: Option<String>,
    location: Option<Vec<Maps>>,
    // TODO: gene_source: Option<GeneSource>,       // NCBI source to Entrez
    locus: Option<Vec<GeneCommentary>>,

    properties: Option<Vec<GeneCommentary>>,
    refgene: Option<Vec<GeneCommentary>>,
    homology: Option<Vec<GeneCommentary>>,
    comments: Option<Vec<GeneCommentary>>,
    unique_keys: Option<Vec<DbTag>>,
    xtra_index_terms: Option<String>,        // see note 2
    xtra_properties: Option<Vec<XtraTerms>>, // see note 2
    xtra_iq: Option<Vec<XtraTerms>>,         // see note 2
    non_unique_keys: Option<Vec<DbTag>>,
}

impl XmlNode for EntrezGene {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Entrezgene")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let track_info_elem = BytesStart::new("Entrez_track-info");
        let source_elem = BytesStart::new("Entrezgene_source");
        let type_elem = BytesStart::new("Entrezgene_type");
        let gene_elem = BytesStart::new("Entrezgene_gene");
        let prot_elem = BytesStart::new("Entrezgene_prot");
        let rna_elem = BytesStart::new("Entrezgene_rna");
        let summary_elem = BytesStart::new("Entrezgene_summary");
        let location_elem = BytesStart::new("Entrezgene_location");
        let gene_source_elem = BytesStart::new("Entrezgene_gene-source");
        let locus_elem = BytesStart::new("Entrezgene_locus");
        let properties_elem = BytesStart::new("Entrezgene_properties");
        let refgene_elem = BytesStart::new("Entrezgene_refgene");
        let homology_elem = BytesStart::new("Entrezgene_homology");
        let comments_elem = BytesStart::new("Entrezgene_comments");
        let unique_keys_elem = BytesStart::new("Entrezgene_unique-keys");
        let xtra_index_terms_elem = BytesStart::new("Entrezgene_xtra-index-terms");
        let xtra_properties_elem = BytesStart::new("Entrezgene_xtra-properties");
        let xtra_iq_elem = BytesStart::new("Entrezgene_xtra-iq");
        let non_unique_keys_elem = BytesStart::new("Entrezgene_non-unique-keys");
    }
}

impl XmlVecNode for EntrezGene {}

#[derive(Default)]
pub struct EntrezGeneSet(Vec<EntrezGene>);

impl EntrezGeneSet {
    pub fn new() -> Self {
        EntrezGeneSet(Vec::new())
    }

    pub fn push(&mut self, gene: EntrezGene) {
        self.0.push(gene);
    }

    pub fn iter(&self) -> std::slice::Iter<EntrezGene> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<EntrezGene> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn remove(&mut self, index: usize) -> EntrezGene {
        self.0.remove(index)
    }

    pub fn get(&self, index: usize) -> Option<&EntrezGene> {
        self.0.get(index)
    }
}

impl From<Vec<EntrezGene>> for EntrezGeneSet {
    fn from(vec: Vec<EntrezGene>) -> Self {
        EntrezGeneSet(vec)
    }
}

impl XmlNode for EntrezGeneSet {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Entrezgene-Set")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        Some(EntrezGene::vec_from_reader(reader, Self::start_bytes().to_end().into()).into())
    }
}

pub enum GeneTrackStatus {
    Live,
    Secondary,    // synonym with merged
    Discontinued, // 'deleted', still index and display to public
}

pub struct GeneTrackInner {
    geneid: i32, // required unique document id
    status: GeneTrackStatus,
    current_id: Option<Vec<DbTag>>,
    create_date: Date, // date created in Entrez
    update_date: Date, // last date updated in Entrez
    discontinue_date: Option<Date>,
}

pub type GeneTrack = Vec<GeneTrackInner>;

pub struct GeneSourceInner {
    src: String,              // key to the source within NCBI locuslink, Ecoli, etc
    src_int: Option<i32>,     // eg: locuslink id
    src_str1: Option<String>, // eg: chromosome1
    src_str2: Option<String>, // see note 3
}

#[repr(u8)]
pub enum GeneCommentaryType {
    Genomic = 1,
    PreRNA = 2,
    mRNA = 3,
    rRNA = 4,
    tRNA = 5,
    snRNA = 6,
    scRNA = 7,
    peptide = 8,
    OtherGenetic = 9,
    GenomicmRNA = 10,
    cRNA = 11,
    MaturePeptide = 12,
    PreProtein = 13,
    MiscRNA = 14,
    snoRNA = 15,
    // used to display tag/value pair
    // for this type label is used as property tag, text is used as property value
    // other fields are not used
    Property = 16,


    Reference = 17, // currently not used
    Generif = 18,   // to include generif in the main blob
    Phenotype = 19, // to include phenotype information
    Complex = 20,   // used (but not limited) to identify  resulting interaction complexes
    Compound = 21,  // pubchem entities
    ncRNA = 22,
    GeneGroup = 23,    // for relationship sets (such as pseudogene / parent gene)
    Assembly = 24,     // for full assembly accession
    AssemblyUnit = 25, // for the assembly unit corresponding to the refseq
    cRegion = 26,
    dSegment = 27,
    jSegment = 28,
    vSegment = 29,
    Comment = 254,
    Other = 255,
}

pub struct GeneCommentaryInner {
    r#type: GeneCommentaryType,
    heading: Option<String>,
    // for protein and RNA types it is a name
    // for property type it is a property tag
    label: Option<String>,
    text: Option<String>,                    // block of text
    accession: Option<String>,               // accession for the gi in the seqloc, see note 3
    version: Option<u32>,                    // version for the accession above
    xtra_properties: Option<Vec<XtraTerms>>, // see note 2
    refs: Option<Vec<Pub>>,                  // refs for this
    source: Option<Vec<OtherSource>>,        // links and refs
    genomic_coords: Option<Vec<SeqLoc>>,     // referenced sequences in genomic coords
    seqs: Option<Vec<SeqLoc>>,               // referenced sequences in non-genomic coords
    products: Option<Vec<GeneCommentary>>,
    properties: Option<Vec<GeneCommentary>>,
    comment: Option<Vec<GeneCommentary>>,
    create_date: Option<Date>,
    update_date: Option<Date>,
    rna: Option<RnaRef>,
}

pub type GeneCommentary = Vec<GeneCommentaryInner>;

pub struct OtherSourceInner {
    src: Option<DbTag>,        // key to non-ncbi source
    pre_text: Option<String>,  // text before anchor
    anchor: Option<String>,    // text to show as highlight
    url: Option<String>,       // if present, use this URL not DbTag and database
    post_text: Option<String>, // text after anchor
}

pub type OtherSource = Vec<OtherSourceInner>;

// Units used in display-str to query map viewer
pub enum MapsMethodType {
    Cyto,
    BP,
    cM,
    cR,
    Min,
}

pub enum MapsMethod {
    Proxy(String),
    MapType(MapsMethodType),
}

pub struct MapsInner {
    display_str: String,
    method: MapsMethod,
}

pub type Maps = Vec<MapsInner>;

/// See note 2
pub struct XtraTermsInner {
    tag: String,
    value: String,
}

/// See note 2
pub type XtraTerms = Vec<XtraTermsInner>;

// **********************************************************************
//
//   Comments, notes, etc.
//
//   1)  Ignored unless status = secondary.  This is where gene_ids (db = "GeneID")
//       are placed toward which the interface will direct users.  It is also
//       available for placing other source-db specific tags (i.e., db = "LocusID").
//
//   2)  These 'xtra' objects are for submitting data for Entrez indexing
//       that might not fit anywhere in the Entrezgene specification but
//       are considered by the data source submittor to be important.
//           xtra-index-terms is any string.
//           xtra-properties are tag/value pairs of properties/feilds as
//               defined in the Entrez database (i.e.: UNIGENE/Hs.74561)
//           xtra-iq are tag/value pairs of Entrez database/UID as defined
//               in the Entrezgene indexing code (i.e.: NUCLEOTIDE/20270626)
//
//   3)  Locus-tag and src-str2 are expected to be unique per organism (tax_id).
//       Protein accessions and the tag-value pairs in unique-keys
//       are expected to be unique over all organisms.
// **********************************************************************
