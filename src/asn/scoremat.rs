
/*
 * Auto-transformed by chatGPT o1 on 20241214
 * Little editing when snake-case was over-used.
 * Comments reintroduced manually.
 */

/*
 * $Id: scoremat.asn 98157 2022-10-05 15:42:23Z lanczyck $
 * ===========================================================================
 *
 *                            PUBLIC DOMAIN NOTICE
 *               National Center for Biotechnology Information
 * --
 *  This software/database is a "United States Government Work" under the
 *  terms of the United States Copyright Act.  It was written as part of
 *  the author's official duties as a United States Government employee and
 *  thus cannot be copyrighted.  This software/database is freely available
 *  to the public for use. The National Library of Medicine and the U.S.
 *  Government have not placed any restriction on its use or reproduction.
 * --
 *  Although all reasonable efforts have been taken to ensure the accuracy
 *  and reliability of the software and data, the NLM and the U.S.
 *  Government do not and cannot warrant the performance or results that
 *  may be obtained by using this software or data. The NLM and the U.S.
 *  Government disclaim all warranties, express or implied, including
 *  warranties of performance, merchantability or fitness for any particular
 *  purpose.
 * --
 *  Please cite the author in any work or product based on this material.
 * --
 * ===========================================================================
 * --
 * Author:  Christiam Camacho
 * --
 * File Description:
 *      ASN.1 definitions for scoring matrix
 * --
 * ===========================================================================
 */

/*
EXPORTS    Pssm, PssmIntermediateData, PssmFinalData,
           PssmParameters, PssmWithParameters;

IMPORTS    Object-id   FROM NCBI-General
           Seq-entry   FROM NCBI-Seqset;
*/

use crate::parsing::{read_bool_attribute, read_double, read_int, read_string, read_vec_double_unchecked, read_vec_int_unchecked, read_vec_node, read_vec_str_unchecked, UnexpectedTags};
use crate::parsing::{XmlNode, XmlVecNode};

use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

use enum_primitive::FromPrimitive;

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    /// Enumeration for BlockProperty types
    #[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Default)]
    #[repr(u8)]
    pub enum BlockPropertyType {
        #[default]
        Unassigned = 0,
        Threshold = 1, /// score threshold for heuristics
        MinScore = 2, /// observed minimum score in CD
        MaxScore = 3, /// observed maximum score in CD
        MeanScore = 4, /// observed meanscore in CD
        Variance = 5, /// observed score variance
        Name = 10, /// just name the block
        IsOptional = 20, /// block may not have to be used
        Other = 255,
    }
}

impl XmlNode for BlockPropertyType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("BlockPropertyType")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut block_property_type = BlockPropertyType::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    if e.name() == QName(b"BlockPropertyType") {
                        block_property_type = BlockPropertyType::from_i64(read_int(reader)?).unwrap_or_default();
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"BlockPropertyType") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(block_property_type)
    }
}

/// BlockProperty structure
/// a rudimentary block/core-model, to be used with block-based alignment
/// routines and threading
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct BlockProperty {
    pub r#type: BlockPropertyType,
    pub intvalue: Option<i64>,
    pub textvalue: Option<String>,
}

impl XmlNode for BlockProperty {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("BlockProperty")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut block_property = BlockProperty::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        QName(b"type") => {
                            block_property.r#type = BlockPropertyType::from_i64(read_int(reader)?).unwrap_or_default();
                        }
                        QName(b"intvalue") => {
                            block_property.intvalue = Some(read_int(reader)?);
                        }
                        QName(b"textvalue") => {
                            block_property.textvalue = Some(read_string(reader)?);
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"BlockProperty") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(block_property)
    }
}

impl XmlVecNode for BlockProperty {}

/// CoreBlock structure
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct CoreBlock {
    pub start: i64, // begin of block on query
    pub stop: i64, // end of block on query
    pub minstart: Option<i64>,// optional N-terminal extension
    pub maxstop: Option<i64>, // optional C-terminal extension
    pub property: Option<Vec<BlockProperty>>,
}

