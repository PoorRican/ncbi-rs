use reqwest::Url;

pub trait EUtil {
    fn build_url(&self) -> Url;
    fn get(&self) -> String {
        reqwest::blocking::get(self.build_url())
            .expect("Could not get url")
            .text()
            .expect("Could not format text from response")
    }
}