/*
NCBI Entrezgene data definitions

Adapted from NCBI ASN.1 specification for Entrezgene at
https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/entrezgene/entrezgene.asn

--$Revision: 76722 $ 
--********************************************************************** 
-- 
--  NCBI Entrezgene 
--  by James Ostell, 2001 
--   
--  Generic "Gene" object for Entrez Genes 
--    This object is designed to incorporate a subset of information from 
--    LocusLink and from records in Entrez Genomes to provide indexing, 
--    linkage, and a useful summary report in Entrez for "Genes" 
-- 
--********************************************************************** 

*/
 
/*
EXPORTS Entrezgene, Entrezgene-Set, Gene-track, Gene-commentary;
 
IMPORTS Gene-ref FROM NCBI-Gene 
    Prot-ref FROM NCBI-Protein 
    BioSource FROM NCBI-BioSource 
    RNA-ref FROM NCBI-RNA 
    Dbtag, Date FROM NCBI-General 
    Seq-loc FROM NCBI-Seqloc 
    Pub FROM NCBI-Pub; 
*/

use crate::seqloc::SeqLoc ;
use crate::r#pub::Pub ;
use crate::seqfeat::{BioSource, GeneRef, ProtRef, RnaRef, RnaRefType} ;

use crate::general::{Date, DbTag, PersonId};
use crate::parsing::{read_vec_node, read_int, read_node, read_string, read_vec_str_unchecked, UnexpectedTags, read_bool_attribute};
use crate::parsing::{XmlNode, XmlVecNode};

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;


