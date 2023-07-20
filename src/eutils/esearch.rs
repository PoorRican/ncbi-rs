use quick_xml::Reader;
use reqwest::Url;
use crate::{EntrezDb, ESearchResult};
use crate::eutils::BASE;
use crate::eutils::eutil::EUtil;
use crate::parsing::XmlNode;

#[derive(Debug, PartialEq)]
/// Enables searching of NCBI databases
///
/// What is returned by this API is a list of ID's that
/// are used to get the biological data associated with the query.
///
/// Search terms are really meant to be fed via a UI and not
/// so much hardcoded as Rust would not be the most practical choice.
///
/// # Examples
///
/// ### A basic search query:
///
/// ```
/// use ncbi::{EntrezDb, ESearch, EUtil};
/// let results = ESearch::new(EntrezDb::Gene)
///                     .term("decarboxylase")
///                     .search();
/// assert_eq!(results.unwrap().ret_max(), 20);
/// ```
///
/// ### A bulk search:
///
/// ```
/// use ncbi::{EntrezDb, ESearch, EUtil};
/// let results = ESearch::new(EntrezDb::Protein)
///                     .term("decarboxylase")
///                     .max(10000)
///                     .search();
/// assert_eq!(results.unwrap().ret_max(), 10000);
/// ```
///
/// ### A paginated search:
///
/// ```
/// use ncbi::{EntrezDb, ESearch, EUtil};
/// let results = ESearch::new(EntrezDb::Protein)
///                     .term("decarboxylase")
///                     .start(1000)
///                     .max(500)
///                     .search();
/// assert_eq!(results.as_ref().unwrap().ret_start(), 1000);
/// assert_eq!(results.as_ref().unwrap().ret_max(), 500);
/// ```
///
/// ### Using NCBI syntax
///
/// ```
/// use ncbi::{EntrezDb, ESearch, EUtil};
///
/// let term = "\"Escherichia coli\"[Organism] AND (bacteria[filter]\
/// AND biomol_genomic[PROP] AND plasmid[filter])";
///
/// let results = ESearch::new(EntrezDb::Protein)
///                     .term(term)
///                     .start(1000)
///                     .max(500)
///                     .search();
/// assert_eq!(results.as_ref().unwrap().ret_start(), 1000);
/// assert_eq!(results.as_ref().unwrap().ret_max(), 500);
/// ```
pub struct ESearch<'a> {
    db: EntrezDb,
    term: Option<&'a str>,
    ret_start: Option<usize>,
    ret_max: Option<usize>,
    field: Option<&'a str>,
}

impl<'a> ESearch<'a> {
    const ENDPOINT: &'static str = "esearch.fcgi?";

    /// Builder method for search "term"
    ///
    /// Those familiar with the NCBI query syntax are able to
    /// pass a full query string as the term.
    ///
    /// # Example
    ///
    /// ```
    /// use ncbi::{ESearch, EntrezDb, EUtil};
    ///
    /// let term = "enzyme modification transport fungi";
    /// let builder = ESearch::new(EntrezDb::Genome)
    ///                 .term(term);
    /// let url = builder.build_url();
    /// assert!(url.as_str()
    ///            .contains(term.replace(' ', "+")
    ///            .as_str()))
    /// ```
    pub fn term(mut self, term: &'a str) -> Self {
        self.term = Some(term);
        self
    }

    /// Builder method for "retstart"
    ///
    /// # Example
    ///
    /// ```
    /// use ncbi::{ESearch, EntrezDb, EUtil};
    ///
    /// let builder = ESearch::new(EntrezDb::Genome)
    ///                 .term("")
    ///                 .start(400);
    /// let url = builder.build_url();
    /// assert!(url.as_str()
    ///            .contains("?retstart=400"))
    /// ```
    pub fn start(mut self, ret_start: usize) -> Self {
        self.ret_start = Some(ret_start);
        self
    }

    /// Builder method for "retmax"
    ///
    /// # Example
    /// ```
    /// use ncbi::{ESearch, EntrezDb, EUtil};
    ///
    /// let builder = ESearch::new(EntrezDb::Genome)
    ///                 .term("")
    ///                 .max(400);
    /// let url = builder.build_url();
    /// assert!(url.as_str()
    ///            .contains("&retmax=400"))
    /// ```
    pub fn max(mut self, ret_max: usize) -> Self {
        self.ret_max = Some(ret_max);
        self
    }

    /// Builder method for "field"
    ///
    /// This string restricts the search to an entire field/column of
    /// the database. The available values for these fields vary
    /// across databases.
    ///
    /// ## Note
    ///
    /// This value is poorly documented, but those familiar with
    /// the NCBI query syntax would be able to use this term.
    ///
    /// # Example
    ///
    /// ```
    /// use ncbi::{ESearch, EntrezDb, EUtil};
    ///
    /// // this is a poor example of this field
    /// let builder = ESearch::new(EntrezDb::Genome)
    ///                 .term("bacteria")
    ///                 .field("organism");
    /// let url = builder.build_url();
    /// assert!(url.as_str()
    ///            .contains("&field=organism"))
    /// ```
    pub fn field(mut self, field: &'a str) -> Self {
        self.field = Some(field);
        self
    }

