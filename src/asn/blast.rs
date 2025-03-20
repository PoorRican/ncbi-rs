//! This file is generated from blast.asn, maintaining original comments and order.
//!
//! Please note:
//! - External types like `BioSeq`, `BioSeqSet`, `SeqLoc`, `SeqAlign`, etc. are assumed
//!   to be defined in other modules (as per the `ncbi-rs` project structure).
//! - This file provides the data structures and their Rust doc comments derived from the ASN.1 comments.
//! - Implementations for XML parsing (XmlNode, XmlVecNode) and related logic are not included here.
//! - The order and comments follow the `blast.asn` file as closely as possible.

use crate::seqloc::{SeqLoc, SeqInterval, SeqId};
use crate::seq::{BioSeq, SeqData};
use crate::seqset::{BioSeqSet};
use crate::seqalign::{SeqAlign, SeqAlignSet};
use crate::scoremat::{PssmWithParameters};

use enum_primitive::FromPrimitive;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::parsing::{read_bool_attribute, read_int, read_node, read_string, read_vec_bool_unchecked, read_vec_double_unchecked, read_vec_int_unchecked, read_vec_node, read_vec_str_unchecked, UnexpectedTags};
use crate::parsing::{XmlNode, XmlVecNode};

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Blast4-archive
///
/// An archive format for results. The results can be reformatted from
/// this format as well.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blast4Archive {
    /// Query and options
    pub request: Blast4Request,
    /// Results of search
    pub results: Blast4GetSearchResultsReply,
    /// Messages (errors) 
    pub messages: Option<Vec<Blast4Error>>,
}

/// Blast4-request
///
/// Represents a BLAST request.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4Request {
    /// Client identifier (email, org name, program/script name, or other form of contact)
    pub ident: Option<String>,
    /// Payload of the request
    pub body: Blast4RequestBody,
}

impl XmlNode for Blast4Request {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-request")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Self { ident: None, body: Blast4RequestBody::default() };
        let ident_element = BytesStart::new("ident");
        let body_element = BytesStart::new("body");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == ident_element.name() {
                        request.ident = Some(read_string(reader).unwrap());
                    } else if name == body_element.name() {
                        request.body = read_node(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}

#[allow(non_camel_case_types)]
/// Blast4-request-body
///
/// Payload of the request.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")] // Tagging approach for clarity
pub enum Blast4RequestBody {
    /// finish-params
    //#[default]
    finishparams(Blast4FinishParamsRequest),
    /// get-databases
    getdatabases,
    /// get-matrices
    getmatrices,
    /// get-parameters
    getparameters,
    /// get-paramsets
    getparamsets,
    /// get-programs
    getprograms,
    /// get-search-results
    getsearchresults(Blast4GetSearchResultsRequest),
    /// get-sequences
    getsequences(Blast4GetSequencesRequest),
    /// queue-search
    queuesearch(Blast4QueueSearchRequest),
    /// get-request-info
    getrequestinfo(Blast4GetRequestInfoRequest),
    /// get-sequence-parts
    getsequenceparts(Blast4GetSeqPartsRequest),
    /// get-windowmasked-taxids
    getwindowmaskedtaxids,
    /// get-protocol-info
    getprotocolinfo(Blast4GetProtocolInfoRequest),
    /// get-search-info
    getsearchinfo(Blast4GetSearchInfoRequest),
    /// get-databases-ex
    getdatabasesex(Blast4GetDatabasesExRequest),
}

impl Default for Blast4RequestBody {
    fn default() -> Self {
        Self::finishparams(Blast4FinishParamsRequest::default()) // Ensure Blast4FinishParamsRequest also implements Default
    }
}

impl XmlNode for Blast4RequestBody {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-request-body") // Ensure this matches the actual XML tag
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    return match name.as_ref() {
                        b"finish-params" => Some(Blast4RequestBody::finishparams(read_node(reader)?)),
                        b"get-databases" => Some(Blast4RequestBody::getdatabases),
                        b"get-matrices" => Some(Blast4RequestBody::getmatrices),
                        b"get-parameters" => Some(Blast4RequestBody::getparameters),
                        b"get-paramsets" => Some(Blast4RequestBody::getparamsets),
                        b"get-programs" => Some(Blast4RequestBody::getprograms),
                        b"get-search-results" => Some(Blast4RequestBody::getsearchresults(read_node(reader)?)),
                        b"get-sequences" => Some(Blast4RequestBody::getsequences(read_node(reader)?)),
                        b"queue-search" => Some(Blast4RequestBody::queuesearch(read_node(reader)?)),
                        b"get-request-info" => Some(Blast4RequestBody::getrequestinfo(read_node(reader)?)),
                        b"get-sequence-parts" => Some(Blast4RequestBody::getsequenceparts(read_node(reader)?)),
                        b"get-windowmasked-taxids" => Some(Blast4RequestBody::getwindowmaskedtaxids),
                        b"get-protocol-info" => Some(Blast4RequestBody::getprotocolinfo(read_node(reader)?)),
                        b"get-search-info" => Some(Blast4RequestBody::getsearchinfo(read_node(reader)?)),
                        b"get-databases-ex" => Some(Blast4RequestBody::getdatabasesex(read_node(reader)?)),
                        _ => {
                            None
                        }
                    };
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return None;
                }
                _ => (),
            }
        }
    }
}


/// blast4-get-databases-ex-request
///
/// for extended database retrieval requests.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetDatabasesExRequest {
    /// optional parameters
    pub params: Option<Blast4Parameters>,
}

/// blast4-finish-params-request
///
/// finalize parameters for a given request.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4FinishParamsRequest {
    /// program name
    pub program: String,
    /// service name
    pub service: String,
    /// task description
    pub paramset: Option<String>,
    /// parameters
    pub params: Option<Blast4Parameters>,
}

impl XmlNode for Blast4FinishParamsRequest {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-finish-params-request")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Blast4FinishParamsRequest::default();
        let program_element = BytesStart::new("program");
        let service_element = BytesStart::new("service");
        let paramset_element = BytesStart::new("paramset");
        let params_element = BytesStart::new("params");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == program_element.name() {
                        request.program = read_string(reader).unwrap();
                    } else if name == service_element.name() {
                        request.service = read_string(reader).unwrap();
                    } else if name == paramset_element.name() {
                        request.paramset = Some(read_string(reader).unwrap());
                    } else if name == params_element.name() {
                        request.params = Some(read_vec_node(reader,e.to_end()));
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}


enum_from_primitive! {
    /// blast4-result-types
    ///
    /// specifies the types of results desired.
    #[allow(non_camel_case_types)]
    #[derive(Clone, Serialize_repr, Deserialize_repr, Debug, Default)]
    #[repr(u64)]
    pub enum Blast4ResultTypes {
        /// default (63)
        #[default]
        default = 63,
        /// alignments (1)
        alignments = 1,
        /// phi-alignments (2)
        phialignments = 2,
        /// masks (4)
        masks = 4,
        /// ka-blocks (8)
        kablocks = 8,
        /// search-stats (16)
        searchstats = 16,
        /// pssm (32)
        pssm = 32,
        /// simple-results (64)
        simpleresults = 64,
    }
}

/// blast4-get-search-results-request
///
/// request to retrieve search results.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSearchResultsRequest {
    /// the request id of the blast search
    pub request_id: String,
    /// logical or of blast4-result-types, assumes default if absent
    pub result_types: Option<u64>,
}

