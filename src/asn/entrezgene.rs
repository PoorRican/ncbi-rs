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

use crate::general::{Date, DbTag} ;
use crate::seqloc::SeqLoc ;
use crate::r#pub::Pub ;
use crate::seqfeat::{BioSource, GeneRef, ProtRef, RnaRef} ;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
pub enum GeneType {
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
    pub r#type: GeneType ,
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
 
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
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
    MapType(String),
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