impl XmlNode for CoreBlock {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("CoreBlock")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut core_block = CoreBlock::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        QName(b"start") => {
                            core_block.start = read_int(reader)?;
                        }
                        QName(b"stop") => {
                            core_block.stop = read_int(reader)?;
                        }
                        QName(b"minstart") => {
                            core_block.minstart = Some(read_int(reader)?);
                        }
                        QName(b"maxstop") => {
                            core_block.maxstop = Some(read_int(reader)?);
                        }
                        QName(b"property") => {
                            core_block.property = Some(read_vec_node(reader, e.to_end()));
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"CoreBlock") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(core_block)
    }
}

impl XmlVecNode for CoreBlock {}

/// LoopConstraint structure
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct LoopConstraint {
    /// Defaults to 0 minimum length of unaligned region
    #[serde(default)]
    pub minlength: i64,
    /// Defaults to 100000 maximum length of unaligned region
    #[serde(default = "default_max_length")]
    pub maxlength: i64,
}
fn default_max_length() -> i64 { 100000 }

impl XmlNode for LoopConstraint {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("LoopConstraint")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut loop_constraint = LoopConstraint::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        QName(b"minlength") => {
                            loop_constraint.minlength = read_int(reader)?;
                        }
                        QName(b"maxlength") => {
                            loop_constraint.maxlength = read_int(reader)?;
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"LoopConstraint") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(loop_constraint)
    }
}

impl XmlVecNode for LoopConstraint {}

/// CoreDef structure
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CoreDef {
    /// number of core elements/blocks
    pub nblocks: i64,
    /// nblocks locations
    pub blocks: Vec<CoreBlock>,
    /// (nblocks+1) constraints
    pub loops: Vec<LoopConstraint>,
    /// is it a discontinuous domain
    pub is_discontinuous: Option<bool>,
    /// positions of long insertions
    pub insertions: Option<Vec<i64>>,
}

impl XmlNode for CoreDef {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("CoreDef")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut core_def = CoreDef::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let end = e.to_end();
                    match name {
                        QName(b"nblocks") => {
                            core_def.nblocks = read_int(reader)?;
                        }
                        QName(b"blocks") => {
                            core_def.blocks = read_vec_node(reader, end);
                        }
                        QName(b"loops") => {
                            core_def.loops = read_vec_node(reader, end);
                        }
                        QName(b"is_discontinuous") => {
                            core_def.is_discontinuous = read_bool_attribute(&e);
                        }
                        QName(b"insertions") => {
                            core_def.insertions = Some(read_vec_int_unchecked(reader,&end));
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"CoreDef") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(core_def)
    }
}

impl XmlVecNode for CoreDef {}

/// SiteAnnot structure
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SiteAnnot {
    /// location of the annotation
    pub start_position: i64,
    /// start and stop position in the PSSM
    pub stop_position: i64,
    /// holds description or names, that can be used for labels in visualization
    pub description: Option<String>,
    /// type of the annotated feature,similarly to Align-annot in NCBI-Cdd
    pub r#type: Option<i64>,
    /// additional names for the annotation
    pub aliases: Option<Vec<String>>,
    /// motif to validate mapping of sites
    pub motif: Option<String>,
    /// 0 for validation
    /// 1 for motif in seqloc
    /// 2 for multiple motifs in seqloc
    pub motif_use: Option<i64>, 
}

impl XmlNode for SiteAnnot {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("SiteAnnot")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut site_annot = SiteAnnot::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let end = e.to_end();
                    match e.name() {
                        QName(b"start_position") => {
                            site_annot.start_position = read_int(reader)?;
                        }
                        QName(b"stop_position") => {
                            site_annot.stop_position = read_int(reader)?;
                        }
                        QName(b"description") => {
                            site_annot.description = Some(read_string(reader)?);
                        }
                        QName(b"type") => {
                            site_annot.r#type = Some(read_int(reader)?);
                        }
                        QName(b"aliases") => {
                            site_annot.aliases = Some(read_vec_str_unchecked(reader,&end));
                        }
                        QName(b"motif") => {
                            site_annot.motif = Some(read_string(reader)?);
                        }
                        QName(b"motif_use") => {
                            site_annot.motif_use = Some(read_int(reader)?);
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"SiteAnnot") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(site_annot)
    }
}

impl XmlVecNode for SiteAnnot {}

