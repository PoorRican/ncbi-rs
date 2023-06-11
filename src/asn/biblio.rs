//! Bibliographic data elements
//! Adapted from ["biblio.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/biblio/biblio.asn)

use crate::asn::{Date, DbTag, PersonId};
use std::collections::HashSet;

pub enum ArticleId {
    PubMed(PubMedId),
    Medline(MedlineUID),
    DOI(DOI),
    PmcId(PmcID),
    PmPid(PmPid),
    /// generic catch all
    Other(DbTag),
}

/// id from the PubMed database at NCBI
pub type PubMedId = u64;

/// id from MEDLINE
pub type MedlineUID = u64;

/// Document Object Identifier
pub type DOI = String;

/// Controlled Publisher Identifier
pub type PII = String;

/// PubMed Central Id
pub type PmcID = u64;

/// Publisher Id supplied to PubMed Central
pub type PmcPid = String;

/// Publisher Id supplied to PubMed
pub type PmPid = String;

pub type ArticleIdSet = HashSet<ArticleId>;

/// points of publication
pub enum PubStatus {
    /// date manuscript received for review
    Received = 1,

    /// accepted for publication
    Accepted,

    /// published electronically by publisher
    EPublish,

    /// published in print by publisher
    PPublish,

    /// article revised by publisher/author
    Revised,

    /// article first appeared in PubMed Central
    PMC,

    /// article revision in PubMed Central
    PMCR,

    /// article first citation appeared in PubMed
    PubMed,

    /// article citation revision in PubMed
    PubMedR,

    /// epublish, but will be followed by print
    AheadOfPrint,

    /// date into PreMedline status
    PreMedline,

    /// date made a MEDLINE record
    Medline,

    Other = 255,
}

/// done as a struct so fields can be added
pub struct PubStatusDate {
    pub pubstatus: PubStatus,
    /// time may be added later
    pub date: Date,
}

pub type PubStatusDateSet = HashSet<PubStatusDate>;

/// journal or book
pub enum CitArtFrom {
    Journal(CitJour),
    Book(CitBook),
    Proc(CitProc),
}

/// Article in journal or book
pub struct CitArt {
    /// title or paper (ANSI requires)
    pub title: Option<Title>,

    /// authors (ANSI requires)
    pub authors: Option<AuthList>,

    /// journal or book
    pub from: CitArtFrom,

    pub ids: Option<ArticleIdSet>,
}

/// journal citation
pub struct CitJour {
    /// title of journal
    pub title: Title,
    pub imp: Imprint,
}

/// book citation
pub struct CitBook {
    /// title of book
    pub title: Title,

    /// part of a collection
    pub coll: Option<Title>,

    /// authors
    pub authors: AuthList,

    pub imp: Imprint,
}

/// meeting proceedings
pub struct CitProc {
    /// citation to meeting
    pub book: CitBook,
    /// time and location of meeting
    pub meet: Meeting,
}

/// Patent citation
pub struct CitPat {
    pub title: String,

    /// author/inventor
    pub authors: AuthList,

    /// patent document country
    pub country: String,

    /// patent document type
    pub doc_type: String,

    /// patent document number
    pub number: Option<String>,

    /// patent issue/pub date
    pub date_issue: Option<Date>,

    /// patent doc class code
    pub class: Option<Vec<String>>,

    /// patent doc application number
    pub app_number: Option<String>,

    /// patent application file date
    pub app_date: Option<Date>,

    /// applicants
    pub applicants: Option<AuthList>,

    /// assignees
    pub assignees: Option<AuthList>,

    /// priorities
    pub priority: Option<Vec<PatentPriority>>,

    /// abstract of patent
    pub r#abstract: Option<String>,
}

pub struct PatentPriority {
    /// patent country code
    pub country: String,

    /// number assigned in that country
    pub number: String,

    /// date of application
    pub date: Date,
}

pub enum IdPatChoice {
    /// patent document number
    Number(String),

    /// patent doc application number
    AppNumber(String),
}

/// identifies a patent
pub struct IdPat {
    /// patent document country
    pub country: String,

    pub id: IdPatChoice,

    ///patent doc type
    pub doc_type: Option<String>,
}

pub enum LetType {
    Manuscript = 1,
    Letter,
    Thesis,
}

/// cite a letter, thesis, or manuscript
pub struct CitLet {
    /// same fields as a book
    pub cit: CitBook,

    /// manuscript identifier
    pub man_id: Option<String>,

    pub r#type: LetType,
}

/// represents medium of submission
pub enum SubMedium {
    Paper = 1,
    Tape,
    Floppy,
    Email,
    Other = 255,
}

pub struct CitSub {
    /// not necessarily authors of the paper
    pub authors: AuthList,

    /// only used to get date.
    ///
    /// Might be deprecated soon.
    pub imp: Option<Imprint>,