/*
# Original comment

--******************************************** 
-- Entrezgene is the "document" indexed in Entrez 
--  and presented in the full display 
-- It also contains the Entrez ID and date information 
--******************************************* 
*/

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
pub enum EntrezgeneType {
    #[default]
    Unknown = 0,
    TRna = 1,
    RRna = 2,
    SnRna = 3,
    ScRna = 4,
    SnoRna = 5,
    ProteinCoding = 6,
    Pseudo = 7,
    Transposon = 8,
    MiscRna = 9,
    NcRna = 10,
    BiologicalRegion = 11,
    Other = 255,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Entrezgene { 
    #[serde(rename = "track-info")]
    pub track_info: Option<GeneTrack> ,            //-- not in submission, but in retrieval 
    #[serde(rename = "type")]
    pub r#type: EntrezgeneType ,
    pub source: BioSource , 
    pub gene: GeneRef ,                           //-- for locus-tag see note 3
    pub prot: Option<ProtRef> , 
    pub rna: Option<RnaRef>, 
    pub summary: Option<String> ,                 //-- short summary 
    pub location: Option<Vec<Maps>> ,
    #[serde(rename = "gene-source")]
    pub gene_source: Option<GeneSource> ,          //-- NCBI source to Entrez 
    pub locus: Option<Vec<GeneCommentary>> ,      //-- location of gene on chromosome (if known)
                                                  //-- and all information about products
                                                  //-- (mRNA, proteins and so on)
    pub properties: Option<Vec<GeneCommentary>> , 
    pub refgene: Option<Vec<GeneCommentary>> ,    //-- NG for this? 
    pub homology: Option<Vec<GeneCommentary>> , 
    pub comments: Option<Vec<GeneCommentary>> ,
    #[serde(rename = "unique-keys")]
    pub unique_keys: Option<Vec<DbTag>> ,          //-- see note 3
    #[serde(rename = "xtra-index-terms")]
    pub xtra_index_terms: Option<Vec<String>> ,     //-- see note 2
    #[serde(rename = "xtra-properties")]
    pub xtra_properties: Option<Vec<XtraTerms>> ,  //-- see note 2
    #[serde(rename = "xtra-iq")]
    pub xtra_iq: Option<Vec<XtraTerms>> ,          //-- see note 2
    #[serde(rename = "non-unique-keys")]
    pub non_unique_keys: Option<Vec<DbTag>> ,
}

pub type EntrezgeneSet = Vec<Entrezgene>;

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum GeneTrackStatus {
    Live = 0,
    Secondary = 1,
    Discontinued = 2,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub struct GeneTrack { 
    pub geneid: u64 ,     //-- required unique document id 
    pub status: GeneTrackStatus ,
    pub current_id: Option<Vec<DbTag>> , //-- see note 1 below
    pub create_date: Date ,   //-- date created in Entrez 
    pub update_date: Date ,   //-- last date updated in Entrez 
    pub discontinue_date: Option<Date>,
}
 
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub struct GeneSource { 
    pub src: String ,                //-- key to the source within NCBI locuslink, Ecoli, etc 
    #[serde(rename = "src-int")]
    pub src_int: Option<u64> ,         //-- eg. locuslink id 
    #[serde(rename = "src-str1")]
    pub src_str1: Option<String> ,  //-- eg. chromosome1 
    #[serde(rename = "src-str2")]
    pub src_str2: Option<String> ,  //-- see note 3
    #[serde(default)]
    #[serde(rename = "gene-display")]
    pub gene_display: bool , // DEFAULT FALSE ,  //-- do we have a URL for gene display? 
    #[serde(default)]
    #[serde(rename = "locus-display")]
    pub locus_display: bool , // DEFAULT FALSE , //-- do we have a URL for map/locus display? 
    #[serde(default)]
    #[serde(rename = "extra-terms")]
    pub extra_terms: bool , // DEFAULT FALSE,   //-- do we have a URL for extra indexing terms? 
}


#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
pub enum GeneCommentaryType {
    Genomic = 1 ,
    #[serde(rename = "pre-RNA")]
    PreRna = 2 ,
    #[serde(rename = "mRNA")]
    MRna = 3 ,
    #[serde(rename = "rRNA")]
    RRna = 4 ,
    #[serde(rename = "rRNA")]
    TRna = 5 ,
    #[serde(rename = "snRNA")]
    SnRNA = 6 ,
    #[serde(rename = "scRNA")]
    ScRNA = 7 ,
    #[serde(rename = "peptide")]
    Peptide = 8 ,
    #[serde(rename = "other-genetic")]
    OtherGenetic = 9 ,
    #[serde(rename = "genome-mRNA")]
    GenomicMrna = 10 ,
    #[serde(rename = "cRNA")]
    CRna = 11 ,
    #[serde(rename = "mature-peptide")]
    MaturePeptide = 12 ,
    #[serde(rename = "pre-protein")]
    PreProtein = 13 ,
    #[serde(rename = "miscRNA")]
    MiscRNA  = 14 ,
    #[serde(rename = "snoRNA")]
    SnoRNA  = 15 ,
    #[serde(rename = "property")]
    Property  = 16 , //-- used to display tag/value pair
                     //-- for this type label is used as property tag, text is used as property value, 
                     //-- other fields are not used.
    Reference = 17 , //-- currently not used             
    Generif = 18 ,   //-- to include generif in the main blob             
    Phenotype= 19 ,  //-- to display phenotype information
    Complex = 20 ,   //-- used (but not limited) to identify resulting 
                     //-- interaction complexes
    Compound = 21 ,  //-- pubchem entities

    #[serde(rename = "ncRNA")]
    NcRna = 22 , 
    #[serde(rename = "gene-group")]
    GeneGroup = 23 ,//-- for relationship sets (such as pseudogene / parent gene)
    #[serde(rename = "assembly")]
    Assembly = 24 ,  //-- for full assembly accession
    #[serde(rename = "assembly-unit")]
    AssemblyUnit = 25 , //-- for the assembly unit corresponding to the refseq
    #[serde(rename = "c-region")]
    CRegion = 26 ,
    #[serde(rename = "d-segment")]
    DSegment = 27 ,
    #[serde(rename = "j-segment")]
    JSegment = 28 ,
    #[serde(rename = "v-segment")]
    VSegment = 29 ,

    Comment = 254 ,
    Other = 255 ,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub struct GeneCommentary { 
    #[serde(rename = "type")]
    pub r#type: GeneCommentaryType ,
    pub heading: Option<String> ,      //-- appears above text 
    pub label: Option<String> ,        //-- occurs to left of text
                                       //-- for protein and RNA types it is a name
                                       //-- for property type it is a property tag  
    pub text: Option<String> ,         //-- block of text 
                                       //-- for property type it is a property value  
    pub accession: Option<String> ,    //-- accession for the gi in the seqloc, see note 3
    pub version: Option<u64> ,          //-- version for the accession above
    #[serde(rename = "xtra-properties")]
    pub xtra_properties: Option<Vec<XtraTerms>> , //-- see note 2
    pub refs: Option<Vec<Pub>> ,       //-- refs for this 
    pub source: Option<Vec<OtherSource>> ,    //-- links and refs 
    #[serde(rename = "genomic-coords")]
    pub genomic_coords: Option<Vec<SeqLoc>> , //-- referenced sequences in genomic coords
    pub seqs: Option<Vec<SeqLoc>> ,       //-- referenced sequences in non-genomic coords
    pub products: Option<Vec<GeneCommentary>> ,
    pub properties: Option<Vec<GeneCommentary>> ,
    pub comment: Option<Vec<GeneCommentary>> ,
    pub create_date: Option<Date> ,   
    pub update_date: Option<Date> ,   
    pub rna: Option<RnaRef> ,
}
 
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub struct OtherSource { 
    pub src: Option<DbTag> ,         //-- key to non-ncbi source 
    #[serde(rename = "pre-text")]
    pub pre_text: Option<String> ,   //-- text before anchor 
    pub anchor: Option<String> ,     //-- text to show as highlight 
    pub url: Option<String> ,        //-- if present, use this URL not Dbtag and datbase 
    #[serde(rename = "post-text")]
    pub post_text: Option<String> ,
}  //-- text after anchor 

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum MapType {
    Cyto = 0,
    Bp = 1,
    CM = 2,
    CR = 3,
    Min = 4,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MapsMethodChoice {
    /// --url to non mapviewer mapviewing resource
    Proxy(String),
    /// --units used in display-str to query mapviewer 
    MapType(MapType),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Maps {
    #[serde(rename = "display-string")]
   pub display_str: String ,
   pub method: MapsMethodChoice ,
}
                        
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct XtraTerms {  //-- see note 2
    pub tag: String ,
    pub value: String ,
}

/*

# Original Comment

--********************************************************************** 
-- 
--  Comments, notes, etc.
--   
--  1)  Ignored unless status = secondary.  This is where gene_ids (db = "GeneID")
--      are placed toward which the interface will direct users.  It is also
--      available for placing other source-db specific tags (i.e., db = "LocusID").
--
--  2)  These 'xtra' objects are for submitting data for Entrez indexing
--      that might not fit anywhere in the Entrezgene specification but
--      are considered by the data source submittor to be important.
--          xtra-index-terms is any string.
--          xtra-properties are tag/value pairs of properties/feilds as
--              defined in the Entrez database (i.e.: UNIGENE/Hs.74561)
--          xtra-iq are tag/value pairs of Entrez database/UID as defined
--              in the Entrezgene indexing code (i.e.: NUCLEOTIDE/20270626)
--
--  3)  Locus-tag and src-str2 are expected to be unique per organism (tax_id).
--      Protein accessions and the tag-value pairs in unique-keys
--      are expected to be unique over all organisms.
--********************************************************************** 
*/

/// # Implementations


impl XmlNode for Entrezgene {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Entrezgene")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        println!("Starting parsing of <Entrezgene>");
  
        let mut gene = Entrezgene {
            track_info: None,
            r#type: EntrezgeneType::Unknown,
            source: BioSource::default(),
            gene: GeneRef::default(),
            prot: None,
            rna: None,
            summary: None,
            location: None,
            gene_source: None,
            locus: None,
            properties: None,
            refgene: None,
            homology: None,
            comments: None,
            unique_keys: None,
            xtra_index_terms: None,
            xtra_properties: None,
            xtra_iq: None,
            non_unique_keys: None,
        };

        use quick_xml::events::BytesStart;

        let forbidden_tags = [
            BytesStart::new("extra-field"),
            BytesStart::new("other-field"),
        ];
        let forbidden = UnexpectedTags(&forbidden_tags);
        

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    println!("D Entrezgene: Encountered tag {:?}", e.name());
                    match e.name().as_ref() {
                        b"track-info" => gene.track_info = read_node(reader),
                        b"Entrezgene_track-info" => gene.track_info = read_node(reader),
                        b"type" => gene.r#type = read_node(reader).unwrap(),
                        b"Entrezgene_type" => gene.r#type = read_node(reader).unwrap(),
                        b"source" => gene.source = read_node(reader).unwrap(),
                        b"Entrezgene_source" => gene.source = read_node(reader).unwrap_or_default(),
                        b"gene" => gene.gene = read_node(reader).unwrap(),
                        b"Entrezgene_gene" => gene.gene = read_node(reader).unwrap(),
                        b"prot" => gene.prot = read_node(reader),
                        b"Entrezgene_prot" => gene.prot = read_node(reader),
                        b"rna" => gene.rna = read_node(reader),
                        b"summary" => gene.summary = read_string(reader),
                        b"Entrezgene_summary" => gene.summary = read_string(reader),
                        b"location" => gene.location = Some(read_vec_node(reader, e.to_end())),
                        b"Entrezgene_location" => gene.location = Some(read_vec_node(reader, e.to_end())),
                        b"gene-source" => gene.gene_source = read_node(reader),
                        b"Entrezgene_gene-source" => gene.gene_source = read_node(reader),
                        b"locus" => gene.locus = Some(read_vec_node(reader, e.to_end())),
                        b"Entrezgene_locus" => gene.locus = Some(read_vec_node(reader, e.to_end())),
                        b"properties" => gene.properties = Some(read_vec_node(reader, e.to_end())),
                        b"Entrezgene_properties" => gene.properties = Some(read_vec_node(reader, e.to_end())),
                        b"comments" => gene.comments = Some(read_vec_node(reader, e.to_end())),
                        b"Entrezgene_comments" => gene.comments = Some(read_vec_node(reader, e.to_end())),
                        b"unique-keys" => gene.unique_keys = Some(read_vec_node(reader, e.to_end())),
                        b"Entrezgene_unique-keys" => gene.unique_keys = Some(read_vec_node(reader, e.to_end())),
                        b"xtra-index-terms" =>  gene.xtra_index_terms = Some(read_vec_node(reader, e.to_end())),
                        b"Entrezgene_xtra-index-terms" =>  gene.xtra_index_terms = Some(read_vec_node(reader, e.to_end())),
                        b"xtra-properties" => gene.xtra_properties = Some(read_vec_node(reader, e.to_end())) ,
                        b"Entrezgene_xtra-properties" => gene.xtra_properties = Some(read_vec_node(reader, e.to_end())) ,
                        b"xtra-iq" => gene.xtra_iq = Some(read_vec_node(reader, e.to_end())),
                        b"Entrezgene_xtra-iq" => gene.xtra_iq = Some(read_vec_node(reader, e.to_end())),
                        b"non-unique-keys" => gene.non_unique_keys = Some(read_vec_node(reader, e.to_end())) ,
                        b"Entrezgene_non-unique-keys" => gene.non_unique_keys = Some(read_vec_node(reader, e.to_end())) ,
                        _ => forbidden.check(&e.name()),
                    }
                },    
                Event::End(e) => {
                    println!("D: Entrezgene: Finished parsing {:?}", e.name());

                    if e.name() == Self::start_bytes().name() {
                        return Some(gene);
                    }
                },
                Event::Eof => break,
                _ => (),
            }
        }
        None
    }
}

impl XmlVecNode for Entrezgene {}

impl XmlNode for XtraTerms {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Xtra-terms")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        Some(XtraTerms::default()) // Placeholder implementation
    }
}