/// 
/// ## PSI-BLAST, formatrpsdb, RPS-BLAST workflow:
///
/// Two possible inputs to PSI-BLAST and formatrpsdb:
/// 1) PssmWithParams where pssm field contains intermediate PSSM data (matrix
///    of frequency ratios)
/// 2) PssmWithParams where pssm field contains final PSSM data (matrix of
///    scores and statistical parameters) - such as written by cddumper
///
/// In case 1, PSI-BLAST's PSSM engine is invoked to create the PSSM and perform
/// the PSI-BLAST search or build the PSSM to then build the RPS-BLAST database.
/// In case 2, PSI-BLAST's PSSM engine is not invoked and the matrix of scores
/// statistical parameters are used to perform the search in PSI-BLAST and the
/// same data and the data in PssmWithParams::params::rpsdbparams is used to
/// build the PSSM and ultimately the RPS-BLAST database
///
/// ```text
///                 reads    ++++++++++++++ writes
/// PssmWithParams  ====>    + PSI-BLAST  + =====> PssmWithParams
///                          ++++++++++++++             |  ^
///         ^                                           |  |
///         |                                           |  |
///         +===========================================+  |
///                                                     |  |
///         +===========================================+  |
///         |                                              |
/// reads   |                                              |
///         v                                              |
///  +++++++++++++++ writes +++++++++++++++++++++++        |
///  | formatrpsdb | =====> | RPS-BLAST databases |        |
///  +++++++++++++++        +++++++++++++++++++++++        |
///                                   ^                    |
///                                   |                    |
///                                   | reads              |
///                             +++++++++++++              |
///                             | RPS-BLAST |              |
///                             +++++++++++++              |
///                                                        |
///       reads  ++++++++++++               writes         |
///  Cdd ======> | cddumper | =============================+
///              ++++++++++++
/// ```
///

/// SiteAnnotSet is a collection of SiteAnnot
pub type SiteAnnotSet = Vec<SiteAnnot>;

/// PssmFinalData structure
/// Contains the PSSM's scores and its associated statistical parameters.
/// Dimensions and order in which scores are stored must be the same as that
/// specified in Pssm::numRows, Pssm::numColumns, and Pssm::byrow
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PssmFinalData {
    /// PSSM's scores
    pub scores: Vec<i64>,
    /// Karlin & Altschul parameter produced during the PSSM's calculation
    pub lambda: f64,
    /// Karlin & Altschul parameter produced during the PSSM's calculation
    pub kappa: f64,
    /// Karlin & Altschul parameter produced during the PSSM's calculation
    pub h: f64,

    /// scaling factor used to obtain more precision when building the PSSM.
    /// (i.e.: scores are scaled by this value). By default, PSI-BLAST's PSSM
    /// engine generates PSSMs which are not scaled-up, however, if PSI-BLAST is
    /// given a PSSM which contains a scaled-up PSSM (indicated by having a
    /// scalingFactor greater than 1), then it will scale down the PSSM to
    /// perform the initial stages of the search with it.
    /// N.B.: When building RPS-BLAST databases, if formatrpsdb is provided
    /// scaled-up PSSMs, it will ensure that all PSSMs used to build the
    /// RPS-BLAST database are scaled by the same factor (otherwise, RPS-BLAST
    /// will silently produce incorrect results).

    #[serde(default)]
    pub scaling_factor: i64, // Defaults to 1
    /// Karlin & Altschul parameter produced during the PSSM's calculation
    pub lambda_ungapped: Option<f64>,
    /// Karlin & Altschul parameter produced during the PSSM's calculation
    pub kappa_ungapped: Option<f64>,
    /// Karlin & Altschul parameter produced during the PSSM's calculation
    pub h_ungapped: Option<f64>,
    /// Word score threshold
    pub word_score_threshold: Option<f64>,
}