/// blast4-queries
///
/// represents queries used in a blast search.
#[allow(non_camel_case_types)]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Blast4Queries {
    /// pssm
    pssm(PssmWithParameters),
    /// seq-loc-list
    seqloclist(Vec<SeqLoc>),
    /// bioseq-set
    bioseqset(BioSeqSet),
}

impl Default for Blast4Queries {
    fn default() -> Self {
        Self::pssm(PssmWithParameters::default()) // Ensure PssmWithParameters also implements Default
    }
}

/// blast4-queue-search-request
///
/// used to initiate a search queueing.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4QueueSearchRequest {
    pub program: String,
    pub service: String,
    pub queries: Blast4Queries,
    pub subject: Blast4Subject,
    /// task description
    pub paramset: Option<String>,
    /// algorithm-options
    pub algorithm_options: Option<Blast4Parameters>,
    /// program-options
    pub program_options: Option<Blast4Parameters>,
    /// format-options
    pub format_options: Option<Blast4Parameters>,
}

/// blast4-get-search-status-request
///
/// request to retrieve the status of a given search.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSearchStatusRequest {
    pub request_id: String,
}

/// blast4-get-search-status-reply
///
/// reply to retrieve the status of a given search.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSearchStatusReply {
    pub status: String,
}

/// blast4-get-request-info-request
///
/// fetch information about the search request.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetRequestInfoRequest {
    pub request_id: String,
}

/// blast4-get-request-info-reply
///
/// reply with search request information.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetRequestInfoReply {
    pub database: Blast4Database,
    pub program: String,
    pub service: String,
    pub created_by: String,
    pub queries: Blast4Queries,
    pub algorithm_options: Blast4Parameters,
    pub program_options: Blast4Parameters,
    pub format_options: Option<Blast4Parameters>,
    pub subjects: Option<Blast4Subject>,
}

/// blast4-get-search-strategy-request
///
/// fetch the search strategy.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSearchStrategyRequest {
    pub request_id: String,
}

/// blast4-get-search-strategy-reply
///
/// return the search strategy (i.e., a blast4-request)
pub type Blast4GetSearchStrategyReply = Blast4Request;

/// blast4-get-sequences-request
///
/// fetch sequence data from a blast database.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSequencesRequest {
    pub database: Blast4Database,
    pub seq_ids: Vec<SeqId>,
    #[serde(default)]
    pub skip_seq_data: bool,
    pub target_only: Option<bool>,
}

impl XmlNode for Blast4GetSequencesRequest {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-sequences-request")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Blast4GetSequencesRequest::default();
        let database_element = BytesStart::new("database");
        let seq_ids_element = BytesStart::new("seq_ids");
        let skip_seq_data_element = BytesStart::new("skip_seq_data");
        let target_only_element = BytesStart::new("target_only");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == database_element.name() {
                        request.database = read_node(reader).unwrap();
                    } else if name == seq_ids_element.name() {
                        request.seq_ids = read_vec_node(reader, seq_ids_element.to_end());
                    } else if name == skip_seq_data_element.name() {
                        request.skip_seq_data = read_bool_attribute(&e)?;
                    } else if name == target_only_element.name() {
                        request.target_only = read_bool_attribute(&e);
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}

/// blast4-get-seq-parts-request
///
/// fetch parts of sequences from a blast database.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSeqPartsRequest {
    pub database: Blast4Database,
    pub seq_locations: Vec<SeqInterval>,
}

impl XmlNode for Blast4GetSeqPartsRequest {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-seq-parts-request")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Blast4GetSeqPartsRequest::default();
        let database_element = BytesStart::new("database");
        let seq_locations_element = BytesStart::new("seq_locations");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == database_element.name() {
                        request.database = read_node(reader).unwrap();
                    } else if name == seq_locations_element.name() {
                        request.seq_locations = read_vec_node(reader, seq_locations_element.to_end());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}

/// blast4-get-protocol-info-request
///
/// for version and checking availability of methods.
pub type Blast4GetProtocolInfoRequest = Blast4Parameters;

/// blast4-get-search-info-request
///
/// variose search information.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSearchInfoRequest {
    pub request_id: String,
    pub info: Option<Blast4Parameters>,
}

/// blast4-reply
///
/// replies to a blast request.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4Reply {
    pub errors: Option<Vec<Blast4Error>>,
    pub body: Blast4ReplyBody,
}

#[allow(non_camel_case_types)]
/// Blast4-reply-body
///
/// Body of the BLAST reply.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum Blast4ReplyBody {
    //#[default]
    FinishParams(Blast4Parameters),
    GetDatabases(Blast4GetDatabasesReply),
    GetMatrices(Blast4GetMatricesReply),
    GetParameters(Blast4GetParametersReply),
    GetParamsets(Blast4GetParamsetsReply),
    GetPrograms(Blast4GetProgramsReply),
    GetSearchResults(Blast4GetSearchResultsReply),
    GetSequences(Blast4GetSequencesReply),
    QueueSearch(Blast4QueueSearchReply),
    GetQueries(Blast4GetQueriesReply),
    GetRequestInfo(Blast4GetRequestInfoReply),
    GetSequenceParts(Blast4GetSeqPartsReply),
    GetWindowmaskedTaxids(Blast4GetWindowmaskedTaxidsReply),
    GetProtocolInfo(Blast4GetProtocolInfoReply),
    GetSearchInfo(Blast4GetSearchInfoReply),
    GetDatabasesEx(Blast4GetDatabasesExReply),
}

impl Default for Blast4ReplyBody {
    fn default() -> Self {
        Self::FinishParams(Blast4Parameters::default()) // Ensure Blast4Parameters also implements Default
    }
}


/// Blast4-finish-params-reply
///
/// This is the same as Blast4-parameters
pub type Blast4FinishParamsReply = Blast4Parameters;

/// Blast4-get-windowmasked-taxids-reply
///
/// Returns a list of integers.
pub type Blast4GetWindowmaskedTaxidsReply = Vec<i64>;

