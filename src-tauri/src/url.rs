/// This module provides functions for classifying URLs and determining if they require modification scripts.
use log::{warn};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UrlType {
    Course,
    Login,
    MainSpace,
    Other,
}

/// Returns the type of the given URL based on its structure and domain.
/// This function checks if the URL belongs to the "chaoxing.com" domain 
/// and classifies it.
/// # Examples
/// ```
/// use uxs_lib::classify_url;
/// use uxs_lib::UrlType;
/// 
/// let url = "https://i.chaoxing.com".parse().unwrap();
/// assert_eq!(classify_url(&url), UrlType::MainSpace);
/// let url = "https://mooc1.chaoxing.com/mycourse/example".parse().unwrap();
/// assert_eq!(classify_url(&url), UrlType::Course);
/// let url = "https://passport2.chaoxing.com/login?refer=https://www.chaoxing.com".parse().unwrap();
/// assert_eq!(classify_url(&url), UrlType::Login);
/// let url = "https://www.chaoxing.com".parse().unwrap();
/// assert_eq!(classify_url(&url), UrlType::Other);
/// let url = "https://www.example.com".parse().unwrap();
/// assert_eq!(classify_url(&url), UrlType::Other);
/// ```
pub fn classify_url(url: &tauri::Url) -> UrlType {
    let host = url.host_str().unwrap_or_default();

    if !host.ends_with("chaoxing.com") {
        warn!("检测到非超星域名URL: {}", url.as_str());
        return UrlType::Other;
    }

    let subdomain = host.split('.').next().unwrap_or_default();
    let result = match subdomain {
        "i" => UrlType::MainSpace,
        "mooc1" if url.path().starts_with("/mycourse/") => UrlType::Course,
        "passport2" if url.path().starts_with("/login") => UrlType::Login,
        _ => UrlType::Other,
    };

    result
}

/// Returns the modification script for the given URL type, if applicable.
pub fn get_modification_script(urltype: UrlType) -> Option<&'static str> {
    match urltype {
        UrlType::Course => None,
        UrlType::MainSpace => Some(include_str!("./scripts/modify-targets.js")),
        UrlType::Login => Some(include_str!("./scripts/click-auto-login.js")),
        UrlType::Other => None,
    }
}