    /// medium of submission
    pub medium: SubMedium,

    /// replaces imp, will become required
    pub date: Option<Date>,

    /// description of changes for public view
    pub descr: Option<String>,
}

/// NOT from ANSI, this is a catchall
pub struct CitGen {
    /// anything, not parsable
    pub cit: Option<String>,

    pub authors: Option<AuthList>,

    /// medline uid
    pub muid: Option<u64>,

    pub journal: Option<Title>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub date: Option<Date>,

    /// for GenBank style references
    pub serial_number: Option<u64>,

    /// eg. cit="unpublished",title="title"
    pub title: Option<String>,

    /// PubMed Id
    pub pmid: Option<PubMedId>,
}

pub enum AuthListNames {
    /// full citations
    Std(Vec<Author>),

    /// MEDLINE, semi-structured
    Ml(Vec<String>),

    /// free-for-all
    Str(Vec<String>),
}

/// authorship group
pub struct AuthList {
    pub names: AuthListNames,

    /// author affiliation
    pub affil: Option<Affil>,
}

pub enum AuthorLevel {
    Primary = 1,
    Secondary,
}

pub enum AuthorRole {
    Compiler = 1,
    Editor,
    PatentAssignee,
    Translator,
}

pub struct Author {
    /// author, primary, or secondary
    pub name: PersonId,
    pub level: Option<AuthorLevel>,

    /// author role indicator
    pub role: Option<AuthorRole>,

    pub affil: Option<Affil>,

    /// true if [corresponding author](https://scientific-publishing.webshop.elsevier.com/publication-recognition/what-corresponding-author/)
    pub is_corr: Option<bool>,
}

/// std representation for affiliations
pub struct AffilStd {
    /// Author Affiliation, Name
    pub affil: Option<String>,

    /// Author Affiliation, Division
    pub div: Option<String>,

    /// Author Affiliation, City
    pub city: Option<String>,

    /// Author Affiliation, County Sub
    pub sub: Option<String>,

    /// Author Affiliation, Country
    pub country: Option<String>,

    /// street address, not ANSI
    pub street: Option<String>,

    pub email: Option<String>,
    pub fax: Option<String>,
    pub phone: Option<String>,
    pub postal_code: Option<String>,
}

pub enum Affil {
    /// unparsed string
    Str(String),

    /// std representation
    Std(AffilStd),
}

/// title group
///
/// # Variants
///
/// Only certain variants are valid for certain types:
/// Valid for = A = Analytic [`CitArt`]
///             J = Journals [`CitJour`]
///             B = Book [`CitBook`]
pub enum TitleItem {
    /// Title, Anal,Coll,Mono
    /// Valid: AJB
    Name(String),

    /// Title, Subordinate
    /// Valid: A B
    TSub(String),

    /// Title, Translated
    /// Valid: AJB
    Trans(String),

    /// Title, Abbreviated
    /// Valid:  J
    Jta(String),

    /// Title, MEDLINE jta
    /// Valid:  J
    IsoJta(String),

    /// specifically ISO jta
    /// Valid:  J
    MlJta(String),

    /// a coden
    /// Valid:  J
    Coden(String),

    /// ISSN
    /// Valid:  J
    ISSN(String),

    /// Title, Abbreviated
    /// Valid:  B
    Abr(String),

    /// ISBN
    /// Valid:  B
    ISBN(String),
}

pub type Title = HashSet<TitleItem>;

/// For pre-publication citations
pub enum ImprintPrePub {
    /// submitted, not accepted
    Submitted = 1,

    /// accepted, not published
    InPress,

    Other = 255,
}

pub struct Imprint {
    /// date of publication
    pub date: Date,

    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub section: Option<String>,

    /// publisher, required for book
    pub r#pub: Option<Affil>,

    /// copyright date, required for book
    pub cprt: Option<Date>,

    /// part/sup of volume
    pub part_sup: Option<String>,

    /// put here for simplicity
    // TODO: default "ENG"
    pub language: Option<String>,

    /// for pre-publication citations
    pub prepub: Option<ImprintPrePub>,

    /// part/sup on issue
    pub part_supi: Option<String>,

    /// retraction info
    pub retract: Option<CitRetract>,

    /// current status of this publication
    pub pubstatus: Option<PubStatus>,

    /// dates for this record
    pub history: Option<PubStatusDateSet>,
}

/// represents type of entry retraction
pub enum CitRetractType {
    /// this citation is retracted
    Retracted = 1,

    /// this citation is a retraction notice
    Notice,

    /// an erratum was published about this
    InError,

    /// citation and/or explanation
    Erratum,
}

pub struct CitRetract {
    /// retraction of an entry
    pub r#type: CitRetractType,

    /// citation and/or explanation
    pub exp: Option<String>,
}

pub struct Meeting {
    pub number: String,
    pub date: Date,
    pub place: Option<Affil>,
}
