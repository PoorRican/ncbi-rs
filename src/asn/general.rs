//! General Data Elements
//!
//! As per [general.asn](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/asn_spec/general.asn.html)

use atoi::atoi;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use serde::{Serialize, Deserialize};
use crate::parsing_utils::{get_next_num, get_next_text, get_vec, get_vec_num, get_vec_text, try_field};
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

impl XMLElement for Date {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Date")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variants
        let std_element = BytesStart::new("Date-std");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == std_element.name() {
                        return Date::Date(DateStd::from_reader(reader).unwrap()).into();
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

impl XMLElement for DateStd {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Date-std")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut date = Self::default();

        // elements
        let year_element = BytesStart::new("Date-std_year");
        let month_element = BytesStart::new("Date-std_month");
        let day_element = BytesStart::new("Date-std_day");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == year_element.name() {
                        date.year = get_next_num::<u16>(reader);
                    }
                    if name == month_element.name() {
                        date.month = get_next_num::<u8>(reader).into();
                    }
                    if name == day_element.name() {
                        date.day = get_next_num::<u8>(reader).into();
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return date.into()
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Can tag or name anything
pub enum ObjectId {
    Id(u64),
    Str(String),
}

/// explicitly implemented because a default is not in original spec
impl Default for ObjectId {
    fn default() -> Self {
        Self::Str(String::default())
    }
}

impl XMLElement for ObjectId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Object-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
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
                        return ObjectId::Str(text.escape_ascii().to_string()).into()
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
    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
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
                        tag = ObjectId::from_reader(reader);
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
        }.into()
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

impl Default for PersonId {
    fn default() -> Self {
        Self::Str(String::default())
    }
}

impl XMLElement for PersonId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Person-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variants
        let name_element = BytesStart::new("Person-id_name");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == name_element.name() {
                        return PersonId::Name(NameStd::from_reader(reader).unwrap()).into()
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

impl XMLElement for NameStd {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Name-std")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut name_std = NameStd::default();

        // elements
        let last_element = BytesStart::new("Name-std_last");
        let first_element = BytesStart::new("Name-std_first");
        let initials_element = BytesStart::new("Name-std_initials");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    try_field(&name, &last_element, &mut name_std.last, reader);
                    try_field(&name, &first_element, &mut name_std.first, reader);
                    try_field(&name, &initials_element, &mut name_std.initials, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return name_std.into()
                    }
                }
                _ => ()
            }

        }
    }
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
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

impl XMLElement for UserObject {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("User-object")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut object = Self::default();

        // elements
        let class_element = BytesStart::new("User-object_class");
        let data_element = BytesStart::new("User-object_data");
        let type_element = BytesStart::new("User-object_type");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    try_field(&name, &class_element, &mut object.class, reader);

                    if name == type_element.name() {
                        object.r#type = ObjectId::from_reader(reader).unwrap();
                    }

                    else if name == data_element.name() {
                        object.data = UserField::vec_from_reader(reader, data_element.to_end());
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return object.into()
                    }
                }
                _ => ()
            }
        }
    }
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

impl UserData {
    fn parse_strs(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {

        let end = BytesEnd::new("User-field_data_strs");

        let items = get_vec_text(reader, &end);

        return Self::Strs(items).into()
    }

    fn parse_ints(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let end = BytesEnd::new("User-field_data_ints");

        let items = get_vec_num(reader, &end);

        return Self::Ints(items).into()
    }

    fn parse_reals(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        unimplemented!()
    }

    fn parse_fields(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let end = BytesEnd::new("User-field_data_fields");

        return Self::Fields(
            get_vec(
                reader,
                &UserField::start_bytes(),
                &UserField::from_reader, &end
            )
        ).into()
    }

    fn parse_objects(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let end = BytesEnd::new("User-field_data_fields");

        return Self::Objects(
            get_vec(
                reader,
                &UserObject::start_bytes(),
                &UserObject::from_reader, &end
            )
        ).into()
    }
}

/// explicitly implemented because a default is not in original spec
impl Default for UserData {
    fn default() -> Self {
        Self::Str(String::default())
    }
}

impl XMLElement for UserData {
    /// This enumerated value is not enclosed by a tag
    fn start_bytes() -> BytesStart<'static> {
        unimplemented!()
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variants
        let str_element = BytesStart::new("User-field_data_str");
        let int_element = BytesStart::new("User-field_data_int");
        let real_element = BytesStart::new("User-field_data_real");
        let bool_element = BytesStart::new("User-field_data_bool");
        let object_element = BytesStart::new("User-field_data_object");
        let strs_element = BytesStart::new("User-field_data_strs");
        let ints_element = BytesStart::new("User-field_data_ints");
        let reals_element = BytesStart::new("User-field_data_reals");
        let fields_element = BytesStart::new("User-field_data_fields");
        let objects_element = BytesStart::new("User-field_data_str");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == str_element.name() {
                        return Self::Str(get_next_text(reader).unwrap()).into();
                    }
                    if name == int_element.name() {
                        let num = get_next_num::<i64>(reader);
                        return Self::Int(num).into();
                    }
                    if name == real_element.name() {
                        unimplemented!()
                    }
                    if name == bool_element.name() {
                        unimplemented!()
                    }
                    if name == object_element.name() {
                        return Self::Object(UserObject::from_reader(reader).unwrap()).into();
                    }
                    if name == strs_element.name() {
                        return Self::parse_strs(reader)
                    }
                    if name == ints_element.name() {
                        return Self::parse_ints(reader)
                    }
                    if name == reals_element.name() {
                        return Self::parse_reals(reader)
                    }
                    if name == fields_element.name() {
                        return Self::parse_fields(reader)
                    }
                    if name == objects_element.name() {
                        return Self::parse_objects(reader)
                    }
                }
                _ => ()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct UserField {
    /// field label
    pub label: ObjectId,
    /// required for strs, ints, reals, oss
    pub num: Option<i64>,
    /// field contents
    pub data: UserData,
}

impl XMLElement for UserField {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("User-field")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        let mut field = Self::default();

        // elements
        let label_element = BytesStart::new("User-field_label");
        let num_element = BytesStart::new("User-field_num");
        let data_element = BytesStart::new("User-field_data");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == label_element.name() {
                        field.label = ObjectId::from_reader(reader).unwrap();
                    }
                    else if name == num_element.name() {
                        field.num = get_next_num::<i64>(reader).into();
                    }
                    else if name == data_element.name() {
                        field.data = UserData::from_reader(reader).unwrap();
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return field.into()
                    }
                }
                _ => ()
            }
        }
    }
}
