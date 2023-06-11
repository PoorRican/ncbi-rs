//! Publication common set
//!
//! Base class definitions for Publications of all sorts are defined here
//!
//! Adapted from ["pub.asn"](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/CPP_DOC/lxr/source/src/objects/pub/pub.asn)

use std::collections::HashSet;
use crate::asn::{CitArt, CitBook, CitGen, CitJour, CitLet, CitPat, CitProc, CitSub, IdPat, PubMedId, MedlineEntry};

pub enum Pub {
    Gen(CitGen),
    Sub(CitSub),
    Medline(MedlineEntry),
    Muid(u64),
    Article(CitArt),
    Journal(CitJour),
    Book(CitBook),
    Proc(CitProc),
    Patent(CitPat),
    PatId(IdPat),
    Man(CitLet),
    Equiv(PubEquiv),
    PmId(PubMedId)
}

pub type PubEquiv = HashSet<Pub>;

pub enum PubSet {
    Pub(HashSet<Pub>),
    Medline(HashSet<MedlineEntry>),
    Article(HashSet<CitArt>),
    Journal(HashSet<CitJour>),
    Book(HashSet<CitBook>),
    Proc(HashSet<CitProc>),
    Patent(HashSet<CitPat>),
}