/// Blast4-get-databases-reply
///
/// Returns database info.
pub type Blast4GetDatabasesReply = Vec<Blast4DatabaseInfo>;

/// Blast4-get-databases-ex-reply
///
/// Extended database info.
pub type Blast4GetDatabasesExReply = Vec<Blast4DatabaseInfo>;

/// Blast4-get-matrices-reply
///
/// Returns a list of matrix IDs.
pub type Blast4GetMatricesReply = Vec<Blast4MatrixId>;

/// Blast4-get-parameters-reply
///
/// Returns a list of parameter info.
pub type Blast4GetParametersReply = Vec<Blast4ParameterInfo>;

/// Blast4-get-paramsets-reply
///
/// Returns a list of task info.
pub type Blast4GetParamsetsReply = Vec<Blast4TaskInfo>;

/// Blast4-get-programs-reply
///
/// Returns a list of program info.
pub type Blast4GetProgramsReply = Vec<Blast4ProgramInfo>;

/// Blast4-get-search-results-reply
///
/// Returns the results of a search.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSearchResultsReply {
    pub alignments: Option<SeqAlignSet>,
    pub phi_alignments: Option<Blast4PhiAlignments>,
    pub masks: Option<Vec<Blast4Mask>>,
    pub ka_blocks: Option<Vec<Blast4KaBlock>>,
    pub search_stats: Option<Vec<String>>,
    pub pssm: Option<PssmWithParameters>,
    pub simple_results: Option<Blast4SimpleResults>,
}

/// Blast4-get-sequences-reply
///
/// Returns a list of BioSeq.
pub type Blast4GetSequencesReply = Vec<BioSeq>;

/// Blast4-seq-part-data
///
/// Bundles Seq-ids and sequence data to fulfill requests.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blast4SeqPartData {
    pub id: SeqId,
    pub data: SeqData,
}

impl XmlNode for Blast4SeqPartData {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-seq-part-data")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut data = Blast4SeqPartData {
            id: SeqId::default(),
            data: SeqData::default(),
        };
        let id_element = BytesStart::new("id");
        let data_element = BytesStart::new("data");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == id_element.name() {
                        data.id = read_node(reader).unwrap();
                    } else if name == data_element.name() {
                        data.data = read_node(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(data);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4SeqPartData {}



/// Blast4-get-seq-parts-reply
///
/// Returns parts of sequences.
pub type Blast4GetSeqPartsReply = Vec<Blast4SeqPartData>;

/// Blast4-queue-search-reply
///
/// Returns the request ID.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4QueueSearchReply {
    pub request_id: Option<String>,
}

/// Blast4-get-queries-reply
///
/// Returns queries.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetQueriesReply {
    pub queries: Blast4Queries,
}

/// Blast4-get-protocol-info-reply
///
/// For version and checking availability of methods (parameters).
pub type Blast4GetProtocolInfoReply = Blast4Parameters;

/// Blast4-get-search-info-reply
///
/// Returns information on a search.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4GetSearchInfoReply {
    pub request_id: String,
    pub info: Option<Blast4Parameters>,
}

impl XmlNode for Blast4GetSearchInfoReply {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-search-info-reply")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut reply = Blast4GetSearchInfoReply::default();
        let request_id_element = BytesStart::new("request_id");
        let info_element = BytesStart::new("info");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == request_id_element.name() {
                        reply.request_id = read_string(reader).unwrap();
                    } else if name == info_element.name() {
                        reply.info = Some(read_node(reader).unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(reply);
                    }
                }
                _ => (),
            }
        }
    }
}


/// Blast4-error
///
/// Represents errors or warnings.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4Error {
    /// This is an integer to allow flexibility.
    pub code: i64,
    /// Optional error message
    pub message: Option<String>,
}

impl XmlNode for Blast4Error {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-error")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut error = Blast4Error::default();
        let code_element = BytesStart::new("code");
        let message_element = BytesStart::new("message");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == code_element.name() {
                        error.code = read_string(reader).unwrap().parse().unwrap();
                    } else if name == message_element.name() {
                        error.message = Some(read_string(reader).unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(error);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4Error {}

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    /// Blast4-error-flags
    ///
    /// Enumeration of error flags.
    #[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
    #[repr(u16)]
    pub enum Blast4ErrorFlags {
        Warning = 1024,
        Error = 2048,
    }
}

/// Blast4-error-code
///
/// Defines values for use in Blast4-error::code.
#[allow(non_camel_case_types)]
pub mod blast4_error_code {
    pub const CONVERSION_WARNING: i64 = 1024;
    pub const INTERNAL_ERROR: i64 = 2048;
    pub const NOT_IMPLEMENTED: i64 = 2049;
    pub const NOT_ALLOWED: i64 = 2050;
    pub const BAD_REQUEST: i64 = 2051;
    pub const BAD_REQUEST_ID: i64 = 2052;
    pub const SEARCH_PENDING: i64 = 2053;
}

#[allow(non_camel_case_types)]
/// Blast4-cutoff
///
/// Choice of cutoff types (e-value or raw-score).
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum Blast4Cutoff {
    EValue(f64),
    RawScore(i64),
}

impl XmlNode for Blast4Cutoff {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-cutoff")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let forbidden_tags = [BytesStart::new("unknown-tag")];
        let forbidden = UnexpectedTags(&forbidden_tags);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    let cutoff = match name.as_ref() {
                        b"EValue" => Some(Blast4Cutoff::EValue(read_string(reader).unwrap().parse().unwrap())),
                        b"RawScore" => Some(Blast4Cutoff::RawScore(read_string(reader).unwrap().parse().unwrap())),
                        _ => {
                            forbidden.check(&name);
                            None
                        }
                    };

                    if cutoff.is_some() {
                        return cutoff;
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

impl XmlVecNode for Blast4Cutoff {}

/// Blast4-database
///
/// Specifies a BLAST database by name and type.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4Database {
    pub name: String,
    pub r#type: Blast4ResidueType,
}

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    /// Blast4-seqtech
    ///
    /// Derived from seq.asn, enumerating sequencing technology.
    #[derive(Clone, Serialize_repr, Deserialize_repr, Debug, Default)]
    #[repr(u16)]
    pub enum Blast4Seqtech {
        #[default]
        Unknown = 0,
        /// standard sequencing
        Standard = 1,
        /// Expressed Sequence Tag
        Est = 2,
        /// Sequence Tagged Site
        Sts = 3,
        /// one-pass genomic sequence
        Survey = 4,
        /// from genetic mapping techniques
        Genemap = 5,
        /// from physical mapping techniques
        Physmap = 6,
        /// derived from other data, not a primary entity
        Derived = 7,
        /// conceptual translation
        ConceptTrans = 8,
        // peptide was sequenced
        SeqPept = 9,
        /// concept transl. w/ partial pept. seq.
        Both = 10,
        /// sequenced peptide, ordered by overlap
        SeqPeptOverlap = 11,
        /// sequenced peptide, ordered by homology
        SeqPeptHomol = 12,
        /// conceptual transl. supplied by author
        ConceptTransA = 13,
        /// unordered High Throughput sequence contig
        Htgs1 = 14,
        /// ordered High Throughput sequence contig
        Htgs2 = 15,
        /// finished High Throughput sequence
        Htgs3 = 16,
        /// full length insert cDNA
        FliCdna = 17,
        /// single genomic reads for coordination
        Htgs0 = 18,
        /// high throughput cDNA
        Htc = 19,
        /// whole genome shotgun sequencing
        Wgs = 20,
        /// use Source.techexp
        Other = 255,
    }
}

//enum_from_primitive! {
    #[allow(non_camel_case_types)]
    /// Blast4-database-info
    ///
    /// Holds database metadata.
    #[derive(Clone, Serialize, Deserialize, Debug, Default)]
    pub struct Blast4DatabaseInfo {
        pub database: Blast4Database,
        pub description: String,
        pub last_updated: String,
        pub total_length: i64,  // BigInt in ASN.1, i64 in Rust
        pub num_sequences: i64, // BigInt in ASN.1, i64 in Rust
        pub seqtech: Blast4Seqtech,
        pub taxid: i64,
        pub extended: Option<Blast4Parameters>,
    }
//}

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    /// Blast4-frame-type
    ///
    /// Frame types for nucleotides and translations.
    #[derive(Clone, Serialize_repr, Deserialize_repr, Debug, Default)]
    #[repr(u8)]
    pub enum Blast4FrameType {
        #[default]
        Notset = 0,
        Plus1 = 1,
        Plus2 = 2,
        Plus3 = 3,
        Minus1 = 4,
        Minus2 = 5,
        Minus3 = 6,
    }
}

impl XmlNode for Blast4FrameType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-frame-type")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    forbidden.check(&e.name());
                }
                Event::Text(e) => {
                    let text = read_string(reader)?;
                    return match text.as_str() {
                        "Notset" => Some(Blast4FrameType::Notset),
                        "Plus1" => Some(Blast4FrameType::Plus1),
                        "Plus2" => Some(Blast4FrameType::Plus2),
                        "Plus3" => Some(Blast4FrameType::Plus3),
                        "Minus1" => Some(Blast4FrameType::Minus1),
                        "Minus2" => Some(Blast4FrameType::Minus2),
                        "Minus3" => Some(Blast4FrameType::Minus3),
                        _ => None, // Handle unknown text safely
                    };
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return None;
                }
                _ => (),
            }
        }
    }
}


