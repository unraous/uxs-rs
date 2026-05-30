use std::collections::HashMap;

use super::html::{Question, QuestionsRaw};
use super::render::render_glyphs;

fn create_maps(font: &[u8]) -> HashMap<char, char> {
    let mut maps = HashMap::new();
    let glyphs = render_glyphs(font).unwrap_or_default();
    log::debug!("成功渲染 {} 个字形", glyphs.len());
    
    let recognizer = match &*super::recognizer::CRNN_MODEL {
        Ok(model) => model,
        Err(e) => {
            log::error!("OCR 全局模型加载失败: {}", e);
            return maps;
        }
    };
    
    for glyph in glyphs {
        maps.insert(
            glyph.original_char, 
            recognizer.predict(glyph.image)
                .unwrap_or_default().chars().next()
                .unwrap_or('?')
        );
    }
    maps
}

fn map(text: &str, maps: &HashMap<char, char>) -> String {
    text.chars().map(|c| {
        maps.get(&c).cloned().unwrap_or(c)
    }).collect()
}

pub fn decrypt(questions: QuestionsRaw) -> Vec<Question> {
    if let Some(font_data) = questions.font {
        log::info!("检测到加密字体，正在尝试解密...");
        let maps = create_maps(&font_data);
        questions.questions.into_iter().map(|q| {
            Question {
                id: map(&q.id, &maps),
                qtype: map(&q.qtype, &maps),
                stem: map(&q.stem, &maps),
                options: q.options.into_iter()
                    .map(|opt| map(&opt, &maps))
                    .collect(),
            }
        }).collect()
    } else {
        log::debug!("未检测到加密字体，跳过解密步骤");
        questions.questions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use serde_json::to_value;

    #[test]
    fn test_map_basic() {
        let mut maps = HashMap::new();
        maps.insert('a', 'x');
        maps.insert('b', 'y');
        let out = map("abc", &maps);
        assert_eq!(out, "xyc");
    }

    #[test]
    fn test_map_unmapped_chars_unchanged() {
        let maps = HashMap::new();
        let out = map("héllo", &maps);
        assert_eq!(out, "héllo");
    }

    #[test]
    fn test_decrypt_no_font_returns_same_questions() {
        let q1 = Question {
            id: "1".into(),
            qtype: "single".into(),
            stem: "What is 1+1?".into(),
            options: vec!["A. 1".into(), "B. 2".into()],
        };
        let q2 = Question {
            id: "2".into(),
            qtype: "multi".into(),
            stem: "Choose colors".into(),
            options: vec!["Red".into(), "Blue".into()],
        };

        let questions = vec![q1.clone(), q2.clone()];
        let raw = QuestionsRaw { questions: questions.clone(), font: None };
        let decrypted = decrypt(raw);

        let expected = to_value(&questions).expect("serialize expected failed");
        let actual = to_value(&decrypted).expect("serialize actual failed");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_decrypt_flow() {
        let html = include_str!("../../../tests/assets/course-page/webpage.html");
        let raw = QuestionsRaw::new(html).expect("Failed to parse HTML");
        let decrypted = decrypt(raw);
        println!("Decrypted questions: {:#?}", decrypted);

        let f = std::fs::File::create("tests/assets/course-page/decrypted.json")
            .expect("create output file failed");
        serde_json::to_writer_pretty(f, &decrypted).expect("write decrypted json failed");
    }
}