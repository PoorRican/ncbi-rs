//! General Data Elements
//!
//! As per [general.asn](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/asn_spec/general.asn.html)

use crate::parsing_utils::{
    parse_int_to, parse_int_to_option, parse_node_to, parse_string_to, parse_vec_node,
    parse_vec_node_to, read_int, read_node, read_string, read_vec_int_unchecked,
    read_vec_str_unchecked,
};
use crate::{XmlNode, XmlVecNode};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
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
        Self::Date(DateStd {
            year: 2023,
            ..DateStd::default()
        })
    }
}

impl XmlNode for Date {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Date")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        // variants
        let std_element = BytesStart::new("Date-std");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == std_element.name() {
                        return Date::Date(read_node(reader).unwrap()).into();
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return None;
                    }
                }
                _ => (),
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

impl XmlNode for DateStd {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Date-std")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut date = Self::default();

        // elements
        let year_element = BytesStart::new("Date-std_year");
        let month_element = BytesStart::new("Date-std_month");
        let day_element = BytesStart::new("Date-std_day");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_int_to(&name, &year_element, &mut date.year, reader);
                    parse_int_to_option(&name, &month_element, &mut date.month, reader);
                    parse_int_to_option(&name, &day_element, &mut date.day, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return date.into();
                    }
                }
                _ => (),
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

impl XmlNode for ObjectId {
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
                    return ObjectId::Id(read_int(reader).unwrap()).into();
                }
                if e.name() == str_element.name() {
                    return ObjectId::Str(read_string(reader).unwrap()).into();
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default)]
/// Generalized for tagging
pub struct DbTag {
    /// name of database or system
    pub db: String,
    /// appropriate tag
    pub tag: ObjectId,
}

impl XmlNode for DbTag {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Dbtag")
    }
    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> {
        let mut tag = DbTag::default();

        let db_element = BytesStart::new("Dbtag_db");
        let tag_element = BytesStart::new("Dbtag_tag");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_string_to(&name, &db_element, &mut tag.db, reader);
                    parse_node_to(&name, &tag_element, &mut tag.tag, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return tag.into();
                    }
                }
                _ => (),
            }
        }
    }
}
impl XmlVecNode for DbTag {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
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

impl XmlNode for PersonId {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Person-id")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        // variants
        let name_element = BytesStart::new("Person-id_name");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == name_element.name() {
                        return PersonId::Name(read_node(reader).unwrap()).into();
                    }
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return None;
                    }
                }
                _ => (),
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

impl XmlNode for NameStd {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Name-std")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut name_std = NameStd::default();

        // elements
        let last_element = BytesStart::new("Name-std_last");
        let first_element = BytesStart::new("Name-std_first");
        let initials_element = BytesStart::new("Name-std_initials");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_string_to(&name, &last_element, &mut name_std.last, reader);
                    parse_string_to(&name, &first_element, &mut name_std.first, reader);
                    parse_string_to(&name, &initials_element, &mut name_std.initials, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return name_std.into();
                    }
                }
                _ => (),
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
#[serde(rename_all = "lowercase")]
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
#[serde(rename_all = "lowercase")]
/// Communicate uncertainties in integer values
pub enum IntFuzz {
    #[serde(rename = "p-m")]
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

    #[serde(rename = "type")]
    /// type of object within class
    pub r#type: ObjectId,

    /// the object itself
    pub data: Vec<UserField>,
}

impl XmlNode for UserObject {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("User-object")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut object = Self::default();

        // elements
        let class_element = BytesStart::new("User-object_class");
        let data_element = BytesStart::new("User-object_data");
        let type_element = BytesStart::new("User-object_type");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_string_to(&name, &class_element, &mut object.class, reader);
                    parse_node_to(&name, &type_element, &mut object.r#type, reader);
                    parse_vec_node_to(&name, &data_element, &mut object.data, reader);
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return object.into();
                    }
                }
                _ => (),
            }
        }
    }
}
impl XmlVecNode for UserObject {}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UserData {
    Str(String),
    Int(i64),
    Real(String),
    Bool(bool),
    // OS(`octal string`),
    /// for using other definitions
    Object(UserObject),
    Strs(Vec<String>),
    Ints(Vec<i64>),
    Reals(Vec<String>),
    Fields(Vec<UserField>),
    Objects(Vec<UserObject>),
}

impl UserData {
    fn parse_strs(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let end = BytesEnd::new("User-field_data_strs");

        let items = read_vec_str_unchecked(reader, &end);

        return Self::Strs(items).into();
    }

    fn parse_ints(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let end = BytesEnd::new("User-field_data_ints");

        let items = read_vec_int_unchecked(reader, &end);

        return Self::Ints(items).into();
    }

    fn parse_reals(_reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let end = BytesEnd::new("User-field_data_reals");

        let items = read_vec_str_unchecked(reader, &end);

        return Self::Reals(items).into();
    }

    fn parse_fields(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let end = BytesEnd::new("User-field_data_fields");

        return Self::Fields(parse_vec_node(reader, end)).into();
    }

    fn parse_objects(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let end = BytesEnd::new("User-field_data_fields");

        return Self::Objects(parse_vec_node(reader, end)).into();
    }
}

/// explicitly implemented because a default is not in original spec
impl Default for UserData {
    fn default() -> Self {
        Self::Str(String::default())
    }
}

impl XmlNode for UserData {
    /// This enumerated value is not enclosed by a tag
    fn start_bytes() -> BytesStart<'static> {
        unimplemented!()
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
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
                        return Self::Str(read_string(reader).unwrap()).into();
                    }
                    if name == int_element.name() {
                        return Self::Int(read_int::<i64>(reader).unwrap()).into();
                    }
                    if name == real_element.name() {
                        return Self::Real(read_real(reader).unwrap()).into()
                    }
                    if name == bool_element.name() {
                        unimplemented!()
                    }
                    if name == object_element.name() {
                        return Self::Object(read_node(reader).unwrap()).into();
                    }
                    if name == strs_element.name() {
                        return Self::parse_strs(reader);
                    }
                    if name == ints_element.name() {
                        return Self::parse_ints(reader);
                    }
                    if name == reals_element.name() {
                        return Self::parse_reals(reader);
                    }
                    if name == fields_element.name() {
                        return Self::parse_fields(reader);
                    }
                    if name == objects_element.name() {
                        return Self::parse_objects(reader);
                    }
                }
                _ => (),
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

impl XmlNode for UserField {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("User-field")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut field = Self::default();

        // elements
        let label_element = BytesStart::new("User-field_label");
        let num_element = BytesStart::new("User-field_num");
        let data_element = BytesStart::new("User-field_data");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    parse_node_to(&name, &label_element, &mut field.label, reader);
                    parse_node_to(&name, &data_element, &mut field.data, reader);
                    parse_int_to_option(&name, &num_element, &mut field.num, reader)
                }
                Event::End(e) => {
                    if Self::is_end(&e) {
                        return field.into();
                    }
                }
                _ => (),
            }
        }
    }
}
impl XmlVecNode for UserField {}