/// Blast4-ka-block
///
/// Karlin & Altschul parameters.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4KaBlock {
    pub lambda: f64,
    pub k: f64,
    pub h: f64,
    pub gapped: bool,
}

impl XmlNode for Blast4KaBlock {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-ka-block")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut ka_block = Blast4KaBlock {
            lambda: 0.0,
            k: 0.0,
            h: 0.0,
            gapped: false,
        };
        let lambda_element = BytesStart::new("lambda");
        let k_element = BytesStart::new("k");
        let h_element = BytesStart::new("h");
        let gapped_element = BytesStart::new("gapped");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == lambda_element.name() {
                        ka_block.lambda = read_string(reader).unwrap().parse().unwrap();
                    } else if name == k_element.name() {
                        ka_block.k = read_string(reader).unwrap().parse().unwrap();
                    } else if name == h_element.name() {
                        ka_block.h = read_string(reader).unwrap().parse().unwrap();
                    } else if name == gapped_element.name() {
                        ka_block.gapped = read_bool_attribute(&e)?;
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(ka_block);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4KaBlock {}

/// Blast4-mask
///
/// Masking locations for a query's frame.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4Mask {
    pub locations: Vec<SeqLoc>,
    pub frame: Blast4FrameType,
}

impl XmlNode for Blast4Mask {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-mask")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut mask = Blast4Mask::default();
        let locations_element = BytesStart::new("locations");
        let frame_element = BytesStart::new("frame");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == locations_element.name() {
                        mask.locations = read_vec_node(reader, locations_element.to_end());
                    } else if name == frame_element.name() {
                        mask.frame = read_node(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(mask);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4Mask {}

/// Blast4-matrix-id
///
/// Identifies a scoring matrix by residue type and name.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4MatrixId {
    pub residue_type: Blast4ResidueType,
    pub name: String,
}

impl XmlNode for Blast4MatrixId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-matrix-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut matrix_id = Blast4MatrixId::default();
        let residue_type_element = BytesStart::new("residue_type");
        let name_element = BytesStart::new("name");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == residue_type_element.name() {
                        matrix_id.residue_type = read_node(reader).unwrap();
                    } else if name == name_element.name() {
                        matrix_id.name = read_string(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(matrix_id);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4MatrixId {}

/// Blast4-parameter
///
/// A named parameter with a value.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4Parameter {
    pub name: String,
    pub value: Blast4Value,
}

/// Blast4-parameter-info
///
/// Information about a parameter: name and type.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blast4ParameterInfo {
    pub name: String,
    pub r#type: String,
}

impl XmlNode for Blast4ParameterInfo {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-parameter-info")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut parameter_info = Blast4ParameterInfo {
            name: String::new(),
            r#type: String::new(),
        };
        let name_element = BytesStart::new("name");
        let type_element = BytesStart::new("type");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == name_element.name() {
                        parameter_info.name = read_string(reader).unwrap();
                    } else if name == type_element.name() {
                        parameter_info.r#type = read_string(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(parameter_info);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4ParameterInfo {}

/// Blast4-task-info
///
/// Human-readable description of a task.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blast4TaskInfo {
    /// Name of this task
    pub name: String,
    /// Description of the task
    pub documentation: String,
}

impl XmlNode for Blast4TaskInfo {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-task-info")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut task_info = Blast4TaskInfo {
            name: String::new(),
            documentation: String::new(),
        };
        let name_element = BytesStart::new("name");
        let documentation_element = BytesStart::new("documentation");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == name_element.name() {
                        task_info.name = read_string(reader).unwrap();
                    } else if name == documentation_element.name() {
                        task_info.documentation = read_string(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(task_info);
                    }
                }
                _ => (),
            }
        }
    }
}
impl XmlVecNode for Blast4TaskInfo {}

/// Blast4-program-info
///
/// Information about a program and its supported services.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blast4ProgramInfo {
    pub program: String,
    pub services: Vec<String>,
}

