//! CRNN (Convolutional Recurrent Neural Network) recognizer for Chinese OCR.
//! 
//! The model used in this module is based on the chineseocr_lite project:
//! https://github.com/DayBreak-u/chineseocr_lite
//! 
//! This implementation provides character-level recognition capabilities
//! for processing images containing Chinese text.

use image::DynamicImage;
use tract_onnx::prelude::*;

const LABELS_TEXT: &str = include_str!("../../../model/keys.txt");
const ONNX_MODEL: &[u8] = include_bytes!("../../../model/chineseocr_lite.onnx");

pub struct CRNNHandle {
    model: TypedRunnableModel<TypedModel>,
}

impl CRNNHandle {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let model = tract_onnx::onnx()
            .model_for_read(&mut std::io::Cursor::new(ONNX_MODEL))?
            .into_typed()?
            .into_optimized()?
            .into_runnable()?;
        
        log::debug!("CRNN模型加载成功，准备进行字符识别");
        Ok(CRNNHandle { model })
    }

    pub fn predict(&self, image: DynamicImage) -> Result<String, Box<dyn std::error::Error>> {
        let img_rgb = image.to_rgb8();
        let (w, h) = img_rgb.dimensions();
        
        let scale = h as f32 / 32.0;
        let new_w = (w as f32 / scale) as u32;
        let img_resized = image::imageops::resize(&img_rgb, new_w, 32, image::imageops::FilterType::Triangle);
        
        // 构造 BCHW 格式的数据
        let mut data_bchw = vec![0.0; 96 * (new_w as usize)];
        
        for y in 0..32 {
            for x in 0..new_w {
                let pixel = img_resized.get_pixel(x, y);
                for c in 0..3 {
                    let val = pixel.0[c] as f32;
                    let normalized = (val / 127.5) - 1.0;
                    let idx = c * 32 * (new_w as usize) + (y as usize) * (new_w as usize) + (x as usize);
                    data_bchw[idx] = normalized;
                }
            }
        }
        
        let input = Tensor::from_shape(&[1, 3, 32, new_w as usize], &data_bchw)?;
        let outputs = self.model.run(tvec![input.into()])?;
        let pred_tensor = &outputs[0];
        
        let pred_data = pred_tensor.as_slice::<f32>()?;
        let shape = pred_tensor.shape();
        
        let seq_len = shape[0];
        let num_classes = shape[2];
        
        let mut result = Vec::new();
        for t in 0..seq_len {
            let mut max_idx = 0;
            let mut max_val = f32::NEG_INFINITY;
            for c in 0..num_classes {
                let val = pred_data[t * num_classes + c];
                if val > max_val {
                    max_val = val;
                    max_idx = c;
                }
            }
            result.push(max_idx);
        }
        
        decode_output(&result, seq_len)
    }
}

fn decode_output(indices: &[usize], length: usize) -> Result<String, Box<dyn std::error::Error>> {
    let labels: Vec<&str> = LABELS_TEXT.lines().collect();
    
    let mut output = String::new();
    for i in 0..length {
        if indices[i] != 0 && !(i > 0 && indices[i - 1] == indices[i]) {
            if let Some(label) = labels.get(indices[i] - 1) {
                output.push_str(label);
            }
        }
    }
    
    Ok(output)
}