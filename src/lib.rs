use photon_rs::transform::SamplingFilter;
use photon_rs::PhotonImage;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct ImageManager {
    base_image: PhotonImage,
    base_canvas: HtmlCanvasElement,
    base_preview_image: Option<PhotonImage>,
    filter_layers: Vec<PhotonImage>,
    filter_previews: HashMap<String, PhotonImage>,
}

#[wasm_bindgen]
impl ImageManager {
    pub fn load_from_canvas(canvas: HtmlCanvasElement) -> Self {
        let ctx = Self::get_context_from_canvas(&canvas);
        let image = photon_rs::open_image(canvas.clone(), ctx);

        Self {
            base_image: image,
            base_canvas: canvas,
            base_preview_image: None,
            filter_layers: vec![],
            filter_previews: HashMap::new(),
        }
    }

    pub fn gen_filter_preview(&mut self, max_width: u32, max_height: u32) {
        let min_max = max_width.min(max_height);
        let (img_height, img_width) = (self.base_image.get_height(), self.base_image.get_width());
        let max_img_dimension = img_height.max(img_width);
        let ratio = min_max as f64 / max_img_dimension as f64;
        let resized_image = photon_rs::transform::resize(
            &self.base_image,
            (img_width as f64 * ratio) as u32,
            (img_height as f64 * ratio) as u32,
            SamplingFilter::CatmullRom,
        );

        self.base_preview_image = Some(resized_image);
    }

    pub fn draw_filter_preview(&mut self, canvas: HtmlCanvasElement, filter_name: &str) {
        let ctx = Self::get_context_from_canvas(&canvas);

        if let Some(image) = self.filter_previews.get(filter_name) {
            let copy = photon_rs::base64_to_image(&image.get_base64());
            photon_rs::putImageData(canvas, ctx, copy);
        } else {
            let mut copy = photon_rs::base64_to_image(&self.base_image.get_base64());

            photon_rs::filters::filter(&mut copy, filter_name);

            let filtered_copy = photon_rs::base64_to_image(&copy.get_base64());
            self.filter_previews
                .insert(filter_name.into(), filtered_copy);

            photon_rs::putImageData(canvas, ctx, copy);
        }
    }
}

impl ImageManager {
    fn get_context_from_canvas(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
        let ctx: JsValue = canvas.get_context("2d").unwrap().unwrap().into();
        ctx.into()
    }
}
