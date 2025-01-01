//! General Data Elements
//!
//! As per [general.asn](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/asn_spec/general.asn.html)

use crate::parsing::{read_vec_node, read_int, read_node, read_real, read_string, read_vec_int_unchecked, read_vec_str_unchecked, UnexpectedTags};
use crate::parsing::{XmlNode, XmlVecNode};
use quick_xml::events::{BytesStart, Event};
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

impl DateStd {
    pub fn new_from_ymd<T>(year: u16, month: T, day: T) -> Self
    where T: Into<Option<u8>>{
        Self {
            year,
            month: month.into(),
            day: day.into(),
            ..Self::default()
        }
    }

    pub fn new_from_ymd_hms<T>(year: u16, month: T, day: T, hour: T, minute: T, second: T) -> Self
    where T: Into<Option<u8>>{
        Self {
            year,
            month: month.into(),
            day: day.into(),
            hour: hour.into(),
            minute: minute.into(),
            second: second.into(),
            ..Self::default()
        }
    }
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
        let season_element = BytesStart::new("Date-std_season");
        let hour_element = BytesStart::new("Date-std_hour");
        let minute_element = BytesStart::new("Date-std_minute");
        let second_element = BytesStart::new("Date-std_second");

        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == year_element.name() {
                        date.year = read_int(reader).unwrap();
                    } else if name == month_element.name() {
                        date.month = read_int(reader);
                    } else if name == day_element.name() {
                        date.day = read_int(reader);
                    } else if name == season_element.name() {
                        date.season = read_string(reader);
                    } else if name == hour_element.name() {
                        date.hour = read_int(reader);
                    } else if name == minute_element.name() {
                        date.minute = read_int(reader);
                    } else if name == second_element.name() {
                        date.second = read_int(reader);
                    } else if name != Self::start_bytes().name() {
                        forbidden.check(&name);
                    }
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

                    if name == db_element.name() {
                        tag.db = read_string(reader).unwrap();
                    } else if name == tag_element.name() {
                        tag.tag = read_node(reader).unwrap();
                    }
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

        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == last_element.name() {
                        name_std.last = read_string(reader).unwrap();
                    } else if name == first_element.name() {
                        name_std.first = read_string(reader);
                    } else if name == initials_element.name() {
                        name_std.initials = read_string(reader);
                    } else if name != Self::start_bytes().name() {
                        forbidden.check(&name);
                    }
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

                    if name == class_element.name() {
                        object.class = read_string(reader);
                    } else if name == type_element.name() {
                        object.r#type = read_node(reader).unwrap();
                    } else if name == data_element.name() {
                        object.data = read_vec_node(reader, data_element.to_end());
                    }
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
        let objects_element = BytesStart::new("User-field_data_objects");

        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == str_element.name() {
                        return Self::Str(read_string(reader).unwrap()).into();
                    } else if name == int_element.name() {
                        return Self::Int(read_int::<i64>(reader).unwrap()).into();
                    } else if name == real_element.name() {
                        return Self::Real(read_real(reader).unwrap()).into()
                    } else if name == bool_element.name() {
                        unimplemented!()
                    } else if name == object_element.name() {
                        return Self::Object(read_node(reader).unwrap()).into();
                    } else if name == strs_element.name() {
                        return Self::Strs(read_vec_str_unchecked(reader, &strs_element.to_end())).into();
                    } else if name == ints_element.name() {
                        return Self::Ints(read_vec_int_unchecked(reader, &ints_element.to_end())).into();
                    } else if name == reals_element.name() {
                        return Self::Reals(read_vec_str_unchecked(reader, &reals_element.to_end())).into();
                    } else if name == fields_element.name() {
                        return Self::Fields(read_vec_node(reader, fields_element.to_end())).into()
                    } else if name == BytesStart::new("User-field").name() {
                        return Self::Fields(read_vec_node(reader, BytesStart::new("User-field").to_end())).into()
                    } else if name == objects_element.name() {
                        return Self::Objects(read_vec_node(reader, objects_element.to_end())).into()
                    } else if name != BytesStart::new("User-field_label").name() {
                        forbidden.check(&name);
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

        let forbidden = UnexpectedTags(&[]);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == label_element.name() {
                        field.label = read_node(reader).unwrap();
                    } else if name == data_element.name() {
                        field.data = read_node(reader).unwrap();
                    } else if name == num_element.name() {
                        field.num = read_int(reader);
                    } else if name != Self::start_bytes().name() {
                        forbidden.check(&name)
                    }
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


#[cfg(test)]
mod tests {
    use quick_xml::Reader;
    use crate::general::UserField;
    use crate::parsing::read_node;

    #[test]
    /// tests a bug where nested <User-field_data_fields> is not denoted by tag
    /// but is instead implied by an <User-field>
    fn test_fields_parsing_bug() {

        let xml = "<User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>ModelEvidence</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_object>\
            <User-object>\
            <User-object_type>\
            <Object-id>\
            <Object-id_str>ModelEvidence</Object-id_str>\
            </Object-id>\
            </User-object_type>\
            <User-object_data>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>Method</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_str>Protein Homology</User-field_data_str>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>SeedProtein</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_str>gi|378979015</User-field_data_str>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>SeedAlignPlacement</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_num>2</User-field_num>\
            <User-field_data>\
            <User-field_data_ints>\
            <User-field_data_ints_E>1232</User-field_data_ints_E>\
            <User-field_data_ints_E>2884</User-field_data_ints_E>\
            </User-field_data_ints>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>SeedCluster</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_str>927852</User-field_data_str>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>SeedProteinSource</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_str>Reference</User-field_data_str>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>phages_in_seed_cluster_ratio</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_real>0</User-field_data_real>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>Cluster</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_str>ArchId:11439550</User-field_data_str>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>ClusterName</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_str>iron ABC transporter permease</User-field_data_str>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>BestCDSIdentity</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_real>99.4555</User-field_data_real>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>BestHMMHit</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_str>gnl|HMM|NF012738.2</User-field_data_str>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>PureAbInitioConcurs</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_bool value=\"true\"/>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>ReferenceProteinSupport</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_bool value=\"true\"/>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>naming_ev_source</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_str>SPARCLE</User-field_data_str>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>prot_support</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_real>567.427</User-field_data_real>\
            </User-field_data>\
            </User-field>\
            <User-field>\
            <User-field_label>\
            <Object-id>\
            <Object-id_str>support</Object-id_str>\
            </Object-id>\
            </User-field_label>\
            <User-field_data>\
            <User-field_data_real>2565.61</User-field_data_real>\
            </User-field_data>\
            </User-field>\
            </User-object_data>\
            </User-object>\
            </User-field_data_object>\
            </User-field_data>\
            </User-field>";

        for xml in [xml] {
            let mut reader = Reader::from_str(xml);
            let node: UserField = read_node(&mut reader).unwrap();
            println!("{:?}", node)
        }
    }
}