//! General Data Elements
//!
//! As per [general.asn](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/asn_spec/general.asn.html)

#[derive(PartialEq, Debug)]
/// Model precise timestamp or an un-parsed string
///
/// The string form is a fall-back for when the input data cannot be parsed
/// into the standard date fields. It should only be used as a last resort to
/// accommodate old data, as it is impossible to compute or index on.
pub enum Date {
    Str(String),
    Date(DateStd),
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
/// Can tag or name anything
pub enum ObjectId {
    Id(u64),
    Str(String),
}

#[derive(PartialEq, Debug)]
/// Generalized for tagging
pub struct DbTag {
    /// name of database or system
    pub db: String,
    /// appropriate tag
    pub tag: ObjectId,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct Range {
    pub max: i64,
    pub min: i64,
}

#[derive(PartialEq, Debug)]
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

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
/// Communicate uncertainties in integer values
pub enum IntFuzz {
    /// plus or minus fixed amount
    PM(i64),
    
    Range(Range),
    Pct(i64),
    Lim(FuzzLimit),
    Alt(Vec<i64>),
}

#[derive(PartialEq, Debug)]
/// a general object for a user defined structured data item
///
/// used by [`SeqFeat`] and [`SeqDescr`]
pub struct UserObject {
    /// endeavor which designed this object
    pub class: Option<String>,
    /// type of object within class
    pub r#type: ObjectId,
    /// the object itself
    pub data: Vec<UserField>,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct UserField {
    /// field label
    pub label: ObjectId,
    /// required for strs, ints, reals, oss
    pub num: Option<i64>,
    /// field contents
    pub data: UserData,
}
