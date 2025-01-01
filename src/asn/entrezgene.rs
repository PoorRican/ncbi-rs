//! https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/entrezgene/entrezgene.asn

use enum_primitive::FromPrimitive;
use crate::general::{Date, DbTag};
use crate::parsing::{attribute_value, greedy_read_string, read_attributes, read_int, read_node, read_string, read_vec_node, UnexpectedTags, XmlNode, XmlValue, XmlVecNode};
use crate::r#pub::Pub;
use crate::seqfeat::{BioSource, GeneRef, ProtRef, RnaRef};
use crate::seqloc::SeqLoc;
use quick_xml::events::{BytesStart, Event};
use quick_xml::events::attributes::Attributes;
use quick_xml::Reader;
use serde_repr::{Deserialize_repr, Serialize_repr};

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    #[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
    #[repr(u8)]
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
}

impl XmlNode for EntrezgeneType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Entrezgene_type")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        EntrezgeneType::from_u8(read_int::<u8>(reader).unwrap())
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct EntrezGene {
    // TODO: look at reference XML for how summary, location, et al are created
    pub track_info: Option<GeneTrack>,
    pub r#type: EntrezgeneType,
    pub source: BioSource,
    pub gene: GeneRef,
    pub prot: ProtRef,
    pub rna: RnaRef,
    pub summary: Option<String>,
    pub location: Option<Maps>,     // NOTE: assuming that `<Maps></Maps>` will contain parallel data
    pub gene_source: Option<GeneSource>,       // NCBI source to Entrez
    pub locus: Option<Vec<GeneCommentary>>,

    pub properties: Option<Vec<GeneCommentary>>,
    pub refgene: Option<Vec<GeneCommentary>>,
    pub homology: Option<Vec<GeneCommentary>>,
    pub comments: Option<Vec<GeneCommentary>>,
    pub unique_keys: Option<Vec<DbTag>>,
    pub xtra_index_terms: Option<String>,        // see note 2
    pub xtra_properties: Option<Vec<XtraTerms>>, // see note 2
    pub xtra_iq: Option<Vec<XtraTerms>>,         // see note 2
    pub non_unique_keys: Option<Vec<DbTag>>,
}

impl XmlNode for EntrezGene {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Entrezgene")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let track_info_elem = BytesStart::new("Entrezgene_track-info");
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

        // ignore unimplemented elements
        let unimplemented = [
            rna_elem,
            summary_elem,
            locus_elem,
            properties_elem,
            refgene_elem,
            homology_elem,
            comments_elem,
            unique_keys_elem,
            xtra_iq_elem,
            non_unique_keys_elem,
        ];
        let unimplemented = UnexpectedTags(&unimplemented);

        let mut buffer = Self::default();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    // TODO: use raw bytes instead of `BytesStart`
                    if name == track_info_elem.name() {
                        buffer.track_info = read_node(reader)
                    } else if name == source_elem.name() {
                        let tmp = read_node(reader);
                        buffer.source = if let Some(tmp) = tmp {
                            tmp
                        } else {
                            // TODO: raise warning for missing source
                            //       continue pattern with other elements
                            BioSource::default()
                        }
                    } else if name == type_elem.name() {
                        buffer.r#type = read_node(reader).unwrap()
                    } else if name == gene_elem.name() {
                        buffer.gene = read_node(reader).unwrap()
                    } else if name == prot_elem.name() {
                        buffer.prot = read_node(reader).unwrap()
                    } else if name == location_elem.name() {
                        buffer.location = read_node(reader).into()
                    } else if name == gene_source_elem.name() {
                        buffer.gene_source = read_node(reader)
                    } else if name == xtra_properties_elem.name() {
                        buffer.xtra_properties = read_vec_node(reader, xtra_properties_elem.to_end()).into();
                    } else if name == xtra_index_terms_elem.name() {
                        buffer.xtra_index_terms = greedy_read_string(reader)
                    } else {
                        // TODO: print warning for unknown element
                        continue
                    }
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return buffer.into()
                    }
                },
                _ => ()
            }
        }

    }
}

impl XmlVecNode for EntrezGene {}

