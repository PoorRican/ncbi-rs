//! Publication common set
//!
//! Base class definitions for Publications of all sorts are defined here
//!
//! Adapted from ["pub.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/pub/pub.asn)

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use crate::biblio::{
    CitArt, CitBook, CitGen, CitJour, CitLet, CitPat, CitProc, CitSub, IdPat,
    PubMedId,
};
use crate::medline::MedlineEntry;
use serde::{Serialize, Deserialize};
use crate::{XMLElement, XMLElementVec};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum Pub {
    /// general or generic unparsed
    Gen(CitGen),

    /// submission
    Sub(CitSub),

    Medline(MedlineEntry),

    /// medline uid
    Muid(u64),
    Article(CitArt),
    Journal(CitJour),
    Book(CitBook),

    /// proceedings of a meeting
    Proc(CitProc),

    Patent(CitPat),

    /// identify a patent
    PatId(IdPat),

    /// manuscript, thesis, or letter
    Man(CitLet),

    /// to cite a variety of ways
    Equiv(PubEquiv),

    /// PubMedId
    PmId(PubMedId),
}

impl XMLElement for Pub {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Pub")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        // variants
        let sub_element = BytesStart::new("Pub_sub");
        let gen_element = BytesStart::new("Pub_gen");

        loop {
            match reader.read_event().unwrap() {
                Event::Start(e) => {
                    let name = e.name();

                    if name == sub_element.name() {
                        return Pub::Sub(
                            CitSub::from_reader(reader)
                                .unwrap()
                        ).into()
                    }
                    else if name == gen_element.name() {
                        return Pub::Gen(
                            CitGen::from_reader(reader)
                                .unwrap()
                        ).into()
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
impl XMLElementVec for Pub {}

pub type PubEquiv = Vec<Pub>;

impl XMLElement for PubEquiv {
    fn start_bytes() -> BytesStart<'static> {
        BytesStart::new("Pub-equiv")
    }

    fn from_reader(reader: &mut Reader<&[u8]>) -> Option<Self> where Self: Sized {
        return Pub::vec_from_reader(reader, Self::start_bytes().to_end()).into()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all="lowercase")]
pub enum PubSet {
    Pub(Vec<Pub>),
    Medline(Vec<MedlineEntry>),
    Article(Vec<CitArt>),
    Journal(Vec<CitJour>),
    Book(Vec<CitBook>),
    Proc(Vec<CitProc>),
    Patent(Vec<CitPat>),
}