impl XmlNode for Blast4ProgramInfo {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-program-info")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut program_info = Blast4ProgramInfo {
            program: String::new(),
            services: Vec::new(),
        };
        let program_element = BytesStart::new("program");
        let services_element = BytesStart::new("services");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == program_element.name() {
                        program_info.program = read_string(reader).unwrap();
                    } else if name == services_element.name() {
                        let end = e.to_end();
                        program_info.services = read_vec_str_unchecked(reader,&end);
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(program_info);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4ProgramInfo {}

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    /// Blast4-residue-type
    ///
    /// Residue type for BLAST searches.
    #[derive(Clone, Serialize_repr, Deserialize_repr, Debug, Default)]
    #[repr(u8)]
    pub enum Blast4ResidueType {
        #[default]
        Unknown = 0,
        Protein = 1,
        Nucleotide = 2,
    }
}

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    /// Blast4-strand-type
    ///
    /// Strand specification.
    #[derive(Clone, Serialize_repr, Deserialize_repr, Debug, Default)]
    #[repr(u8)]
    pub enum Blast4StrandType {
        ForwardStrand = 1,
        ReverseStrand = 2,
        #[default]
        BothStrands = 3,
    }
}

impl XmlNode for Blast4StrandType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-strand-type")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    forbidden.check(&e.name());
                }
                Event::Text(_) => {
                    let text = read_string(reader)?;
                    return match text.as_str() {
                        "ForwardStrand" => Some(Blast4StrandType::ForwardStrand),
                        "ReverseStrand" => Some(Blast4StrandType::ReverseStrand),
                        "BothStrands" => Some(Blast4StrandType::BothStrands),
                        _ => None, // Handle unknown text safely
                    };
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    return None;
                }
                _ => (),
            }
        }
    }
}


impl XmlVecNode for Blast4StrandType {}

#[allow(non_camel_case_types)]
/// Blast4-subject
///
/// Subject sequences for a BLAST search.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "choice", content = "data")]
pub enum Blast4Subject {
    Database(String),
    Sequences(Vec<BioSeq>),
    SeqLocList(Vec<SeqLoc>),
}

impl Default for Blast4Subject {
    fn default() -> Self {
        Self::Sequences(Vec::new()) // Equivalent to `Vec<BioSeq>::default()`
    }
}

/// Blast4-parameters
///
/// A sequence of parameters.
pub type Blast4Parameters = Vec<Blast4Parameter>;

/// Blast4-phi-alignments
///
/// Used for PHI-BLAST alignments.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4PhiAlignments {
    pub num_alignments: i64,
    pub seq_locs: Vec<SeqLoc>,
}

impl XmlNode for Blast4PhiAlignments {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-phi-alignments")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut alignments = Blast4PhiAlignments::default();
        let num_alignments_element = BytesStart::new("num_alignments");
        let seq_locs_element = BytesStart::new("seq_locs");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == num_alignments_element.name() {
                        alignments.num_alignments = read_int(reader).unwrap();
                    } else if e.name() == seq_locs_element.name() {
                        alignments.seq_locs = read_vec_node(reader,seq_locs_element.to_end());
                    } else {
                        forbidden.check(&e.name());
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(alignments);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4PhiAlignments {}


#[allow(non_camel_case_types)]
/// Blast4-value
///
/// Represents the value of a parameter.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum Blast4Value {
    BigInteger(i64),
    Bioseq(BioSeq),
    Boolean(bool),
    Cutoff(Blast4Cutoff),
    Integer(i64),
    Matrix(PssmWithParameters),
    Real(f64),
    SeqAlign(SeqAlign),
    SeqId(SeqId),
    SeqLoc(SeqLoc),
    StrandType(Blast4StrandType),
    String(String),
    BigIntegerList(Vec<i64>),
    BioseqList(Vec<BioSeq>),
    BooleanList(Vec<bool>),
    CutoffList(Vec<Blast4Cutoff>),
    IntegerList(Vec<i64>),
    MatrixList(Vec<PssmWithParameters>),
    RealList(Vec<f64>),
    SeqAlignList(Vec<SeqAlign>),
    SeqIdList(Vec<SeqId>),
    SeqLocList(Vec<SeqLoc>),
    StrandTypeList(Vec<Blast4StrandType>),
    StringList(Vec<String>),
    BioSeqSet(BioSeqSet),
    SeqAlignSet(SeqAlignSet),
    QueryMask(Blast4Mask),
}

/// Not part of standard.
impl Default for Blast4Value {
    fn default() -> Self {
        Blast4Value::Integer(0)
    }
}

impl XmlNode for Blast4Value {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-value")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let forbidden_tags = [BytesStart::new("unknown-tag")];
        let forbidden = UnexpectedTags(&forbidden_tags);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    let end = e.to_end();

                    let value = match name.as_ref() {
                        b"BigInteger" => Some(Blast4Value::BigInteger(read_string(reader).unwrap().parse().unwrap())),
                        b"Bioseq" => Some(Blast4Value::Bioseq(read_node(reader).unwrap())),
                        b"Boolean" => Some(Blast4Value::Boolean(read_string(reader).unwrap().parse().unwrap())),
                        b"Cutoff" => Some(Blast4Value::Cutoff(read_node(reader).unwrap())),
                        b"Integer" => Some(Blast4Value::Integer(read_string(reader).unwrap().parse().unwrap())),
                        b"Matrix" => Some(Blast4Value::Matrix(read_node(reader).unwrap())),
                        b"Real" => Some(Blast4Value::Real(read_string(reader).unwrap().parse().unwrap())),
                        b"SeqAlign" => Some(Blast4Value::SeqAlign(read_node(reader).unwrap())),
                        b"SeqId" => Some(Blast4Value::SeqId(read_node(reader).unwrap())),
                        b"SeqLoc" => Some(Blast4Value::SeqLoc(read_node(reader).unwrap())),
                        b"StrandType" => Some(Blast4Value::StrandType(read_node(reader).unwrap())),
                        b"String" => Some(Blast4Value::String(read_string(reader).unwrap())),
                        b"BigIntegerList" => Some(Blast4Value::BigIntegerList(read_vec_int_unchecked(reader, &end))),
                        b"BioseqList" => Some(Blast4Value::BioseqList(read_vec_node(reader, e.to_end()))),
                        b"BooleanList" => Some(Blast4Value::BooleanList(read_vec_bool_unchecked(reader, &end))),
                        b"CutoffList" => Some(Blast4Value::CutoffList(read_vec_node(reader, e.to_end()))),
                        b"IntegerList" => Some(Blast4Value::IntegerList(read_vec_int_unchecked(reader, &end))),
                        b"MatrixList" => Some(Blast4Value::MatrixList(read_vec_node(reader, e.to_end()))),
                        b"RealList" => Some(Blast4Value::RealList(read_vec_double_unchecked(reader, &end))),
                        b"SeqAlignList" => Some(Blast4Value::SeqAlignList(read_vec_node(reader, e.to_end()))),
                        b"SeqIdList" => Some(Blast4Value::SeqIdList(read_vec_node(reader, e.to_end()))),
                        b"SeqLocList" => Some(Blast4Value::SeqLocList(read_vec_node(reader, e.to_end()))),
                        b"StrandTypeList" => Some(Blast4Value::StrandTypeList(read_vec_node(reader, e.to_end()))),
                        b"StringList" => Some(Blast4Value::StringList(read_vec_node(reader, e.to_end()))),
                        b"BioSeqSet" => Some(Blast4Value::BioSeqSet(read_node(reader).unwrap())),
                        b"SeqAlignSet" => Some(Blast4Value::SeqAlignSet(read_vec_node(reader,e.to_end()))),
                        b"QueryMask" => Some(Blast4Value::QueryMask(read_node(reader).unwrap())),
                        _ => {
                            forbidden.check(&name);
                            None
                        }
                    };

