extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
    .add(b'<').add(b'>').add(b'`');

pub fn construct_generic_url(domain: &str) -> String {
    format!("https://{}", domain)
}

pub fn construct_generic_search_url(domain: &str, query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT)
        .to_string();
    let generic_search_url = format!("https://{}/search?q={}", domain, encoded_query);
    generic_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_generic_search_url() {
        let fake_domain = "generic.com";
        let fake_query = "hello";
        assert_eq!(
            construct_generic_search_url(fake_domain, fake_query),
            "https://generic.com/search?q=hello"
        );
    }

    #[test]
    fn test_construct_generic_search_url_with_encoding() {
        let fake_domain = "generic.com";
        let fake_query = "hello world";
        assert_eq!(
            construct_generic_search_url(fake_domain, fake_query),
            "https://generic.com/search?q=hello%20world"
        );
    }
}