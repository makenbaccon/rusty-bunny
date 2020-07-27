use crate::utils::generic;

pub fn construct_twitter_url(query: &str) -> String {
    if query == "tw" {
        // Check if it's just the short cut to twitter
        let twitter_dotcom = generic::construct_generic_url("twitter.com");
        twitter_dotcom.to_string()
    } else if &query[..4] == "tw @" {
        // Check if it looks like a Twitter profile
        construct_twitter_profile_url(&query[4..])
    } else {
        // Assume the other match is "tw sometext"
        // and search on Twitter
        construct_twitter_search_url(&query[3..])
    }
}

pub fn construct_twitter_profile_url(query: &str) -> String {
    format!("https://twitter.com/{}", query)
}

pub fn construct_twitter_search_url(query: &str) -> String {
    let twitter_search_url = generic::construct_generic_search_url("twitter.com", query);
    twitter_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_twitter_url() {
        let fake_query = "tw";
        assert_eq!(construct_twitter_url(fake_query),
        "https://twitter.com");
    }

    #[test]
    fn test_construct_twitter_url_query() {
        let fake_query = "tw hello world";
        assert_eq!(construct_twitter_url(fake_query),
        "https://twitter.com/search?q=hello%20world");
    }

    #[test]
    fn test_construct_twitter_url_profile() {
        let fake_query = "tw @fbOpenSource";
        assert_eq!(construct_twitter_url(fake_query),
        "https://twitter.com/fbOpenSource");
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        let fake_profile = "jsjoeio";
        assert_eq!(construct_twitter_profile_url(fake_profile),
        "https://twitter.com/jsjoeio");
    }

    #[test]
    fn test_construct_twitter_search_url() {
        let fake_query = "hello world";
        assert_eq!(construct_twitter_search_url(fake_query),
        "https://twitter.com/search?q=hello%20world");
    }

}