                    if value.is_some() {
                        return value;
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

/// Blast4-simple-results
///
/// Complete set of simple Blast results.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blast4SimpleResults {
    pub all_alignments: Vec<Blast4AlignmentsForQuery>,
}

/// Blast4-alignments-for-query
///
/// Alignments for one query.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blast4AlignmentsForQuery {
    /// Query sequence identifier
    pub query_id: String,
    /// All the alignments for this query
    pub alignments: Vec<Blast4SimpleAlignment>,
}

impl XmlNode for Blast4AlignmentsForQuery {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-alignments-for-query")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut alignments_for_query = Blast4AlignmentsForQuery {
            query_id: String::new(),
            alignments: Vec::new(),
        };
        let query_id_element = BytesStart::new("query_id");
        let alignments_element = BytesStart::new("alignments");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == query_id_element.name() {
                        alignments_for_query.query_id = read_string(reader).unwrap();
                    } else if name == alignments_element.name() {
                        alignments_for_query.alignments = read_vec_node(reader, alignments_element.to_end());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(alignments_for_query);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4AlignmentsForQuery {}


/// Blast4-simple-alignment
///
/// A single alignment record.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Blast4SimpleAlignment {
    /// Subject sequence identifier
    pub subject_id: String,
    /// E-value
    pub e_value: f64,
    /// Bit score
    pub bit_score: f64,
    /// Number of identities
    pub num_identities: Option<i64>,
    /// Number of insertions/deletions
    pub num_indels: Option<i64>,
    /// Full query range
    pub full_query_range: Blast4Range,
    /// Full subject range
    pub full_subject_range: Blast4Range,
}

impl XmlNode for Blast4SimpleAlignment {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-simple-alignment")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut alignment = Blast4SimpleAlignment {
            subject_id: String::new(),
            e_value: 0.0,
            bit_score: 0.0,
            num_identities: None,
            num_indels: None,
            full_query_range: Blast4Range::default(),
            full_subject_range: Blast4Range::default(),
        };
        let subject_id_element = BytesStart::new("subject_id");
        let e_value_element = BytesStart::new("e_value");
        let bit_score_element = BytesStart::new("bit_score");
        let num_identities_element = BytesStart::new("num_identities");
        let num_indels_element = BytesStart::new("num_indels");
        let full_query_range_element = BytesStart::new("full_query_range");
        let full_subject_range_element = BytesStart::new("full_subject_range");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == subject_id_element.name() {
                        alignment.subject_id = read_string(reader).unwrap();
                    } else if name == e_value_element.name() {
                        alignment.e_value = read_string(reader).unwrap().parse().unwrap();
                    } else if name == bit_score_element.name() {
                        alignment.bit_score = read_string(reader).unwrap().parse().unwrap();
                    } else if name == num_identities_element.name() {
                        alignment.num_identities = Some(read_string(reader).unwrap().parse().unwrap());
                    } else if name == num_indels_element.name() {
                        alignment.num_indels = Some(read_string(reader).unwrap().parse().unwrap());
                    } else if name == full_query_range_element.name() {
                        alignment.full_query_range = read_node(reader).unwrap();
                    } else if name == full_subject_range_element.name() {
                        alignment.full_subject_range = read_node(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(alignment);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4SimpleAlignment {}

/// Blast4-range
///
/// Range on a sequence (zero offset).
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blast4Range {
    pub start: Option<i64>,
    pub end: Option<i64>,
    /// The frame of the range (optional)
    pub strand: Option<i64>,
}

impl XmlNode for Blast4Range {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-range")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut range = Blast4Range::default();
        let start_element = BytesStart::new("start");
        let end_element = BytesStart::new("end");
        let strand_element = BytesStart::new("strand");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == start_element.name() {
                        range.start = Some(read_string(reader).unwrap().parse().unwrap());
                    } else if name == end_element.name() {
                        range.end = Some(read_string(reader).unwrap().parse().unwrap());
                    } else if name == strand_element.name() {
                        range.strand = Some(read_string(reader).unwrap().parse().unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(range);
                    }
                }
                _ => (),
            }
        }
    }
}

/*
/// Blast4-get-databases-ex-reply
///
/// Returns extended database info.
pub type Blast4GetDatabasesExReply = Vec<Blast4DatabaseInfo>;

/// Blast4-get-matrices-reply
///
/// Returns a list of matrix IDs.
pub type Blast4GetMatricesReply = Vec<Blast4MatrixId>;

/// Blast4-get-parameters-reply
///
/// Returns a list of parameter info.
pub type Blast4GetParametersReply = Vec<Blast4ParameterInfo>;

/// Blast4-get-paramsets-reply
///
/// Returns a list of task info.
pub type Blast4GetParamsetsReply = Vec<Blast4TaskInfo>;

/// Blast4-get-programs-reply
///
/// Returns a list of program info.
pub type Blast4GetProgramsReply = Vec<Blast4ProgramInfo>;

/// Blast4-get-seq-parts-reply
///
/// Returns sequence parts data.
pub type Blast4GetSeqPartsReply = Vec<Blast4SeqPartData>;

/// Blast4-get-sequences-reply
///
/// Returns a list of Bioseq.
pub type Blast4GetSequencesReply = Vec<Bioseq>;

/// Blast4-get-windowmasked-taxids-reply
///
/// Returns a list of taxonomic IDs.
pub type Blast4GetWindowmaskedTaxidsReply = Vec<i64>;
*/

