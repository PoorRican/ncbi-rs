use ncbi::biblio::{
    Affil, AffilStd, AuthList, AuthListNames, Author, CitGen, CitSub, CitSubMedium,
};
use ncbi::general::{
    Date, DateStd, DbTag, NameStd, ObjectId, PersonId, UserData, UserField, UserObject,
};
use ncbi::r#pub::Pub;
use ncbi::seq::{BioMol, BioSeq, MolInfo, MolTech, PubDesc, SeqDesc};
use ncbi::seqfeat::{
    BinomialOrgName, BioSource, BioSourceGenome, OrgMod, OrgModSubType, OrgName, OrgNameChoice,
    OrgRef, SubSource, SubSourceSubType,
};
use ncbi::seqloc::SeqId;
use ncbi::seqset::{BioSeqSet, SeqEntry};
use ncbi::{get_local_xml, parse_xml, DataType};
use std::ops::Not;

const DATA1: &str = "tests/data/2519734237.xml";

fn get_bioseq(path: &str) -> BioSeq {
    let set = get_seq_set(path);
    let entry = set.seq_set.get(0).unwrap();
    match entry {
        SeqEntry::Seq(data) => return data.clone(),
        _ => panic!("Entry is not Bioseq"),
    }
}

fn get_seq_set(path: &str) -> BioSeqSet {
    let data = get_local_xml(path);
    let parsed = parse_xml(data.as_str()).unwrap();
    if let DataType::BioSeqSet(set) = parsed {
        return set;
    } else {
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
        _ => assert!(false),
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
                } else {
                    assert!(false);
                }
            }
            SeqId::Other(text) => {
                assert_eq!(
                    text.accession.as_ref().unwrap().as_str(),
                    "NZ_JARQWN010000024"
                );
                assert_eq!(*text.version.as_ref().unwrap(), 1);
                assert!(text.name.is_none());
                assert!(text.release.is_none());
            }
            SeqId::Gi(gi) => assert_eq!(*gi, 2519734237),
            _ => (),
        }
    }
}

#[test]
fn parse_bioseq_descr() {
    let bioseq = get_bioseq(DATA1);

    assert!(bioseq.descr.is_some());
    assert_eq!(bioseq.descr.unwrap().len(), 10);
}

#[test]
fn parse_bioseq_descr_source() {
    let bioseq = get_bioseq(DATA1);

    let subtype = Some(vec![
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
    ]);
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
    let mut has_biosource = false;
    for entry in bioseq.descr.unwrap().iter() {
        if let SeqDesc::Source(source) = entry {
            assert_eq!(*source, expected);
            has_biosource = true;
        }
    }
    assert!(has_biosource);
}

#[test]
fn parse_bioseq_desc_molinfo() {
    let bioseq = get_bioseq(DATA1);

    let expected = MolInfo {
        bio_mol: BioMol::Genomic,
        tech: MolTech::WGS,
        tech_exp: None,
        completeness: Default::default(),
        gb_mol_type: None,
    };

    let mut has_mol_info = false;
    for entry in bioseq.descr.unwrap().iter() {
        if let SeqDesc::MolInfo(mol_info) = entry {
            assert_eq!(*mol_info, expected);
            has_mol_info = true;
        }
    }
    assert!(has_mol_info);
}

#[test]
fn parse_bioseq_desc_pub() {
    let bioseq = get_bioseq(DATA1);

    let authors = [
        ("Blaikie", "Jack", "J.M."),
        ("Sapula", "Sylvia", "S.A."),
        ("Amsalu", "Anteneh", "A."),
        ("Siderius", "Naomi", "N.L."),
        ("Hart", "Bradley", "B.J."),
        ("Venter", "Henrietta", "H."),
    ];
    let authors = authors.iter().map(|author| {
        Author::new(PersonId::Name(NameStd {
            last: author.0.to_string(),
            first: author.1.to_string().into(),
            initials: author.2.to_string().into(),
            ..NameStd::default()
        }))
    });

    let sub_authors = vec![Pub::Sub(CitSub {
        authors: AuthList {
            names: AuthListNames::Std(authors.clone().collect::<Vec<Author>>().into()),
            affil: Affil::Std(AffilStd {
                affil: "University of South Australia".to_string().into(),
                div: "Clinical Health Sciences".to_string().into(),
                city: "Adelaide".to_string().into(),
                sub: "SA".to_string().into(),
                country: "Australia".to_string().into(),
                street: "Cnr North Terrace and Frome Rd".to_string().into(),
                postal_code: "5001".to_string().into(),
                ..AffilStd::default()
            })
            .into(),
        },
        imp: None,
        medium: CitSubMedium::Paper,
        date: Date::Date(DateStd {
            year: 2023,
            month: 3.into(),
            day: 28.into(),
            ..DateStd::default()
        })
        .into(),
        descr: None,
    })];

    let expected1 = PubDesc {
        r#pub: sub_authors,
        ..PubDesc::default()
    };

    let expected2 = PubDesc {
        r#pub: vec![Pub::Gen(CitGen {
            cit: "Unpublished".to_string().into(),
            authors: AuthList {
                names: AuthListNames::Std(
                    authors.clone().collect::<Vec<Author>>().into()),
                affil: None,
            }.into(),
            title: "The resistome of Klebsiella pneumoniae complex isolates recovered from Residential Aged Care Facilities"
                .to_string()
                .into(),
            ..CitGen::default()
        })],
        ..PubDesc::default()
    };

    let mut has_pub = false;
    for entry in bioseq.descr.unwrap().iter() {
        if let SeqDesc::Pub(desc) = entry {
            let expected = match desc.r#pub.first().unwrap() {
                Pub::Gen(_) => &expected2,
                Pub::Sub(_) => &expected1,
                _ => panic!("Encountered unexpected type"),
            };
            assert_eq!(desc, expected);

            if has_pub == false {
                has_pub = true;
            }
        }
    }

    assert!(has_pub);
}

