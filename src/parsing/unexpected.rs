use quick_xml::events::BytesStart;
use quick_xml::name::QName;

/// Watchdog that guarantees all tags are being parsed.
///
/// If a particular tag is known about, but not yet implemented, it should be added to
/// the internal container. When a tag is encountered that is known, a warning will be
/// printed to stderr, however, when a tag that is not known is encountered, the program
/// panics. The intention is to not overlook any tag elements given by the eutils. The
/// internal store of unimplemented tags is a method of accountability.
///
/// Internal tags typically object fields and enum variants.
pub struct UnexpectedTags<'a>(pub &'a [BytesStart<'a>]);

impl UnexpectedTags<'_> {
    /// See if a given tag is accounted for
    pub fn check(&self, current: &QName) {
        let mut expected = false;
        for tag in self.0.iter() {
            if *current == tag.name() {
                expected = true;
                eprintln!("Encountered XML tag {}, which has not been implemented yet...", tag.escape_ascii().to_string())
            }
        }
        if !expected {
            panic!("Encountered {}, which has not been implemented yet...", current.0.escape_ascii().to_string());
        }
    }
}