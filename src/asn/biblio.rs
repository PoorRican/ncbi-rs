//! Bibliographic data elements
//! Adapted from ["biblio.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/biblio/biblio.asn)

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::general::{Date, DbTag, PersonId};
use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::parsing_utils::{try_next_string, parse_next_string_into};
use crate::{XMLElement, XMLElementVec};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
/// represents multiple ways to id an article
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

pub type ArticleIdSet = Vec<ArticleId>;

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// points of publication
///
/// # Notes
///
/// Originally implement as `INTEGER`. Therefore, it is assumed that serialized
/// representation is an 8-bit integer.
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// done as a struct so fields can be added
pub struct PubStatusDate {
    pub pubstatus: PubStatus,
    /// time may be added later
    pub date: Date,
}

pub type PubStatusDateSet = Vec<PubStatusDate>;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
/// journal or book
pub enum CitArtFrom {
    Journal(CitJour),
    Book(CitBook),
    Proc(CitProc),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// journal citation
pub struct CitJour {
    /// title of journal
    pub title: Title,
    pub imp: Imprint,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// meeting proceedings
pub struct CitProc {
    /// citation to meeting
    pub book: CitBook,
    /// time and location of meeting
    pub meet: Meeting,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
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

    #[serde(rename="abstract")]
    /// abstract of patent
    pub r#abstract: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PatentPriority {
    /// patent country code
    pub country: String,

    /// number assigned in that country
    pub number: String,