#[test]
fn parse_bioseq_desc_comment() {
    let bioseq = get_bioseq(DATA1);

    let expected = "The annotation was added by the NCBI Prokaryotic Genome Annotation Pipeline (PGAP). Information about PGAP can be found here: https://www.ncbi.nlm.nih.gov/genome/annotation_prok/".to_string();

    let mut has_comment = false;
    for entry in bioseq.descr.unwrap().iter() {
        if let SeqDesc::Comment(comment) = entry {
            assert_eq!(*comment, expected);
            has_comment = true;
        }
    }
    assert!(has_comment);
}

#[test]
fn parse_bioseq_desc_user() {
    let bioseq = get_bioseq(DATA1);

    let expected1 = UserObject {
        class: None,
        r#type: ObjectId::Str("DBLink".to_string()),
        data: vec![
            UserField {
                label: ObjectId::Str("BioSample".to_string()),
                num: 1.into(),
                data: UserData::Strs(vec!["SAMN33942939".to_string()]),
            },
            UserField {
                label: ObjectId::Str("BioProject".to_string()),
                num: 1.into(),
                data: UserData::Strs(vec!["PRJNA224116".to_string()]),
            },
            UserField {
                label: ObjectId::Str("Assembly".to_string()),
                num: 1.into(),
                data: UserData::Strs(vec!["GCF_030238925.1".to_string()]),
            },
        ],
    };
    let expected2 = UserObject {
        class: None,
        r#type: ObjectId::Str("StructuredComment".into()),
        data: vec![
            UserField {
                label: ObjectId::Str("StructuredCommentPrefix".to_string()),
                num: None,
                data: UserData::Str("##Genome-Annotation-Data-START##".to_string()),
            },
            UserField {
                label: ObjectId::Str("Annotation Provider".to_string()),
                num: None,
                data: UserData::Str("NCBI RefSeq".to_string()),
            },
            UserField {
                label: ObjectId::Str("Annotation Date".to_string()),
                num: None,
                data: UserData::Str("06/09/2023 17:06:50".to_string()),
            },
            UserField {
                label: ObjectId::Str("Annotation Pipeline".to_string()),
                num: None,
                data: UserData::Str(
                    "NCBI Prokaryotic Genome Annotation Pipeline (PGAP)".to_string(),
                ),
            },
            UserField {
                label: ObjectId::Str("Annotation Method".to_string()),
                num: None,
                data: UserData::Str("Best-placed reference protein set; GeneMarkS-2+".to_string()),
            },
            UserField {
                label: ObjectId::Str("Annotation Software revision".to_string()),
                num: None,
                data: UserData::Str("6.5".to_string()),
            },
            UserField {
                label: ObjectId::Str("Features Annotated".to_string()),
                num: None,
                data: UserData::Str("Gene; CDS; rRNA; tRNA; ncRNA".to_string()),
            },
            UserField {
                label: ObjectId::Str("Genes (total)".to_string()),
                num: None,
                data: UserData::Str("5,288".to_string()),
            },
            UserField {
                label: ObjectId::Str("CDSs (total)".to_string()),
                num: None,
                data: UserData::Str("5,202".to_string()),
            },
            UserField {
                label: ObjectId::Str("Genes (coding)".to_string()),
                num: None,
                data: UserData::Str("5,041".to_string()),
            },
            UserField {
                label: ObjectId::Str("CDSs (with protein)".to_string()),
                num: None,
                data: UserData::Str("5,041".to_string()),
            },
            UserField {
                label: ObjectId::Str("Genes (RNA)".to_string()),
                num: None,
                data: UserData::Str("86".to_string()),
            },
            UserField {
                label: ObjectId::Str("rRNAs".to_string()),
                num: None,
                data: UserData::Str("2, 3, 6 (5S, 16S, 23S)".to_string()),
            },
            UserField {
                label: ObjectId::Str("complete rRNAs".to_string()),
                num: None,
                data: UserData::Str("2 (5S)".to_string()),
            },
            UserField {
                label: ObjectId::Str("partial rRNAs".to_string()),
                num: None,
                data: UserData::Str("3, 6 (16S, 23S)".to_string()),
            },
            UserField {
                label: ObjectId::Str("tRNAs".to_string()),
                num: None,
                data: UserData::Str("64".to_string()),
            },
            UserField {
                label: ObjectId::Str("ncRNAs".to_string()),
                num: None,
                data: UserData::Str("11".to_string()),
            },
            UserField {
                label: ObjectId::Str("Pseudo Genes (total)".to_string()),
                num: None,
                data: UserData::Str("161".to_string()),
            },
            UserField {
                label: ObjectId::Str("CDSs (without protein)".to_string()),
                num: None,
                data: UserData::Str("161".to_string()),
            },
            UserField {
                label: ObjectId::Str("Pseudo Genes (ambiguous residues)".to_string()),
                num: None,
                data: UserData::Str("0 of 161".to_string()),
            },
            UserField {
                label: ObjectId::Str("Pseudo Genes (frameshifted)".to_string()),
                num: None,
                data: UserData::Str("59 of 161".to_string()),
            },
            UserField {
                label: ObjectId::Str("Pseudo Genes (incomplete)".to_string()),
                num: None,
                data: UserData::Str("107 of 161".to_string()),
            },
            UserField {
                label: ObjectId::Str("Pseudo Genes (internal stop)".to_string()),
                num: None,
                data: UserData::Str("25 of 161".to_string()),
            },
            UserField {
                label: ObjectId::Str("Pseudo Genes (multiple problems)".to_string()),
                num: None,
                data: UserData::Str("27 of 161".to_string()),
            },
            UserField {
                label: ObjectId::Str("StructuredCommentSuffix".to_string()),
                num: None,
                data: UserData::Str("##Genome-Annotation-Data-END##".to_string()),
            },
        ],
    };
    let expected3 = UserObject {
        class: None,
        r#type: ObjectId::Str("RefGeneTracking".to_string()),
        data: vec![
            UserField {
                label: ObjectId::Str("Status".to_string()),
                num: None,
                data: UserData::Str("PIPELINE".to_string()),
            },
            UserField {
                label: ObjectId::Str("IdenticalTo".to_string()),
                num: None,
                data: UserData::Fields(vec![UserField {
                    label: ObjectId::Id(0),
                    num: None,
                    data: UserData::Fields(vec![UserField {
                        label: ObjectId::Str("accession".to_string()),
                        num: None,
                        data: UserData::Str("JARQWN010000024.1".to_string()),
                    }]),
                }]),
            },
        ],
    };
    let expected4 = UserObject {
        class: None,
        r#type: ObjectId::Str("FeatureFetchPolicy".to_string()),
        data: vec![UserField {
            label: ObjectId::Str("Policy".to_string()),
            num: None,
            data: UserData::Str("OnlyNearFeatures".to_string()),
        }],
    };
    let expected5 = UserObject {
        class: None,
        r#type: ObjectId::Str("StructuredComment".to_string()),
        data: vec![
            UserField {
                label: ObjectId::Str("StructuredCommentPrefix".to_string()),
                num: None,
                data: UserData::Str("##Genome-Assembly-Data-START##".to_string()),
            },
            UserField {
                label: ObjectId::Str("Assembly Method".to_string()),
                num: None,
                data: UserData::Str("SPAdes v. 1".to_string()),
            },
            UserField {
                label: ObjectId::Str("Genome Representation".to_string()),
                num: None,
                data: UserData::Str("Full".to_string()),
            },
            UserField {
                label: ObjectId::Str("Expected Final Version".to_string()),
                num: None,
                data: UserData::Str("Yes".to_string()),
            },
            UserField {
                label: ObjectId::Str("Genome Coverage".to_string()),
                num: None,
                data: UserData::Str("100x".to_string()),
            },
            UserField {
                label: ObjectId::Str("Sequencing Technology".to_string()),
                num: None,
                data: UserData::Str("Illumina HiSeq".to_string()),
            },
            UserField {
                label: ObjectId::Str("StructuredCommentSuffix".to_string()),
                num: None,
                data: UserData::Str("##Genome-Assembly-Data-END##".to_string()),
            },
        ],
    };

    let expected = [expected1, expected2, expected3, expected4, expected5];

    let mut has_user_object = false;
    for entry in bioseq.descr.unwrap().iter() {
        if let SeqDesc::User(object) = entry {
            for exp in expected.iter() {
                if object.r#type == exp.r#type {
                    if object.r#type == ObjectId::Str("StructuredComment".to_string())
                        && object.data.first().unwrap() != exp.data.first().unwrap()
                    {
                        continue;
                    }
                    assert_eq!(object, exp);
                    has_user_object = true;
                }
            }
        }
    }
    assert!(has_user_object)
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
