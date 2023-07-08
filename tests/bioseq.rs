use std::ops::Not;
use ncbi::{DataType, get_local_xml, parse_xml};
use ncbi::general::{DbTag, ObjectId};
use ncbi::seq::{BioSeq, SeqDesc};
use ncbi::seqfeat::{BinomialOrgName, BioSource, BioSourceGenome, BioSourceOrigin, OrgMod, OrgModSubType, OrgName, OrgNameChoice, OrgRef, SubSource, SubSourceSubType};
use ncbi::seqloc::SeqId;
use ncbi::seqset::{BioSeqSet, SeqEntry};

const DATA1: &str = "tests/data/2519734237.xml";

fn get_bioseq(path: &str) -> BioSeq {
    let set = get_seq_set(path);
    let entry = set.seq_set.get(0).unwrap();
    match entry {
        SeqEntry::Seq(data) => return data.clone(),
        _ => panic!("Entry is not Bioseq")
    }
}

fn get_seq_set(path: &str) -> BioSeqSet {
    let data = get_local_xml(path);
    let parsed = parse_xml(data.as_str()).unwrap();
    if let DataType::BioSeqSet(set) = parsed {
        return set
    }
    else {
        panic!("No Bioseq set found")
    }
}

#[test]
fn has_seq_set() {
    let data = get_seq_set(DATA1);
    assert!(data.seq_set.is_empty().not());
    assert_eq!(data.seq_set.len(), 1)
}

#[test]
/// Variant is SeqEntry::seq
fn parse_seq() {
    let data = get_seq_set(DATA1);
    let entry = data.seq_set.get(0).unwrap();
    match entry {
        SeqEntry::Seq(_) => assert!(true),
        _ => assert!(false)
    }
}

#[test]
fn parse_bioseq_id() {
    let bioseq = get_bioseq(DATA1);

    assert!(bioseq.id.is_empty().not());
    assert_eq!(bioseq.id.len(), 3);
    for id in bioseq.id.iter() {
        match id {
            SeqId::General(tag) => {
                assert_eq!(tag.db.as_str(), "WGS:NZ_JARQWN01");
                if let ObjectId::Str(s) = &tag.tag {
                    assert_eq!(s.as_str(), "NODE_24_length_86489_cov_60.972353")
                }
                else {
                    assert!(false);
                }
            },
            SeqId::Other(text) => {
                assert_eq!(text.accession.as_ref().unwrap().as_str(), "NZ_JARQWN010000024");
                assert_eq!(*text.version.as_ref().unwrap(), 1);
                assert!(text.name.is_none());
                assert!(text.release.is_none());
            }
            SeqId::Gi(gi) => assert_eq!(*gi, 2519734237),
            _ => ()
        }
    }
}

#[test]
fn parse_bioseq_descr() {
    let bioseq = get_bioseq(DATA1);

    assert!(bioseq.descr.is_some());
    // only BioSource should be parsed
    assert_eq!(bioseq.descr.unwrap().len(), 1);
}

#[test]
fn parse_bioseq_descr_source() {
    let bioseq = get_bioseq(DATA1);

    let subtype = vec![
        SubSource {
            subtype: SubSourceSubType::Country,
            name: "Australia".to_string(),
            attrib: None,
        },
        SubSource {
            subtype: SubSourceSubType::IsolationSource,
            name: "RESIDENTIAL AGED CARE FACILITY".to_string(),
            attrib: None,
        },
        SubSource {
            subtype: SubSourceSubType::LatLon,
            name: "34.9285 S 138.6007 E".to_string(),
            attrib: None,
        },
        SubSource {
            subtype: SubSourceSubType::CollectionDate,
            name: "2019".to_string(),
            attrib: None,
        },
        SubSource {
            subtype: SubSourceSubType::CollectedBy,
            name: "UNIVERSITY OF SOUTH AUSTRALIA".to_string(),
            attrib: None,
        },
    ].into();
    let expected = BioSource {
        genome: BioSourceGenome::Genomic,
        org: OrgRef {
            taxname: "Klebsiella pneumoniae".to_string().into(),
            db: vec![DbTag { db: "taxon".to_string(), tag: ObjectId::Id(573) }].into(),
            orgname: OrgName {
                name: OrgNameChoice::Binomial(BinomialOrgName {
                    genus: "Klebsiella".to_string(),
                    species: "pneumoniae".to_string().into(),
                    subspecies: None,
                }).into(),
                attrib: "specified".to_string().into(),
                r#mod: vec![OrgMod {
                    subtype: OrgModSubType::Strain,
                    subname: "A922".to_string(),
                    attrib: None,
                }].into(),
                lineage: "Bacteria; Pseudomonadota; Gammaproteobacteria; Enterobacterales; Enterobacteriaceae; Klebsiella/Raoultella group; Klebsiella"
                    .to_string()
                    .into(),
                gcode: 11.into(),
                mgcode: None,
                div: "BCT".to_string().into(),
                pgcode: None,
            }.into(),
            ..OrgRef::default()
        },
        subtype,
        ..BioSource::default()
    };
    for entry in bioseq.descr.unwrap().iter() {
        if let SeqDesc::Source(source) = entry {
            assert_eq!(*source, expected)
        }
    }
}

#[test]
#[ignore]
fn parse_bioseq_inst() {
    let bioseq = get_bioseq(DATA1);
    assert!(bioseq.inst.is_some());
}

#[test]
#[ignore]
fn parse_bioseq_annot() {
    let bioseq = get_bioseq(DATA1);
    assert!(bioseq.annot.is_some());
}