impl XmlNode for PssmFinalData {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("PssmFinalData")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut pssm_final_data = PssmFinalData::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let end = e.to_end();
                    match name {
                        QName(b"scores") => {
                            pssm_final_data.scores = read_vec_int_unchecked(reader, &end);
                        }
                        QName(b"lambda") => {
                            pssm_final_data.lambda = read_double(reader)?;
                        }
                        QName(b"kappa") => {
                            pssm_final_data.kappa = read_double(reader)?;
                        }
                        QName(b"h") => {
                            pssm_final_data.h = read_double(reader)?;
                        }
                        QName(b"scaling_factor") => {
                            pssm_final_data.scaling_factor = read_int(reader)?;
                        }
                        QName(b"lambda_ungapped") => {
                            pssm_final_data.lambda_ungapped = read_double(reader);
                        }
                        QName(b"kappa_ungapped") => {
                            pssm_final_data.kappa_ungapped = read_double(reader);
                        }
                        QName(b"h_ungapped") => {
                            pssm_final_data.h_ungapped = read_double(reader);
                        }
                        QName(b"word_score_threshold") => {
                            pssm_final_data.word_score_threshold = read_double(reader);
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"PssmFinalData") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(pssm_final_data)
    }
}
/// Contains the PSSM's intermediate data used to create the PSSM's scores
/// and statistical parameters. Dimensions and order in which scores are
/// stored must be the same as that specified in Pssm::numRows,
/// Pssm::numColumns, and Pssm::byrow

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PssmIntermediateData {
    /// observed residue frequencies (or counts) per position of the PSSM 
    /// (prior to application of pseudocounts)
    pub res_freqs_per_pos: Option<Vec<i64>>,
    /// Weighted observed residue frequencies per position of the PSSM.
    /// (N.B.: each position's weights should add up to 1.0).
    /// This field corresponds to f_i (f sub i) in equation 2 of 
    /// Nucleic Acids Res. 2001 Jul 15;29(14):2994-3005.
    /// NOTE: this is needed for diagnostics information only (i.e.:
    /// -out_ascii_pssm option in psiblast)
    pub weighted_res_freqs_per_pos: Option<Vec<f64>>,
    /// PSSM's frequency ratios
    pub freq_ratios: Vec<f64>,
    /// Information content per position of the PSSM
    /// NOTE: this is needed for diagnostics information only (i.e.:
    /// -out_ascii_pssm option in psiblast)
    pub information_content: Option<Vec<f64>>,
    /// Relative weight for columns of the PSSM without gaps to pseudocounts
    /// NOTE: this is needed for diagnostics information only (i.e.:
    /// -out_ascii_pssm option in psiblast)
    pub gapless_column_weights: Option<Vec<f64>>,
    /// Used in sequence weights computation
    /// NOTE: this is needed for diagnostics information only (i.e.:
    /// -out_ascii_pssm option in psiblast)
    pub sigma: Option<Vec<f64>>,
    /// Length of the aligned regions per position of the query sequence
    /// NOTE: this is needed for diagnostics information only (i.e.:
    /// -out_ascii_pssm option in psiblast)
    pub interval_sizes: Option<Vec<i64>>,
    /// Number of matching sequences per position of the PSSM (including the query)
    /// NOTE: this is needed for diagnostics information only (i.e.:
    /// -out_ascii_pssm option in psiblast)
    pub num_matching_seqs: Option<Vec<i64>>,
    /// Number of independent observations per position of the PSSM
    /// NOTE: this is needed for building CDD database for DELTA-BLAST
    pub num_indept_obsr: Option<Vec<f64>>,
}

impl XmlNode for PssmIntermediateData {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("PssmIntermediateData")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut pssm_intermediate_data = PssmIntermediateData::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let end = e.to_end();
                    match name {
                        QName(b"res_freqs_per_pos") => {
                            pssm_intermediate_data.res_freqs_per_pos = Some(read_vec_int_unchecked(reader, &end));
                        }
                        QName(b"weighted_res_freqs_per_pos") => {
                            pssm_intermediate_data.weighted_res_freqs_per_pos = Some(read_vec_double_unchecked(reader,&end));
                        }
                        QName(b"freq_ratios") => {
                            pssm_intermediate_data.freq_ratios = read_vec_double_unchecked(reader, &end);
                        }
                        QName(b"information_content") => {
                            pssm_intermediate_data.information_content = Some(read_vec_double_unchecked(reader, &end));
                        }
                        QName(b"gapless_column_weights") => {
                            pssm_intermediate_data.gapless_column_weights = Some(read_vec_double_unchecked(reader,&end));
                        }
                        QName(b"sigma") => {
                            pssm_intermediate_data.sigma = Some(read_vec_double_unchecked(reader,&end));
                        }
                        QName(b"interval_sizes") => {
                            pssm_intermediate_data.interval_sizes = Some(read_vec_int_unchecked(reader, &end));
                        }
                        QName(b"num_matching_seqs") => {
                            pssm_intermediate_data.num_matching_seqs = Some(read_vec_int_unchecked(reader, &end));
                        }
                        QName(b"num_indept_obsr") => {
                            pssm_intermediate_data.num_indept_obsr = Some(read_vec_double_unchecked(reader, &end));
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"PssmIntermediateData") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(pssm_intermediate_data)
    }
}

