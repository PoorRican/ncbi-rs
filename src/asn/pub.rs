//! Publication common set
//!
//! Base class definitions for Publications of all sorts are defined here
//!
//! Adapted from ["pub.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/pub/pub.asn)

use crate::biblio::{
    CitArt, CitBook, CitGen, CitJour, CitLet, CitPat, CitProc, CitSub, IdPat,
    PubMedId,
};
use crate::medline::MedlineEntry;
use std::collections::BTreeSet;

#[derive(PartialEq, Debug)]
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

pub type PubEquiv = BTreeSet<Pub>;

#[derive(PartialEq, Debug)]
pub enum PubSet {
    Pub(BTreeSet<Pub>),
    Medline(BTreeSet<MedlineEntry>),
    Article(BTreeSet<CitArt>),
    Journal(BTreeSet<CitJour>),
    Book(BTreeSet<CitBook>),
    Proc(BTreeSet<CitProc>),
    Patent(BTreeSet<CitPat>),
}
