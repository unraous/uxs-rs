use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    #[serde(rename = "题号")]
    pub id: String,
    #[serde(rename = "题型")]
    pub qtype: String,
    #[serde(rename = "题干")]
    pub stem: String,
    #[serde(rename = "选项")]
    pub options: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct QuestionsRaw {
    pub questions: Vec<Question>,
    pub font: Option<Vec<u8>>,
}

fn extract_font(html: &str) -> Option<Vec<u8>> {
    let re = Regex::new(
        r#"@font-face\s*\{[^}]*font-family\s*:\s*['\"]font-cxsecret['\"][^}]*src\s*:\s*url\(\s*['\"]data:application/font-ttf[^,]*,([^'\"\\)]+)['\"]\s*\)"#,
    ).unwrap();

    re.captures(html).and_then(|cap| {
        let b64str = cap.get(1)?.as_str();
        STANDARD.decode(b64str).ok()
    })
}

fn trim(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ").trim().to_string()
}

fn select_options(elem: &scraper::element_ref::ElementRef) -> Vec<String> {
    Selector::parse("li")
        .ok()
        .map(|sel| {
            elem
                .select(&sel)
                .map(|li| trim(&li.text().collect::<String>()))
                .filter(|t| !t.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn extract_questions(html: &str) -> Vec<Question> {
    let document = Html::parse_document(html);
    
    let select_text = |elem: &scraper::element_ref::ElementRef, selector: &str| {
        Selector::parse(selector)
            .ok()
            .and_then(|sel| elem.select(&sel).next())
            .map(|e| trim(&e.text().collect::<String>()))
    };

    document
        .select(&Selector::parse("div.TiMu.newTiMu").unwrap())
        .enumerate()
        .map(|(idx, qelem)| {
            Question {
                id: select_text(&qelem, "i.fl").unwrap_or_else(|| (idx + 1).to_string()),
                qtype: select_text(&qelem, "span.newZy_TItle").unwrap_or_default(),
                stem: select_text(&qelem, "div.fontLabel").unwrap_or_default(),
                options: select_options(&qelem),
            }
        })
        .collect()
}

impl QuestionsRaw {
    pub fn new(html: &str) -> Result<Self> {
        Ok(Self {
            questions: extract_questions(html),
            font: extract_font(html),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction() {
        let html = include_str!("../../../tests/assets/course-page/webpage.html");
        let raw = QuestionsRaw::new(html).expect("Failed to parse HTML");

        let expected_json: serde_json::Value = serde_json::from_str(
            include_str!("../../../tests/assets/course-page/questions.json")
        ).expect("Invalid expected questions.json");
        let actual_json = serde_json::to_value(&raw.questions).expect("Serialize parsed questions failed");
        assert_eq!(expected_json, actual_json, "Parsed questions differ from questions.json");

        let expected_ttf = include_bytes!("../../../tests/assets/course-page/cxs-font.ttf");
        let actual_ttf = raw.font.as_ref().expect("No font extracted from HTML");
        assert_eq!(expected_ttf, actual_ttf.as_slice(), "Extracted font bytes differ from cxs-font.ttf");      

    }
}

