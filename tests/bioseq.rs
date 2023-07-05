extern crate reqwest;
extern crate quick_xml;

use quick_xml::de::from_str;
use quick_xml::Reader;
use ncbi::{build_fetch_url, build_search_url, EntrezDb, parse_response};
use ncbi::seqset::BioSeqSet;

#[test]
fn search_url() {
    let url = build_search_url(EntrezDb::Protein, "deaminase");
    println!("{}", url);
}

#[test]
fn test_protein() {
    let id = "2520667272";
    let url = build_fetch_url(EntrezDb::Protein, id, "native", "xml");
    println!("{}", url)
}

#[test]
fn build_url() {
    let url = build_fetch_url(EntrezDb::Nucleotide, "2519734237", "native", "xml");
    let response =
        reqwest::blocking::get(url)
            .unwrap()
            .text()
            .unwrap();
    parse_response(response.as_str()).unwrap();
}


#[test]
fn test_article_set() {
    let id = "37332098";
    let db = EntrezDb::PubMed;

    let url = build_fetch_url(db, id, "xml", "xml");
    println!("{}", url);
    let _ =
        reqwest::blocking::get(url)
            .unwrap()
            .text()
            .unwrap();
    //let expected = from_str(text.as_str()).unwrap();
    //assert!(expected.is_empty().not())
}