    #[must_use]
    /// Executes search query and returns [`ESearchResult`]
    ///
    /// # Example
    ///
    /// A simple query for the term "bacteria":
    /// ```
    /// use ncbi::{ESearch, EntrezDb, EUtil, ESearchResult};
    ///
    /// let results = ESearch::new(EntrezDb::Genome)
    ///                 .term("bacteria")
    ///                 .search();
    /// assert!(results.unwrap().count() >= 1);
    /// ```
    pub fn search(&self) -> Option<ESearchResult> {
        let response = self.get();
        let mut reader = Reader::from_str(response.as_str());
        ESearchResult::from_reader(&mut reader)
    }
}

impl EUtil for ESearch<'_> {
    fn new(db: EntrezDb) -> Self {
        Self {
            db,
            term: None,
            ret_start: None,
            ret_max: None,
            field: None
        }
    }

    fn build_url(&self) -> Url {
        let mut url = Url::parse(BASE).unwrap();
        url = url.join(Self::ENDPOINT).unwrap();

        let mut queries = url.query_pairs_mut();
        queries.append_pair("db", self.db.as_str())
            .append_pair("term", self.term.expect("No term given!"))
            .append_pair("rettype", "xml")
            .append_pair("retmode", "xml")
            .append_pair("sort", "relevance");

        // optional query refinements
        if let Some(ret_start) = self.ret_start {
            queries.append_pair("retstart", format!("{}", ret_start).as_str());
        }
        if let Some(ret_max) = self.ret_max {
            queries.append_pair("retmax", format!("{}", ret_max).as_str());
        }
        if let Some(field) = self.field {
            queries.append_pair("field", field);
        }
        drop(queries);

        url
    }
}


#[cfg(test)]
mod tests {
    use reqwest::Url;
    use crate::{EntrezDb, ESearch, EUtil};
    use crate::eutils::BASE;

    #[test]
    fn test_builder() {
        let query = "?db=genome&term=deaminase&rettype=xml&retmode=xml&sort=relevance";

        let mut expected = Url::parse(BASE).unwrap();
        expected = expected
            .join(ESearch::ENDPOINT)
            .unwrap()
            .join(query)
            .unwrap();


        let builder = ESearch::new(
            EntrezDb::Genome
        ).term("deaminase");

        assert_eq!(expected, builder.build_url())
    }

    #[test]
    fn test_builder_retstart() {
        let query = "?db=genome&term=deaminase&rettype=xml&retmode=xml&sort=relevance&retstart=100";

        let mut expected = Url::parse(BASE).unwrap();
        expected = expected
            .join(ESearch::ENDPOINT)
            .unwrap()
            .join(query)
            .unwrap();


        let builder = ESearch::new(
            EntrezDb::Genome)
            .term("deaminase")
            .start(100);

        assert_eq!(expected, builder.build_url())
    }

    #[test]
    fn test_builder_retmax() {
        let query = "?db=genome&term=deaminase&rettype=xml&retmode=xml&sort=relevance&retmax=100";

        let mut expected = Url::parse(BASE).unwrap();
        expected = expected
            .join(ESearch::ENDPOINT)
            .unwrap()
            .join(query)
            .unwrap();


        let builder = ESearch::new(
            EntrezDb::Genome)
            .term("deaminase")
            .max(100);

        assert_eq!(expected, builder.build_url())
    }

    #[test]
    fn test_builder_field() {
        let query = "?db=genome&term=deaminase&rettype=xml&retmode=xml&sort=relevance&field=title";

        let mut expected = Url::parse(BASE).unwrap();
        expected = expected
            .join(ESearch::ENDPOINT)
            .unwrap()
            .join(query)
            .unwrap();


        let builder = ESearch::new(
            EntrezDb::Genome)
            .term("deaminase")
            .field("title");

        assert_eq!(expected, builder.build_url())
    }

    #[test]
    fn test_builder_combination() {
        let query = "?db=genome&term=deaminase&rettype=xml&retmode=xml&sort=relevance&retstart=1000&retmax=1000&field=title";

        let mut expected = Url::parse(BASE).unwrap();
        expected = expected
            .join(ESearch::ENDPOINT)
            .unwrap()
            .join(query)
            .unwrap();


        let builder = ESearch::new(
            EntrezDb::Genome)
            .term("deaminase")
            .start(1000)
            .max(1000)
            .field("title");

        assert_eq!(expected, builder.build_url())
    }

}