#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use quick_xml::Reader;
    use quick_xml::events::Event;

    use ncbi::entrezgene::{Entrezgene, EntrezgeneType};
    use ncbi::parsing::XmlNode;

    #[test]
    fn test_parse_entrezgene_tp73() {
        // Path to the test file
        let file_path = "tp73.genbank.xml";
        let mut file = File::open(file_path).expect("Error opening test file.");

        // Read file content into a string
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Error reading file content.");

        // Initialize XML reader with &[u8]
        let mut xml_reader = Reader::from_str(&content);
        xml_reader.trim_text(true);

        let mut buf = Vec::new();
        let mut entrezgene: Option<Entrezgene> = None;

        // Parse XML content
        loop {
            match xml_reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"Entrezgene" => {
                    // Parse the Entrezgene object
                    entrezgene = Entrezgene::from_reader(&mut xml_reader);
                }
                Ok(Event::Eof) => break, // End of file
                Err(e) => panic!("Error parsing XML file: {:?}", e),
                _ => (),
            }
            buf.clear();
        }

        // Check if parsing was successful
        assert!(entrezgene.is_some(), "The Entrezgene object was not parsed correctly.");

        let gene = entrezgene.unwrap();

        // Example assertions to verify parsed data
        assert_eq!(
            gene.gene.locus.as_deref(),
            Some("TP73"),
            "Gene locus is not 'TP73' as expected."
        );

        assert_eq!(
            gene.gene.desc.as_deref(),
            Some("tumor protein p73"),
            "Gene description is not 'tumor protein p73' as expected."
        );

        assert!(
            gene.track_info.is_some(),
            "Track information is missing from the parsed data."
        );

        assert_eq!(
            gene.r#type,
            EntrezgeneType::ProteinCoding,
            "Gene type is not 'ProteinCoding' as expected."
        );

        println!("Test successful: Entrezgene file was parsed correctly.");
    }
}