impl XmlVecNode for XtraTerms {}

impl XmlNode for GeneSource {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Gene-source")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut source = GeneSource::default();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => match e.name().as_ref() {
                    b"Src" => source.src = read_string(reader).unwrap_or_default(),
                    b"SrcInt" => source.src_int = read_int(reader),
                    b"SrcStr1" => source.src_str1 = read_string(reader),
                    b"SrcStr2" => source.src_str2 = read_string(reader),
                    _ => (),
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return Some(source);
                    }
                }
                Event::Eof => break,
                _ => (),
            }
        }
        None
    }
}

// should go elsewhere
impl XmlNode for RnaRef {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Rna-ref")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        Some(RnaRef::default()) // Placeholder: add real parsing logic here
    }
}

impl XmlNode for Maps {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Maps")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        Some(Maps::default()) // Placeholder: Add parsing logic
    }
}

impl XmlVecNode for Maps {}

impl XmlNode for String {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("String")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        read_string(reader)
    }
}

impl XmlVecNode for String {}

impl XmlNode for GeneCommentary {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Gene-commentary")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut commentary = GeneCommentary {
            r#type: GeneCommentaryType::Other,
            heading: None,
            label: None,
            text: None,
            accession: None,
            version: None,
            xtra_properties: None,
            refs: None,
            source: None,
            genomic_coords: None,
            seqs: None,
            products: None,
            properties: None,
            comment: None,
            create_date: None,
            update_date: None,
            rna: None,
        };

        let forbidden_tags = [BytesStart::new("unknown-tag")];
        let forbidden = UnexpectedTags(&forbidden_tags);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => match e.name().as_ref() {
                    b"type" => commentary.r#type = read_node(reader).unwrap(),
                    b"Gene-commentary_type" => commentary.r#type = read_node(reader).unwrap(),
                    b"heading" => commentary.heading = read_string(reader),
                    b"Gene-commentary_heading" => commentary.heading = read_string(reader),
                    b"label" => commentary.label = read_string(reader),
                    b"Gene-commentary_label" => commentary.label = read_string(reader),
                    b"text" => commentary.text = read_string(reader),
                    b"Gene-commentary_text" => commentary.text = read_string(reader),
                    b"accession" => commentary.accession = read_string(reader),
                    b"Gene-commentary_accession" => commentary.accession = read_string(reader),
                    b"version" => commentary.version = Some(read_string(reader).unwrap().parse().unwrap()),
                    b"Gene-commentary_version" => commentary.version = Some(read_string(reader).unwrap().parse().unwrap()),
                    b"xtra-properties" => commentary.xtra_properties = Some(read_vec_node(reader, e.to_end())) ,
                    b"Gene-commentary_xtra-properties" => commentary.xtra_properties = Some(read_vec_node(reader, e.to_end())) ,
                    b"refs" => commentary.refs = Some(read_vec_node(reader, e.to_end())),
                    b"Gene-commentary_refs" => commentary.comment = Some(read_vec_node(reader, e.to_end())) ,
                    b"seqs" => commentary.seqs = Some(read_vec_node(reader, e.to_end())),
                    b"Gene-commentary_seqs" => commentary.seqs = Some(read_vec_node(reader, e.to_end())) ,
                    b"source" => commentary.seqs = Some(read_vec_node(reader, e.to_end())),
                    b"Gene-commentary_source" => commentary.source = Some(read_vec_node(reader, e.to_end())) ,
                    b"Gene-commentary_products" => commentary.products = Some(read_vec_node(reader, e.to_end())) ,
                    b"genomic-coords" => commentary.genomic_coords = Some(read_vec_node(reader, e.to_end())) ,
                    b"Gene-commentary_genomic-coords" => commentary.genomic_coords = Some(read_vec_node(reader, e.to_end())) ,
                    b"Gene-commentary_comment" => commentary.comment = Some(read_vec_node(reader, e.to_end())) ,
                    b"Gene-commentary_create-date" => commentary.create_date = read_node(reader) ,
                    b"Gene-commentary_update-date" => commentary.update_date = read_node(reader) ,
                    _ => forbidden.check(&e.name()),
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return Some(commentary);
                    }
                }
                Event::Eof => break,
                _ => (),
            }
        }
        None
    }
}