impl XmlVecNode for PssmIntermediateData {}

/// Pssm structure
/// Position-specific scoring matrix
/// --
/// Column indices on the PSSM refer to the positions corresponding to the
/// query/master sequence, i.e. the number of columns (N) is the same
/// as the length of the query/master sequence.
/// Row indices refer to individual amino acid types, i.e. the number of
/// rows (M) is the same as the number of different residues in the
/// alphabet we use. Consequently, row labels are amino acid identifiers.
/// --
/// PSSMs are stored as linear arrays of integers. By default, we store
/// them column-by-column, M values for the first column followed by M
/// values for the second column, and so on. In order to provide
/// flexibility for external applications, the boolean field "byrow" is
/// provided to specify the storage order.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Pssm {
    /// Is the this a protein or nucleotide scoring matrix?
    #[serde(default = "default_true")]
    pub is_protein: bool,
    /// PSSM identifier
    pub identifier: Option<String>,
    /// The dimensions of the matrix are returned so the client can
    /// verify that all data was received.
    pub num_rows: i64,
    pub num_columns: i64,
    /// row-labels is given to note the order of residue types so that it can
    /// be cross-checked between applications.
    /// If this field is not given, the matrix values are presented in 
    /// order of the alphabet ncbistdaa is used for protein, ncbi4na for nucl.
    /// for proteins the values returned correspond to 
    /// (-,-), (-,A), (-,B), (-,C) ... (A,-), (A,A), (A,B), (A,C) ...
    pub row_labels: Option<Vec<String>>,
    /// are matrices stored row by row?
    #[serde(default)]
    pub by_row: bool,
    /// PSSM representative sequence (master)
    pub query: Option<String>, // Seq-entry would be replaced with a proper type

    /// both intermediateData and finalData can be provided, but at least one of
    /// them must be provided.
    /// N.B.: by default PSI-BLAST will return the PSSM in its PssmIntermediateData
    /// representation.

    /// Intermediate or final data for the PSSM
    pub intermediate_data: Option<PssmIntermediateData>,
    /// Final representation for the PSSM
    pub final_data: Option<PssmFinalData>,
}

impl XmlNode for Pssm {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Pssm")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut pssm = Pssm::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let end = e.to_end();
                    match name {
                        QName(b"is_protein") => {
                            pssm.is_protein = read_bool_attribute(&e)?;
                        }
                        QName(b"identifier") => {
                            pssm.identifier = Some(read_string(reader)?);
                        }
                        QName(b"num_rows") => {
                            pssm.num_rows = read_int(reader)?;
                        }
                        QName(b"num_columns") => {
                            pssm.num_columns = read_int(reader)?;
                        }
                        QName(b"row_labels") => {
                            pssm.row_labels = Some(read_vec_str_unchecked(reader, &end));
                        }
                        QName(b"by_row") => {
                            pssm.by_row = read_bool_attribute(&e)?;
                        }
                        QName(b"query") => {
                            pssm.query = Some(read_string(reader)?);
                        }
                        QName(b"intermediate_data") => {
                            pssm.intermediate_data = Some(PssmIntermediateData::from_reader(reader)?);
                        }
                        QName(b"final_data") => {
                            pssm.final_data = Some(PssmFinalData::from_reader(reader)?);
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"Pssm") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(pssm)
    }
}


fn default_true() -> bool { true }

/// FormatRpsDbParameters structure
/// This structure is used to create the RPS-BLAST database auxiliary file
/// (*.aux) and it contains parameters set at creation time of the PSSM.
/// Also, the matrixName field is used by formatrpsdb to build a PSSM from
/// a Pssm structure which only contains PssmIntermediateData.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FormatRpsDbParameters {
    /// name of the underlying score matrix whose frequency ratios were
    /// used in PSSM construction (e.g.: BLOSUM62)
    pub matrix_name: String,
    /// gap opening penalty corresponding to the matrix above
    pub gap_open: Option<i64>,
    /// gap extension penalty corresponding to the matrix above
    pub gap_extend: Option<i64>,
}