    /// date of application
    pub date: Date,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum IdPatChoice {
    /// patent document number
    Number(String),

    /// patent doc application number
    AppNumber(String),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
/// identifies a patent
pub struct IdPat {
    /// patent document country
    pub country: String,

    pub id: IdPatChoice,

    ///patent doc type
    pub doc_type: Option<String>,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer
pub enum CitLetType {
    Manuscript = 1,
    Letter,
    Thesis,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
/// cite a letter, thesis, or manuscript
pub struct CitLet {
    /// same fields as a book
    pub cit: CitBook,

    /// manuscript identifier
    pub man_id: Option<String>,

    #[serde(rename="type")]
    pub r#type: CitLetType,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u8)]
/// Internal representation for medium of submission for `medium` in [`CitSub`]
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer
pub enum CitSubMedium {
    #[default]
    Paper = 1,
    Tape,
    Floppy,
    Email,
    Other = 255,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Cite a direct data submission
///
/// # Original Comment
///     See "NCBI-Submit" for the form of a direct sequence submission
pub struct CitSub {
    /// not necessarily authors of the paper
    pub authors: AuthList,

    /// only used to get date.
    ///
    /// Might be deprecated soon.
    pub imp: Option<Imprint>,

    /// medium of submission
    pub medium: CitSubMedium,

    /// replaces imp, will become required
    pub date: Option<Date>,

    /// description of changes for public view
    pub descr: Option<String>,
}

impl CitSub {
    pub fn new(authors: AuthList) -> Self {
        Self {
            authors,
            imp: None,
            medium: Default::default(),
            date: None,
            descr: None,
        }
    }
}

impl XMLElement for CitSub {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Cit-sub")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let authors_element = BytesStart::new("Cit-sub_authors");
        let date_element = BytesStart::new("Cit-sub_date");

        let mut cit = CitSub::new(
            AuthList {
                names: AuthListNames::Std(vec![]), affil: None
            }
        );

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == authors_element.name() {
                        cit.authors = AuthList::from_reader(reader).unwrap();
                    }
                    if name == date_element.name() {
                        cit.date = Date::from_reader(reader);
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        break;
                    }
                }
                _ => ()
            }
        }

        cit.into()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all="kebab-case")]
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

impl XMLElement for CitGen {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Cit-gen")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut gen = CitGen::default();

        // elements
        let cit_element = BytesStart::new("Cit-gen_cit");
        let authors_element = BytesStart::new("Cit-gen_authors");
        let title_element = BytesStart::new("Cit-gen_title");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == cit_element.name() {
                        gen.cit = try_next_string(reader);
                    }
                    else if name == authors_element.name() {
                        gen.authors = AuthList::from_reader(reader);
                    }
                    else if name == title_element.name() {
                        gen.title = try_next_string(reader)
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return gen.into();
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum AuthListNames {
    /// full citations
    Std(Vec<Author>),

    /// MEDLINE, semi-structured
    Ml(Vec<String>),

    /// free-for-all
    Str(Vec<String>),
}

/// Explicit definition instead of using derive
///
/// This default is not in original NCBI spec,
/// therefore, it is subject to change
impl Default for AuthListNames {
    fn default() -> Self {
        Self::Str(vec![])
    }
}

impl XMLElement for AuthListNames {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Auth-list_names")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variants
        let std_element = BytesStart::new("Auth-list_names_std");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == std_element.name() {
                        return Self::Std(Author::vec_from_reader(reader, Self::start_bytes().to_end())).into()
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
/// authorship group
pub struct AuthList {
    pub names: AuthListNames,

    /// author affiliation
    pub affil: Option<Affil>,
}

impl XMLElement for AuthList {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Auth-list")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut list = AuthList::default();

        let names_element = BytesStart::new("Auth-list_names");
        let affil_element = BytesStart::new("Auth-list_affil");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == names_element.name() {
                        list.names = AuthListNames::from_reader(reader).unwrap();
                    }
                    else if name == affil_element.name() {
                        list.affil = Affil::from_reader(reader)
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return list.into()
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer
pub enum AuthorLevel {
    Primary = 1,
    Secondary,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer
pub enum AuthorRole {
    Compiler = 1,
    Editor,
    PatentAssignee,
    Translator,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
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

impl Author {
    pub fn new(name: PersonId) -> Self {
        Self {
            name,
            level: None,
            role: None,
            affil: None,
            is_corr: None
        }
    }
}

impl XMLElement for Author {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Author")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut author = Author::new(PersonId::default());

        let name_element = BytesStart::new("Author_name");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == name_element.name() {
                        author.name = PersonId::from_reader(reader).unwrap();
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return author.into()
                    }
                }
                _ => ()
            }
        }
    }
}
impl XMLElementVec for Author {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all="kebab-case")]
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

impl XMLElement for AffilStd {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Affil_std")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut affil = AffilStd::default();

        // elements
        let affil_element = BytesStart::new("Affil_std_affil");
        let div_element = BytesStart::new("Affil_std_div");
        let city_element = BytesStart::new("Affil_std_city");
        let sub_element = BytesStart::new("Affil_std_sub");
        let country_element = BytesStart::new("Affil_std_country");
        let street_element = BytesStart::new("Affil_std_street");
        let postal_code_element = BytesStart::new("Affil_std_postal-code");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_next_string_into(&name, &affil_element, &mut affil.affil, reader);
                    parse_next_string_into(&name, &div_element, &mut affil.div, reader);
                    parse_next_string_into(&name, &city_element, &mut affil.city, reader);
                    parse_next_string_into(&name, &sub_element, &mut affil.sub, reader);
                    parse_next_string_into(&name, &country_element, &mut affil.country, reader);
                    parse_next_string_into(&name, &street_element, &mut affil.street, reader);
                    parse_next_string_into(&name, &postal_code_element, &mut affil.postal_code, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return affil.into()
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum Affil {
    /// unparsed string
    Str(String),

    /// std representation
    Std(AffilStd),
}

impl XMLElement for Affil {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Affil")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variants
        let str_element = BytesStart::new("Affil_str");
        let std_element = BytesStart::new("Affil_std");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == std_element.name() {
                        return Self::Std(AffilStd::from_reader(reader).unwrap()).into()
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
#[serde(rename_all="lowercase")]
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

    #[serde(rename="iso-jta")]
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

pub type Title = Vec<TitleItem>;

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// For pre-publication citations
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer
pub enum ImprintPrePub {
    /// submitted, not accepted
    Submitted = 1,

    /// accepted, not published
    InPress,

    Other = 255,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="kebab-case")]
pub struct Imprint {
    /// date of publication
    pub date: Date,

    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub section: Option<String>,

    #[serde(rename="pub")]
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

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
/// represents type of entry retraction
///
/// # Note
///
/// Original implementation lists this as `ENUMERATED`, therefore it is assumed that
/// serialized representation is an integer
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct CitRetract {
    #[serde(rename="type")]
    /// retraction of an entry
    pub r#type: CitRetractType,

    /// citation and/or explanation
    pub exp: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Meeting {
    pub number: String,
    pub date: Date,
    pub place: Option<Affil>,
}