impl XmlVecNode for GeneCommentary {}


impl XmlVecNode for GeneTrack {}

impl XmlNode for EntrezgeneType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("type")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let text = read_string(reader).unwrap_or_default();
        match text.as_str() {
            "unknown" => Some(EntrezgeneType::Unknown),
            "tRNA" => Some(EntrezgeneType::TRna),
            "rRNA" => Some(EntrezgeneType::RRna),
            "snRNA" => Some(EntrezgeneType::SnRna),
            "scRNA" => Some(EntrezgeneType::ScRna),
            "snoRNA" => Some(EntrezgeneType::SnoRna),
            "protein-coding" => Some(EntrezgeneType::ProteinCoding),
            "pseudo" => Some(EntrezgeneType::Pseudo),
            "transposon" => Some(EntrezgeneType::Transposon),
            "miscRNA" => Some(EntrezgeneType::MiscRna),
            "ncRNA" => Some(EntrezgeneType::NcRna),
            "biological-region" => Some(EntrezgeneType::BiologicalRegion),
            "other" => Some(EntrezgeneType::Other),
            _ => Some(EntrezgeneType::Unknown),
        }
    }
}

impl XmlNode for GeneCommentaryType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Gene-commentary-type")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let text = read_string(reader).unwrap_or_default();
        match text.as_str() {
            "Genomic" => Some(Self::Genomic) ,
            "pre-RNA" => Some(Self::PreRna) ,
            "mRNA" => Some(Self::MRna) ,
            "rRNA" => Some(Self::RRna) ,
            "rRNA" => Some(Self::TRna) ,
            "snRNA" => Some(Self::SnRNA) ,
            "scRNA" => Some(Self::ScRNA) ,
            "peptide" => Some(Self::Peptide) ,
            "other-genetic" => Some(Self::OtherGenetic) ,
            "genome-mRNA" => Some(Self::GenomicMrna) ,
            "cRNA" => Some(Self::CRna) ,
            "mature-peptide" => Some(Self::MaturePeptide) ,
            "pre-protein" => Some(Self::PreProtein) ,
            "miscRNA" => Some(Self::MiscRNA)  ,
            "snoRNA" => Some(Self::SnoRNA) ,
            "property" => Some(Self::Property) , //-- used to display tag/value pair
                     //-- for this type label is used as property tag, text is used as property value, 
                     //-- other fields are not used.
             "reference" =>Some(Self::Reference) , //-- currently not used             
             "generif" => Some(Self::Generif) ,   //-- to include generif in the main blob             
             "phenotype" => Some(Self::Phenotype),  //-- to display phenotype information
             "complex" => Some(Self::Complex) ,   //-- used (but not limited) to identify resulting 
                     //-- interaction complexes
             "compound" => Some(Self::Compound) ,  //-- pubchem entities
             "ncRNA" => Some(Self::NcRna) , 
             "gene-group" => Some(Self::GeneGroup) ,//-- for relationship sets (such as pseudogene / parent gene)
             "assembly" => Some(Self::Assembly) ,  //-- for full assembly accession
             "assembly-unit" => Some(Self::AssemblyUnit) , //-- for the assembly unit corresponding to the refseq
             "c-region" => Some(Self::CRegion) ,
             "d-segment" => Some(Self::DSegment) ,
             "j-segment" => Some(Self::JSegment) ,
             "v-segment" => Some(Self::VSegment) ,
             "comment" => Some(Self::Comment) ,
             "other" => Some(Self::Other) ,
             &_ => Some(Self::Other) ,
        }
             
    }
}
impl XmlNode for GeneTrackStatus {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Gene-track-status")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let text = read_string(reader).unwrap_or_default();
        match text.as_str() {
            "live" => Some(GeneTrackStatus::Live),
            "discontinued" => Some(GeneTrackStatus::Discontinued),
            _ => None,
        }
    }
}