impl XmlNode for Blast4Reply {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-reply")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut reply = Blast4Reply {
            errors: None,
            body: Blast4ReplyBody::FinishParams(vec![]), // Default placeholder
        };

        let errors_element = BytesStart::new("errors");
        let body_element = BytesStart::new("body");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == errors_element.name() {
                        reply.errors = Some(read_vec_node(reader,e.to_end()));
                    } else if e.name() == body_element.name() {
                        reply.body = read_node::<Blast4ReplyBody>(reader).unwrap();
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().name() {
                        return Some(reply);
                    }
                }
                Event::Eof => break,
                _ => (),
            }
        }
        None
    }
}

impl XmlNode for Blast4GetDatabasesExRequest {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-databases-ex-request") // Ensure this matches the actual XML tag
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Blast4GetDatabasesExRequest::default();
        let params_element = BytesStart::new("params");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == params_element.name() {
                        request.params = Some(read_vec_node(reader,e.to_end()));
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4GetSearchInfoRequest {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-search-info-request") // Ensure this matches the actual XML tag
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Blast4GetSearchInfoRequest::default();
        let request_id_element = BytesStart::new("request_id");
        let info_element = BytesStart::new("info");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == request_id_element.name() {
                        request.request_id = read_string(reader).unwrap();
                    } else if name == info_element.name() {
                        request.info = Some(read_vec_node(reader,e.to_end()));
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4GetRequestInfoRequest {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-request-info-request")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Blast4GetRequestInfoRequest::default();
        let request_id_element = BytesStart::new("request_id");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == request_id_element.name() {
                        request.request_id = read_string(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4QueueSearchRequest {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-queue-search-request")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Blast4QueueSearchRequest::default();
        let program_element = BytesStart::new("program");
        let service_element = BytesStart::new("service");
        let queries_element = BytesStart::new("queries");
        let subject_element = BytesStart::new("subject");
        let paramset_element = BytesStart::new("paramset");
        let algorithm_options_element = BytesStart::new("algorithm-options");
        let program_options_element = BytesStart::new("program-options");
        let format_options_element = BytesStart::new("format-options");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == program_element.name() {
                        request.program = read_string(reader).unwrap();
                    } else if name == service_element.name() {
                        request.service = read_string(reader).unwrap();
                    } else if name == queries_element.name() {
                        request.queries = read_node(reader).unwrap();
                    } else if name == subject_element.name() {
                        request.subject = read_node(reader).unwrap();
                    } else if name == paramset_element.name() {
                        request.paramset = Some(read_string(reader).unwrap());
                    } else if name == algorithm_options_element.name() {
                        request.algorithm_options = Some(read_node(reader).unwrap());
                    } else if name == program_options_element.name() {
                        request.program_options = Some(read_node(reader).unwrap());
                    } else if name == format_options_element.name() {
                        request.format_options = Some(read_node(reader).unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}


impl XmlNode for Blast4Parameter {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4Parameter")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut parameter = Blast4Parameter::default();
        let name_element = BytesStart::new("name");
        let value_element = BytesStart::new("value");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == name_element.name() {
                        parameter.name = read_string(reader).unwrap();
                    } else if name == value_element.name() {
                        parameter.value = read_node(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(parameter);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4Parameter {}

impl XmlNode for Blast4Parameters {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4Parameters")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        read_node(reader)
    }
}

impl XmlNode for Blast4GetSearchResultsRequest {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-search-results-request")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut request = Blast4GetSearchResultsRequest::default();
        let request_id_element = BytesStart::new("request_id");
        let result_types_element = BytesStart::new("result_types");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == request_id_element.name() {
                        request.request_id = read_string(reader).unwrap();
                    } else if name == result_types_element.name() {
                        request.result_types = Some(read_int(reader).unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(request);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4ReplyBody {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-reply-body")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let forbidden_tags = [BytesStart::new("unknown-tag")];
        let forbidden = UnexpectedTags(&forbidden_tags);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    let end = e.to_end();

                    let body = match name.as_ref() {
                        b"FinishParams" => Some(Blast4ReplyBody::FinishParams(read_node(reader)?)),
                        b"GetDatabases" => Some(Blast4ReplyBody::GetDatabases(read_vec_node(reader,end))),
                        b"GetMatrices" => Some(Blast4ReplyBody::GetMatrices(read_vec_node(reader,end))),
                        b"GetParameters" => Some(Blast4ReplyBody::GetParameters(read_vec_node(reader,end))),
                        b"GetParamsets" => Some(Blast4ReplyBody::GetParamsets(read_vec_node(reader,end))),
                        b"GetPrograms" => Some(Blast4ReplyBody::GetPrograms(read_vec_node(reader,end))),
                        b"GetSearchResults" => Some(Blast4ReplyBody::GetSearchResults(read_node(reader)?)),
                        b"GetSequences" => Some(Blast4ReplyBody::GetSequences(read_vec_node(reader,end))),
                        b"QueueSearch" => Some(Blast4ReplyBody::QueueSearch(read_node(reader)?)),
                        b"GetQueries" => Some(Blast4ReplyBody::GetQueries(read_node(reader)?)),
                        b"GetRequestInfo" => Some(Blast4ReplyBody::GetRequestInfo(read_node(reader)?)),
                        b"GetSequenceParts" => Some(Blast4ReplyBody::GetSequenceParts(read_vec_node(reader,end))),
                        b"GetWindowmaskedTaxids" => Some(Blast4ReplyBody::GetWindowmaskedTaxids(read_vec_int_unchecked(reader,&end))),
                        b"GetProtocolInfo" => Some(Blast4ReplyBody::GetProtocolInfo(read_node(reader)?)),
                        b"GetSearchInfo" => Some(Blast4ReplyBody::GetSearchInfo(read_node(reader)?)),
                        b"GetDatabasesEx" => Some(Blast4ReplyBody::GetDatabasesEx(read_vec_node(reader,end))),
                        _ => {
                            forbidden.check(&name);
                            None
                        }
                    };

                    if body.is_some() {
                        return body;
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

impl XmlNode for Blast4GetRequestInfoReply {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-request-info-reply")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut reply = Blast4GetRequestInfoReply::default();
        let database_element = BytesStart::new("database");
        let program_element = BytesStart::new("program");
        let service_element = BytesStart::new("service");
        let created_by_element = BytesStart::new("created_by");
        let queries_element = BytesStart::new("queries");
        let algorithm_options_element = BytesStart::new("algorithm_options");
        let program_options_element = BytesStart::new("program_options");
        let format_options_element = BytesStart::new("format_options");
        let subjects_element = BytesStart::new("subjects");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == database_element.name() {
                        reply.database = read_node(reader).unwrap();
                    } else if name == program_element.name() {
                        reply.program = read_string(reader).unwrap();
                    } else if name == service_element.name() {
                        reply.service = read_string(reader).unwrap();
                    } else if name == created_by_element.name() {
                        reply.created_by = read_string(reader).unwrap();
                    } else if name == queries_element.name() {
                        reply.queries = read_node(reader).unwrap();
                    } else if name == algorithm_options_element.name() {
                        reply.algorithm_options = read_node(reader).unwrap();
                    } else if name == program_options_element.name() {
                        reply.program_options = read_node(reader).unwrap();
                    } else if name == format_options_element.name() {
                        reply.format_options = Some(read_node(reader).unwrap());
                    } else if name == subjects_element.name() {
                        reply.subjects = Some(read_node(reader).unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(reply);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4GetQueriesReply {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-queries-reply")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut reply = Blast4GetQueriesReply::default();
        let queries_element = BytesStart::new("queries");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == queries_element.name() {
                        reply.queries = read_node(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(reply);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4QueueSearchReply {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-queue-search-reply")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut reply = Blast4QueueSearchReply::default();
        let request_id_element = BytesStart::new("request_id");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == request_id_element.name() {
                        reply.request_id = Some(read_string(reader).unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(reply);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4GetSearchResultsReply {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-get-search-results-reply")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut reply = Blast4GetSearchResultsReply::default();
        let alignments_element = BytesStart::new("alignments");
        let phi_alignments_element = BytesStart::new("phi_alignments");
        let masks_element = BytesStart::new("masks");
        let ka_blocks_element = BytesStart::new("ka_blocks");
        let search_stats_element = BytesStart::new("search_stats");
        let pssm_element = BytesStart::new("pssm");
        let simple_results_element = BytesStart::new("simple_results");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == alignments_element.name() {
                        reply.alignments = Some(read_vec_node(reader,e.to_end()));
                    } else if name == phi_alignments_element.name() {
                        reply.phi_alignments = read_node(reader);
                    } else if name == masks_element.name() {
                        reply.masks = Some(read_vec_node(reader,masks_element.to_end()));
                    } else if name == ka_blocks_element.name() {
                        reply.ka_blocks = Some(read_vec_node(reader,ka_blocks_element.to_end()));
                    } else if name == search_stats_element.name() {
                        reply.search_stats = Some(read_vec_node(reader,search_stats_element.to_end()));
                    } else if name == pssm_element.name() {
                        reply.pssm = Some(read_node(reader).unwrap());
                    } else if name == simple_results_element.name() {
                        reply.simple_results = Some(read_node(reader).unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(reply);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4SimpleResults {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-simple-results")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut results = Blast4SimpleResults {
            all_alignments: Vec::new(),
        };
        let alignments_element = BytesStart::new("all_alignments");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == alignments_element.name() {
                        results.all_alignments = read_vec_node(reader,alignments_element.to_end());
                    } else {
                        forbidden.check(&e.name());
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(results);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4Queries {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-queries")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let binding = [BytesStart::new("unknown-tag")];
        let forbidden = UnexpectedTags(&binding);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    let queries = match name.as_ref() {
                        b"pssm" => Some(Blast4Queries::pssm(read_node(reader)?)),
                        b"seq-loc-list" => Some(Blast4Queries::seqloclist(read_vec_node(reader, e.to_end()))),
                        b"bioseq-set" => Some(Blast4Queries::bioseqset(read_node(reader)?)),
                        _ => {
                            forbidden.check(&name);
                            None
                        }
                    };

                    if let Some(q) = queries {
                        return Some(q);
                    }
                }
                Event::End(e) if e.name() == Self::start_bytes().to_end().name() => {
                    break;
                }
                _ => (),
            }
        }
        None
    }
}


impl XmlNode for Blast4Subject {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-subject")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let forbidden_tags = [BytesStart::new("unknown-tag")];
        let forbidden = UnexpectedTags(&forbidden_tags);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {

                    let name = e.name();
                    let end = e.to_end();

                    let subject = match name.as_ref() {
                        b"Database" => Some(Blast4Subject::Database(read_string(reader)?)),
                        b"Sequences" => Some(Blast4Subject::Sequences(read_vec_node(reader,end))),
                        b"SeqLocList" => Some(Blast4Subject::SeqLocList(read_vec_node(reader,end))),
                        _ => {
                            forbidden.check(&name);
                            None
                        }
                    };

                    if subject.is_some() {
                        return subject;
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

impl XmlNode for Blast4Database {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-database")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut database = Blast4Database::default();
        let name_element = BytesStart::new("name");
        let type_element = BytesStart::new("type");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == name_element.name() {
                        database.name = read_string(reader).unwrap();
                    } else if name == type_element.name() {
                        database.r#type = read_node(reader).unwrap();
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(database);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4DatabaseInfo {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-database-info")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut info = Blast4DatabaseInfo::default();
        let database_element = BytesStart::new("database");
        let description_element = BytesStart::new("description");
        let last_updated_element = BytesStart::new("last_updated");
        let total_length_element = BytesStart::new("total_length");
        let num_sequences_element = BytesStart::new("num_sequences");
        let seqtech_element = BytesStart::new("seqtech");
        let taxid_element = BytesStart::new("taxid");
        let extended_element = BytesStart::new("extended");
        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    if name == database_element.name() {
                        info.database = read_node(reader).unwrap();
                    } else if name == description_element.name() {
                        info.description = read_string(reader).unwrap();
                    } else if name == last_updated_element.name() {
                        info.last_updated = read_string(reader).unwrap();
                    } else if name == total_length_element.name() {
                        info.total_length = read_int(reader).unwrap();
                    } else if name == num_sequences_element.name() {
                        info.num_sequences = read_int(reader).unwrap();
                    } else if name == seqtech_element.name() {
                        info.seqtech = read_node(reader).unwrap();
                    } else if name == taxid_element.name() {
                        info.taxid = read_int(reader).unwrap();
                    } else if name == extended_element.name() {
                        info.extended = Some(read_node(reader).unwrap());
                    } else {
                        forbidden.check(&name);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(info);
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlVecNode for Blast4DatabaseInfo {}

impl XmlNode for Blast4Seqtech {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-seqtech")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let forbidden = UnexpectedTags(&[]);
        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    forbidden.check(&name);
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(Blast4Seqtech::default());
                    }
                }
                _ => (),
            }
        }
    }
}

impl XmlNode for Blast4ResidueType {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Blast4-residue-type")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let forbidden = UnexpectedTags(&[]);
        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();
                    forbidden.check(&name);
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        return Some(Blast4ResidueType::default());
                    }
                }
                _ => (),
            }
        }
    }
}
