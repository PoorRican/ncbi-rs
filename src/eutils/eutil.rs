//! Common methods for Entrez EUtils

use reqwest::Url;
use crate::EntrezDb;

pub trait EUtil {
    #[must_use]
    /// Constructor for builder pattern
    fn new(db: EntrezDb) -> Self where Self: Sized;

    /// Takes fields (from implementations) and assembles the URL to access
    ///
    /// This is used by [`Self::get()`]
    fn build_url(&self) -> Url;

    /// Perform a non-synchronous GET to Entrez
    fn get(&self) -> String {
        reqwest::blocking::get(self.build_url())
            .expect("Could not get url")
            .text()
            .expect("Could not format text from response")
    }
}