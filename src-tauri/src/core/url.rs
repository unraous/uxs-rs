/// This module provides functions for classifying URLs and determining if they require modification scripts.
use log::{warn};

/// Represents the type of a URL based on its structure and domain.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Course,
    Login,
    MainSpace,
    Mask,
    Other,
}

/// Returns the type of the given URL based on its structure and domain.
/// This function checks if the URL belongs to the "chaoxing.com" domain 
/// and classifies it.
pub fn classify(url: &tauri::Url) -> Type {
    let host = url.host_str().unwrap_or_default();

    if url.as_str() == "about:blank" {
        return Type::Mask;
    }

    if !host.ends_with("chaoxing.com") {
        warn!("检测到非超星域名URL: {}", url.as_str());
        return Type::Other;
    }

    let sub = host.split('.').next().unwrap_or_default();
    let result = match sub {
        "i" => Type::MainSpace,
        "mooc1" if url.path().starts_with("/mycourse/") => Type::Course,
        "passport2" if url.path().starts_with("/login") => Type::Login,
        _ => Type::Other,
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_urls() {
        let url = "https://i.chaoxing.com".parse().unwrap();
        assert_eq!(classify(&url), Type::MainSpace);
        
        let url = "https://mooc1.chaoxing.com/mycourse/...".parse().unwrap();
        assert_eq!(classify(&url), Type::Course);
        
        let url = "https://passport2.chaoxing.com/login?refer=https://www.chaoxing.com".parse().unwrap();
        assert_eq!(classify(&url), Type::Login);

        let url = "about:blank".parse().unwrap();
        assert_eq!(classify(&url), Type::Mask);
    }
}