impl XmlNode for FormatRpsDbParameters {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("FormatRpsDbParameters")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut params = FormatRpsDbParameters::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        QName(b"matrix_name") => {
                            params.matrix_name = read_string(reader)?;
                        }
                        QName(b"gap_open") => {
                            params.gap_open = Some(read_int(reader)?);
                        }
                        QName(b"gap_extend") => {
                            params.gap_extend = Some(read_int(reader)?);
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"FormatRpsDbParameters") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }
        Some(params)
    }
}

/// PssmParameters structure
/// Populated by PSSM engine of PSI-BLAST, original source for these values
/// are the PSI-BLAST options specified using the BLAST options API
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PssmParameters {
    /// pseudocount constant used for PSSM. This field corresponds to beta in 
    /// equation 2 of Nucleic Acids Res. 2001 Jul 15;29(14):2994-3005.
    pub pseudocount: Option<i64>,
    /// data needed by formatrpsdb to create RPS-BLAST databases. matrixName is
    /// populated by PSI-BLAST
    pub rpsdbparams: Option<FormatRpsDbParameters>,
    /// alignment constraints needed by sequence-structure threader
    /// and other global or local block-alignment algorithms
    pub constraints: Option<CoreDef>,
    /// bit score threshold for specific conserved domain hits
    pub bit_score_thresh: Option<f64>,
    /// bit score threshold for reporting any conserved domain hits
    pub bit_score_reporting_thresh: Option<f64>,
    /// conserved functional sites with annotations
    pub annotated_sites: Option<SiteAnnotSet>,
}

impl XmlNode for PssmParameters {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("PssmParameters")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut params = PssmParameters::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let end = e.to_end();
                    match name.as_ref() {
                        b"pseudocount" => {
                            params.pseudocount = Some(read_int(reader)?);
                        }
                        b"rpsdbparams" => {
                            params.rpsdbparams = Some(FormatRpsDbParameters::from_reader(reader)?);
                        }
                        b"constraints" => {
                            params.constraints = Some(CoreDef::from_reader(reader)?);
                        }
                        b"bit_score_thresh" => {
                            params.bit_score_thresh = read_double(reader);
                        }
                        b"bit_score_reporting_thresh" => {
                            params.bit_score_reporting_thresh = read_double(reader);
                        }
                        b"annotated_sites" => {
                            params.annotated_sites = Some(read_vec_node(reader,end));
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"PssmParameters") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(params)
    }
}

/// PssmWithParameters structure
/// Envelope containing PSSM and the parameters used to create it.
/// Provided for use in PSI-BLAST, formatrpsdb, and for the structure group.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PssmWithParameters {
    /// This field is applicable to PSI-BLAST and formatrpsdb.
    /// When both the intermediate and final PSSM data are provided in this
    /// field, the final data (matrix of scores and associated statistical
    /// parameters) takes precedence and that data is used for further
    /// processing. The rationale for this is that the PSSM's scores and
    /// statistical parameters might have been calculated by other applications
    /// and it might not be possible to recreate it by using PSI-BLAST's PSSM
    /// engine.
    pub pssm: Pssm,
    /// This field's rpsdbparams is used to specify the values of options
    /// for processing by formatrpsdb. If these are not set, the command
    /// line defaults of formatrpsdb are applied. This field is used
    /// by PSI-BLAST to verify that the underlying scorem matrix used to BUILD
    /// the PSSM is the same as the one being specified through the BLAST
    /// Options API. If this field is omitted, no verification will be
    /// performed, so be careful to keep track of what matrix was used to build
    /// the PSSM or else the results produced by PSI-BLAST will be unreliable.
    pub params: Option<PssmParameters>,
}

impl XmlNode for PssmWithParameters {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("PssmWithParameters")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut pssm_with_params = PssmWithParameters::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        QName(b"Pssm") => {
                            pssm_with_params.pssm = Pssm::from_reader(reader)?;
                        }
                        QName(b"params") => {
                            pssm_with_params.params = Some(PssmParameters::from_reader(reader)?);
                        }
                        _ => (),
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name() == QName(b"PssmWithParameters") {
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(_) => return None,
                _ => (),
            }
        }

        Some(pssm_with_params)
    }
}

impl XmlVecNode for PssmWithParameters {}

