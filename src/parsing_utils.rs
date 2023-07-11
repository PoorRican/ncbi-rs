use std::ops::{AddAssign, Deref};
use atoi::{atoi, FromRadix10Checked, FromRadix10SignedChecked};
use num::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, Num, One, Zero};
use num::traits::{NumAssign, NumOps};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
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

fn parse_num<T>(text: &[u8]) -> T
where
    T: FromRadix10SignedChecked
{
    atoi::<T>(text.as_ref()).expect("Conversion error")
}

fn parse_text(text: &[u8]) -> String {
    text.escape_ascii().to_string()
}

pub fn get_next_num<T>(reader: &mut Reader<&[u8]>) -> T
where
    T: FromRadix10SignedChecked {
    if let Event::Text(text) = reader.read_event().unwrap() {
        parse_num(text.deref())
    }
    else {
        panic!("Incorrectly formatted number");
    }
}

pub fn get_next_text(reader: &mut Reader<&[u8]>) -> Option<String> {
    if let Event::Text(text) = reader.read_event().unwrap() {
        return parse_text(text.deref()).into()
    }
    return None
}

pub fn bad_element_formatting(element: &BytesStart) {
    panic!("Incorrectly formatted element {}", parse_text(element.name().0));
}

pub fn get_vec_text(reader: &mut Reader<&[u8]>, end: &BytesEnd) -> Vec<String>
{
    let mut items = Vec::new();
    loop {
        match reader.read_event().unwrap() {
                Event::Text(text) => {
                    let text =
                        parse_text(text.deref())
                            .trim()
                            .to_string();
                    if !(text == "\\\\n" || text.is_empty()) {
                        items.push(text)
                    }
                },
            Event::End(e) => {
                if e.name() == end.name() {
                    return items
                }
            }
            _ => ()
        }
    }
}

pub fn get_vec_num<T>(reader: &mut Reader<&[u8]>, end: &BytesEnd) -> Vec<T>
    where
        T: FromRadix10SignedChecked,
{
    let mut nums = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Text(text) => nums.push(parse_num(text.deref())),
            Event::End(e) => {
                if e.name() == end.name() {
                    return nums
                }
            }
            _ => ()
        }
    }
}

pub fn get_vec<T, F>(reader: &mut Reader<&[u8]>, item_element: &BytesStart, parser: &F, end: &BytesEnd) -> Vec<T>
    where
        F: Fn(&mut Reader<&[u8]>) -> Option<T>
{
    let mut items = Vec::new();
    loop {
        match reader.read_event().unwrap() {
            Event::Start(e) => {
                if e.name() == item_element.name() {
                    items.push(parser(reader).unwrap());
                }
            }
            Event::End(e) => {
                if e.name() == end.name() {
                    return items
                }
            }
            _ => ()
        }
    }
}