#[derive(Clone, PartialEq)]
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
        let start_bytes = Self::start_bytes();

        Some(EntrezGene::vec_from_reader(reader, start_bytes.to_end()).into())
    }
}

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    #[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
    #[repr(u8)]
    pub enum GeneTrackStatus {
        #[default]
        Live,
        Secondary,    // synonym with merged
        Discontinued, // 'deleted', still index and display to public
    }
}

impl XmlNode for GeneTrackStatus {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Gene-track_status")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        GeneTrackStatus::from_u8(read_int::<u8>(reader).unwrap())
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct GeneTrackInner {
    pub geneid: i32, // required unique document id
    pub status: GeneTrackStatus,
    pub current_id: Option<Vec<DbTag>>,
    pub create_date: Date, // date created in Entrez
    pub update_date: Date, // last date updated in Entrez
    pub discontinue_date: Option<Date>,
}

impl XmlNode for GeneTrackInner {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Gene-track")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized
    {
        let geneid_elem = BytesStart::new("Gene-track_geneid");
        let status_elem = BytesStart::new("Gene-track_status");
        let current_id_elem = BytesStart::new("Gene-track_current-id");
        let create_date_elem = BytesStart::new("Gene-track_create-date");
        let update_date_elem = BytesStart::new("Gene-track_update-date");
        let discontinue_date_elem = BytesStart::new("Gene-track_discontinue-date");

        let mut buffer = Self::default();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == geneid_elem.name() {
                        buffer.geneid = read_int(reader).unwrap()
                    } else if name == status_elem.name() {
                        buffer.status = read_node(reader).unwrap()
                    } else if name == current_id_elem.name() {
                        buffer.current_id = Some(read_vec_node(reader, current_id_elem.to_end()))
                    } else if name == create_date_elem.name() {
                        buffer.create_date = read_node(reader).unwrap()
                    } else if name == update_date_elem.name() {
                        buffer.update_date = read_node(reader).unwrap()
                    } else if name == discontinue_date_elem.name() {
                        buffer.discontinue_date = read_node(reader)
                    } else {
                        panic!("Unexpected tag: {:?}", name)
                    }
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return buffer.into()
                    }
                },
                _ => ()
            }
        }
    }
}

impl XmlVecNode for GeneTrackInner {}

pub type GeneTrack = Vec<GeneTrackInner>;

impl XmlNode for GeneTrack {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Entrezgene_track-info")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized
    {
        let mut buffer = Vec::new();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == GeneTrackInner::start_bytes().name() {
                        buffer.push(read_node(reader).unwrap())
                    } else {
                        eprintln!("Unexpected tag: {:?}", name)
                    }
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return buffer.into()
                    }
                },
                _ => ()
            }
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct GeneSourceInner {
    src: String,              // key to the source within NCBI locuslink, Ecoli, etc
    src_int: Option<i32>,     // eg: locuslink id
    src_str1: Option<String>, // eg: chromosome1
    src_str2: Option<String>, // see note 3
}

pub type GeneSource = Vec<GeneSourceInner>;

impl XmlNode for GeneSource {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Entrezgene_gene-source")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized
    {
        let src_elem = BytesStart::new("Gene-source_src");
        let src_int_elem = BytesStart::new("Gene-source_src-int");
        let src_str1_elem = BytesStart::new("Gene-source_src-str1");
        let src_str2_elem = BytesStart::new("Gene-source_src-str2");

        let mut buffer: Option<GeneSourceInner> = None;
        let mut vec_buffer = Vec::new();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == src_elem.name() {
                        if let Some(buffer) = buffer.take() {
                            vec_buffer.push(buffer)
                        }
                        buffer = GeneSourceInner::default().into();
                        buffer.as_mut().unwrap().src = read_string(reader).unwrap()
                    } else if name == src_int_elem.name() {
                        buffer.as_mut().unwrap().src_int = read_int(reader)
                    } else if name == src_str1_elem.name() {
                        buffer.as_mut().unwrap().src_str1 = read_string(reader)
                    } else if name == src_str2_elem.name() {
                        buffer.as_mut().unwrap().src_str2 = read_string(reader)
                    } else {
                        eprintln!("Unexpected tag: {:?}", name)
                    }
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return vec_buffer.into()
                    }
                },
                _ => ()
            }
        }
    }
}

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub struct OtherSourceInner {
    src: Option<DbTag>,        // key to non-ncbi source
    pre_text: Option<String>,  // text before anchor
    anchor: Option<String>,    // text to show as highlight
    url: Option<String>,       // if present, use this URL not DbTag and database
    post_text: Option<String>, // text after anchor
}

