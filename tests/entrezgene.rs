
use ncbi::entrezgene::{EntrezGene, EntrezGeneSet, EntrezgeneType, GeneTrackInner, GeneTrackStatus, XtraTermsInner};
use ncbi::{get_local_xml, parse_xml, DataType};
use ncbi::general::Date::Date;
use ncbi::general::DateStd;

const DATA1: &str = "tests/data/entrezgene.xml";

fn read_first_gene(path: &str) -> Result<EntrezGene, &'static str> {
    let set = get_gene_set(path)?;
    match set.get(0).to_owned() {
        Some(gene) => Ok(gene.clone()),
        None => Err("No EntrezGene found"),
    }
}

fn get_gene_set(path: &str) -> Result<EntrezGeneSet, &'static str> {
    let data = get_local_xml(path);
    let parsed = parse_xml(data.as_str()).unwrap();
    match parsed {
        DataType::EntrezGeneSet(set) => Ok(set),
        _ => Err("No EntrezGene set found"),
    }
}

#[test]
fn test_gene_set() {
    let set = get_gene_set(DATA1).unwrap();
    assert!(set.iter().count() > 0);
    println!("EntrezGeneSet has {} genes", set.iter().count());
}

/// test top-level struct
#[test]
fn test_entrez_gene() {
    let gene = read_first_gene(DATA1).unwrap();

    assert!(gene.track_info.is_some());
    assert_eq!(gene.r#type, EntrezgeneType::ProteinCoding);

    // ...
}

#[test]
fn test_gene_track() {
    let gene = read_first_gene(DATA1).unwrap();
    assert!(gene.track_info.is_some());

    let track = gene.track_info.unwrap();

    assert_eq!(track.len(), 1);

    let first_inner = track.get(0).unwrap();

    let expected_inner = GeneTrackInner {
        geneid: 7161,
        status: GeneTrackStatus::Live,
        create_date: Date(DateStd::new_from_ymd(1998, 8, 13)),
        update_date: Date(DateStd::new_from_ymd_hms(2024, 12, 10, 8, 46, 0)),
        ..Default::default()
    };

    assert_eq!(first_inner, &expected_inner);
}

#[test]
fn test_source() {
    let gene = read_first_gene(DATA1).unwrap();
    let source = gene.source;

    // Entrezgene_source ...
}

#[test]
fn test_gene() {
    let gene = read_first_gene(DATA1).unwrap();
    let gene_ref = gene.gene;

    // Entrezgene_gene ...
}

#[test]
fn test_prot() {
    let gene = read_first_gene(DATA1).unwrap();
    let prot = gene.prot;

    // Entrezgene_prot ...
}

#[test]
fn test_summary() {
    let gene = read_first_gene(DATA1).unwrap();
    let summary = gene.summary;

    // Entrezgene_summary ...
}

#[test]
fn test_location() {
    let gene = read_first_gene(DATA1).unwrap();
    let location = gene.location;

    // Entrezgene_location ...
}

#[test]
fn test_gene_source() {
    let gene = read_first_gene(DATA1).unwrap();
    let source = gene.source;

    // Entrezgene_source ...
}

#[test]
fn test_locus() {
    let gene = read_first_gene(DATA1).unwrap();
    let locus = gene.locus;

    // Entrezgene_locus ...
}

#[test]
fn test_properties() {
    let gene = read_first_gene(DATA1).unwrap();
    let properties = gene.properties;

    assert!(properties.is_some());
    // Entrezgene_properties ...
}

#[test]
fn test_comments() {
    let gene = read_first_gene(DATA1).unwrap();
    let comments = gene.comments;

    assert!(comments.is_some());
    // Entrezgene_comments ...
}

#[test]
fn test_unique_keys() {
    // Entrezgene_unique-keys ...
}

#[test]
fn test_xtra_index_terms() {
    let gene = read_first_gene(DATA1).unwrap();
    let xtra = gene.xtra_index_terms;

    let expected = "LOC7161";

    println!("{:?}", xtra);
    assert!(xtra.is_some_and(|x| x.as_str() == expected));
}

#[test]
fn test_xtra_properties() {
    let gene = read_first_gene(DATA1).unwrap();
    let xtra = gene.xtra_properties;

    let expected = XtraTermsInner {
        tag: "PROP".to_string(),
        value: "phenotype".to_string(),
    };

    assert!(xtra.is_some());
    assert_eq!(xtra.as_ref().unwrap().len(), 1);

    let first_term = xtra.as_ref().unwrap().get(0).unwrap();
    assert_eq!(
        first_term.get(0)
            .unwrap(),
        &expected);
}