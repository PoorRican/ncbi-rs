use std::ops::AddAssign;
use atoi::{atoi, FromRadix10Checked, FromRadix10SignedChecked};
use num::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Num, One, Zero};
use num::traits::{NumAssign, NumOps};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use crate::XMLElement;

pub fn try_field<T>(current: &QName, element: &BytesStart, field: &mut T, reader: &mut Reader<&[u8]>)
where
    T: From<String> {
    if *current == element.name() {
        let text = get_next_text(reader);
        if text.is_none() {
            bad_element_formatting(element);
        }
        else {
            *field = text.unwrap().into();
        }
    }
}

pub fn get_next_num<T>(reader: &mut Reader<&[u8]>) -> T
where
    T: FromRadix10SignedChecked {
    if let Event::Text(text) = reader.read_event().unwrap() {
        return atoi::<T>(text.as_ref()).expect("Conversion error");
    }
    else {
        panic!("Incorrectly formatted number");
    }
}

pub fn get_next_text(reader: &mut Reader<&[u8]>) -> Option<String> {
    if let Event::Text(text) = reader.read_event().unwrap() {
        return Some(text.escape_ascii().to_string())
    }
    return None
}

pub fn bad_element_formatting(element: &BytesStart) {
    panic!("Incorrectly formatted element {}", element.name().0.escape_ascii().to_string());
}