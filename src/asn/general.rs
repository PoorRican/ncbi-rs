//! General Data Elements
//!
//! As per [general.asn](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/asn_spec/general.asn.html)

use atoi::atoi;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde::{Serialize, Deserialize};
use crate::XMLElement;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
/// Model precise timestamp or an un-parsed string
///
/// The string form is a fall-back for when the input data cannot be parsed
/// into the standard date fields. It should only be used as a last resort to
/// accommodate old data, as it is impossible to compute or index on.
pub enum Date {
    Str(String),
    Date(DateStd),
}

impl Default for Date {
    fn default() -> Self {
        Self::Date(
            DateStd {
                year: 2023,
                ..DateStd::default()
            }
        )
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
/// NOTE: this is NOT a unix tm struct
pub struct DateStd {
    /// full year
    pub year: u16,
    /// month (1-12)
    pub month: Option<u8>,
    /// day of month (1-31
    pub day: Option<u8>,
    /// for "spring", "may-june", etc
    pub season: Option<String>,
    /// hour of day (0-23)
    pub hour: Option<u8>,
    /// minute of hour (0-59)
    pub minute: Option<u8>,
    /// second of minute (0-59)
    pub second: Option<u8>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Can tag or name anything
pub enum ObjectId {
    Id(u64),
    Str(String),
}

impl XMLElement for ObjectId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Object-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Self {
        // variants
        let id_element = BytesStart::new("Object-id_id");
        let str_element = BytesStart::new("Object-id_str");

        loop {
            if let Event::Start(e) = reader.read_event().unwrap() {
                if e.name() == id_element.name() {
                    if let Event::Text(text) = reader.read_event().unwrap() {
                        return ObjectId::Id(
                            atoi(text.as_ref()).expect("Can't parse &[u8] into int")
                        ).into();
                    }
                }
                else if e.name() == str_element.name() {
                    if let Event::Text(text) = reader.read_event().unwrap() {
                        return ObjectId::Str(text.escape_ascii().to_string())
                    }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Generalized for tagging
pub struct DbTag {
    /// name of database or system
    pub db: String,
    /// appropriate tag
    pub tag: ObjectId,
}

impl XMLElement for DbTag {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Dbtag")
    }
    fn from_reader(reader: &mut Reader<&[u8]>) -> Self {
        let mut db = None;
        let mut tag = None;

        let db_element = BytesStart::new("Dbtag_db");
        let tag_element = BytesStart::new("Dbtag_tag");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    if e.name() == db_element.name() {
                        if let Event::Text(text) = reader.read_event().unwrap() {
                            db = text.escape_ascii().to_string().into();
                        }
                    }
                    else if e.name() == tag_element.name() {
                        tag = ObjectId::from_reader(reader).into();
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::start_bytes().to_end().name() {
                        break;
                    }
                }
                _ => ()
            }
        }

        Self {
            db: db.unwrap(),
            tag: tag.unwrap(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
/// define a std element for people
pub enum PersonId {
    /// any defined database tag
    DbTag(DbTag),
    /// structured name
    Name(NameStd),
    /// MEDLINE name (semi-structured)
    ML(String),
    /// unstructured name
    Str(String),
    /// consortium name
    Consortium(String),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// structured names
pub struct NameStd {
    pub last: String,
    pub first: Option<String>,
    pub middle: Option<String>,

    /// full name (eg: "J. John Smith, Esq")
    pub full: Option<String>,

    /// first + middle initials
    pub initials: Option<String>,

    /// Jr, Sr, III
    pub suffix: Option<String>,

    /// Dr., Sister, etc
    pub title: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Range {
    pub max: i64,
    pub min: i64,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum FuzzLimit {
    /// unknown
    Unk,

    /// greater than
    GT,

    /// less than
    LT,

    /// space to right of position
    TR,

    /// space to left of position
    TL,

    /// artificial break at origin of circle
    Circle,

    /// something else
    Other = 255,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
/// Communicate uncertainties in integer values
pub enum IntFuzz {
    #[serde(rename="p-m")]
    /// plus or minus fixed amount
    PM(i64),

    Range(Range),
    Pct(i64),
    Lim(FuzzLimit),
    Alt(Vec<i64>),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// a general object for a user defined structured data item
///
/// used by [`SeqFeat`] and [`SeqDescr`]
pub struct UserObject {
    /// endeavor which designed this object
    pub class: Option<String>,

    #[serde(rename="type")]
    /// type of object within class
    pub r#type: ObjectId,

    /// the object itself
    pub data: Vec<UserField>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum UserData {
    Str(String),
    Int(i64),
    Real(f64),
    Bool(bool),
    // OS(`octal string`),
    /// for using other definitions
    Object(UserObject),
    Strs(Vec<String>),
    Ints(Vec<i64>),
    Reals(Vec<f64>),
    Fields(Vec<UserField>),
    Objects(Vec<UserObject>),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct UserField {
    /// field label
    pub label: ObjectId,
    /// required for strs, ints, reals, oss
    pub num: Option<i64>,
    /// field contents
    pub data: UserData,
}