impl XmlVecNode for SeqLoc {}

impl Default for XtraTerms {
    fn default() -> Self {
        XtraTerms {
            tag: String::new(),      // Default empty string
            value: String::new(),    // Default empty string
        }
    }
}

impl Default for GeneSource {
    fn default() -> Self {
        GeneSource {
            src: String::new(),
            src_int: None,
            src_str1: None,
            src_str2: None,
            extra_terms: false,         // Optional, initialize as `None`
            gene_display: false,       // Default `false`
            locus_display: false,      // Default `false`
        }
    }
}

impl Default for RnaRef {
    fn default() -> Self {
        RnaRef {
            ext: None,                   // Optional, initialize as `None`
            pseudo: Some(false),         // Default `false`
            r#type: RnaRefType::Unknown, // Default enum variant
        }
    }
}

impl Default for Maps {
    fn default() -> Self {
        Maps {
            display_str: String::new(), // Default empty string
            method: MapsMethodChoice::MapType(MapType::Bp),       // Optional, initialize as `Bp` (Basepair)
        }
    }
}

impl XmlNode for Vec<Entrezgene> {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Entrezgene-Set")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut genes = Vec::new();
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
        
                    if name == BytesStart::new("Entrezgene").name() {
                        if let Some(entrezgene) = read_node::<Entrezgene>(reader) {
                            genes.push(entrezgene);
                        } else {
                            println!("Skipping a failed <Entrezgene>");
                        }
                    } else {
                        forbidden.check(&name); // Check unexpected tags here
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    println!("Successfully finished parsing <Entrezgene-Set>");
                    return Some(genes);
                }
                Event::Text(e) => {
                    // Step 1: Store the unescaped string into a variable
                    let unescaped = e.unescape().unwrap_or_default();
                    
                    // Step 2: Trim the string to remove unnecessary whitespace
                    let text = unescaped.trim();
                    
                    if !text.is_empty() {
                        println!("Unexpected text between nodes: '{}'", text);
                    }
                }

