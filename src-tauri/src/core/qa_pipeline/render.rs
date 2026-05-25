use image::DynamicImage;
use fontdue::Font;
use ttf_parser::Face;
use std::result::Result;

#[derive(Clone)]
pub struct GlyphImage {
    pub original_char: char,
    pub image: DynamicImage
}

const PADDING: u32 = 16;  // 白色padding的宽度（像素）

fn codepoint(glyph_id: u16, cmap: ttf_parser::cmap::Table) -> Option<u32> {
    for subtable in cmap.subtables {
        let mut result = None;
        subtable.codepoints(|cp| {
            if subtable.glyph_index(cp) == Some(ttf_parser::GlyphId(glyph_id)) {
                result = Some(cp);
            }
        });
        if result.is_some() {
            return result;
        }
    }
    None
}

pub fn render_glyphs(font_data: &[u8]) -> Result<Vec<GlyphImage>, Box<dyn std::error::Error>> {
    let face = Face::parse(font_data, 0)?;
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default())?;
    
    let mut results = Vec::new();
    
    // 遍历字体中的所有字形
    for glyph_id in 0..face.number_of_glyphs() {
        
        let Some(cmap) = face.tables().cmap else { continue };
        if let Some(cp) = codepoint(glyph_id, cmap) {
            let char_val = char::from_u32(cp).expect("Invalid character");
            let (metrics, bitmap) = font.rasterize(char_val, 64.0);
            
            let orig_width = metrics.width as u32;
            let orig_height = metrics.height as u32;

            if orig_width == 0 || orig_height == 0 { continue };
            
                // 反转颜色
            let inverted_bitmap: Vec<u8> = bitmap.iter().map(|&b| 255 - b).collect();
            
            // 直接创建包含padding的图像
            let padded_width = orig_width + 2 * PADDING;
            let padded_height = orig_height + 2 * PADDING;
            let mut luma = image::ImageBuffer::<image::Luma<u8>, Vec<u8>>::new(
                padded_width, padded_height
            );
            
            // 填充白色背景
            for pixel in luma.pixels_mut() {
                *pixel = image::Luma([255u8]);
            }
            
            // 直接在中心位置复制bitmap数据
            for (i, &b) in inverted_bitmap.iter().enumerate() {
                let x = (i % orig_width as usize) as u32 + PADDING;
                let y = (i / orig_width as usize) as u32 + PADDING;
                luma.put_pixel(x, y, image::Luma([b]));
            }
            
            let img = DynamicImage::ImageLuma8(luma);
            
            results.push(GlyphImage {
                original_char: char_val,
                image: img,
            });
                
        }
        
    }
    
    Ok(results)
}
