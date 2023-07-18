use reqwest::Url;
use crate::EntrezDb;

pub trait EUtil {
    fn new(db: EntrezDb) -> Self where Self: Sized;
    fn build_url(&self) -> Url;
    fn get(&self) -> String {
        reqwest::blocking::get(self.build_url())
            .expect("Could not get url")
            .text()
            .expect("Could not format text from response")
    }
}