                Event::Eof => {
                    println!("Unexpected EOF while parsing <Entrezgene-Set>");
                    break;
                }
                _ => (), // Catch all other events
            }
        }
        
        None
    }
}

impl Default for GeneTrack {
    fn default() -> Self {
        GeneTrack {
            geneid: 0,                                // Default `0`
            status: GeneTrackStatus::Live,            // Default enum variant
            current_id: None,                         // Initialize as `None`
            create_date: Date::default(),             // Default for `Date`
            update_date: Date::default(),             // Default for `Date`
            discontinue_date: None,                   // Initialize as `None`
        }
    }
}


impl XmlNode for GeneTrack {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Gene-track") // This MUST match the XML element name exactly
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut track = GeneTrack::default();

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => match e.name().as_ref() {
                    b"geneid" => track.geneid = read_string(reader).unwrap().parse().unwrap(),
                    b"status" => track.status = read_node(reader).unwrap(),
                    b"current-id" => track.current_id = Some(read_vec_node(reader, e.to_end())),
                    b"create-date" => track.create_date = read_node(reader).unwrap(),
                    b"update-date" => track.update_date = read_node(reader).unwrap(),
                    b"discontinue-date" => track.discontinue_date = read_node(reader),
                    _ => (), // Ignore unknown tags
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return Some(track);
                    }
                }
                Event::Eof => break,
                _ => (),
            }
        }
        None
    }
}


impl XmlNode for OtherSource {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("OtherSource")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut source = Self::default();
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => match e.name().as_ref() {
                    b"src" => source.src = read_node(reader),
                    b"pre-text" => source.pre_text = read_string(reader),
                    b"anchor" => source.anchor = read_string(reader),
                    b"url" => source.url = read_string(reader),
                    b"post-text" => source.post_text = read_string(reader),
                    _ => forbidden.check(&e.name()),
                },
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(source);
                    }
                }
                _ => (),
            }
        }
    }
}

// This tells the compiler that `OtherSource` can be parsed as a list of nodes
impl XmlVecNode for OtherSource {}