pub type OtherSource = Vec<OtherSourceInner>;

/// Units used in display-str to query map viewer
#[derive(Clone, PartialEq, Debug)]
pub enum MapsMethodType {
    Cyto,
    BP,
    cM,
    cR,
    Min,
}

impl XmlValue for MapsMethodType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Maps_method_map-type")
    }

    fn from_attributes(attributes: Attributes) -> Option<Self> {
        if let Some(attributes) = attribute_value(attributes) {
            match attributes.as_str() {
                "cyto" => Self::Cyto.into(),
                "bp" => Self::BP.into(),
                "cm" => Self::cM.into(),
                "cr" => Self::cR.into(),
                "min" => Self::Min.into(),
                _ => None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum MapsMethod {
    Proxy(String),
    MapType(MapsMethodType),
}

impl XmlNode for MapsMethod {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Maps_method")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized
    {
        let proxy_elem = BytesStart::new("Maps_method_proxy");
        let map_type_elem = BytesStart::new("Maps_method_map-type");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    // TODO: proxy element might be an empty element
                    if name == proxy_elem.name() {
                        return MapsMethod::Proxy(read_string(reader).unwrap()).into()
                    } else {
                        panic!("Unexpected tag: {:?}", name)
                    }
                },
                Event::Empty(e) => {
                    let name = e.name();

                    // TODO: proxy element might be an empty element
                    if name == proxy_elem.name() {
                        return MapsMethod::Proxy(attribute_value(e.attributes()).unwrap()).into()
                    } else if name == map_type_elem.name() {
                        return MapsMethod::MapType(read_attributes(&e)?).into()
                    } else {
                        panic!("Unexpected tag: {:?}", name)
                    }
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return None
                    }
                },
                _ => ()
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MapsInner {
    display_str: String,
    method: MapsMethod,
}

pub type Maps = Vec<MapsInner>;

impl XmlNode for Maps {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Maps")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized
    {
        let display_str_elem = BytesStart::new("Maps_display-str");
        let method_elem = BytesStart::new("Maps_method");

        let mut vec_buffer = Vec::new();
        let mut display_str_buffer = None;

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == display_str_elem.name() {
                        display_str_buffer = read_string(reader);
                    } else if name == method_elem.name() {
                        vec_buffer.push(
                            MapsInner {
                                display_str: display_str_buffer.take().unwrap(),
                                method: read_node(reader).unwrap()
                            }
                        );
                    } else if name != Self::start_bytes().name() {
                        eprintln!("Unexpected tag: {:?}", name);
                    }
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return vec_buffer.into()
                    }
                },
                _ => ()
            }
        }
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
/// See note 2
pub struct XtraTermsInner {
    pub tag: String,
    pub value: String,
}

/// See note 2
pub type XtraTerms = Vec<XtraTermsInner>;

impl XmlNode for XtraTerms {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Xtra-terms")
    }

    // assumes that `Xtra-terms` valid tag/value pairs, instead of encapsulating values
    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized
    {
        let tag_elem = BytesStart::new("Xtra-terms_tag");
        let value_elem = BytesStart::new("Xtra-terms_value");

        let mut terms_buffer = Vec::new();
        let mut val_buffer_tag = None;

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == tag_elem.name() {
                        val_buffer_tag = Some(read_string(reader).unwrap());
                    } else if name == value_elem.name() {
                        terms_buffer.push(
                            XtraTermsInner {
                                tag: val_buffer_tag.take().unwrap(),
                                value: read_string(reader).unwrap()
                            }
                        )
                    } else {
                        panic!("Unexpected tag: {:?}", name)
                    }
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return terms_buffer.into()
                    }
                },
                _ => ()
            }
        }
    }
}

impl XmlVecNode for XtraTerms {}

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
//       are considered by the data source submitter to be important.
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
