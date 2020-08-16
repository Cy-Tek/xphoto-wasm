pub mod filter;

use photon_rs::transform::SamplingFilter;
use photon_rs::PhotonImage;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use crate::filter::{FilterType, get_filter_name};

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}

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
            SamplingFilter::Lanczos3,
        );

        self.base_preview_image = Some(resized_image);
    }

    pub fn draw_filter_preview(&mut self, canvas: HtmlCanvasElement, filter: FilterType) {
        let ctx = Self::get_context_from_canvas(&canvas);
        let filter_name = get_filter_name(filter);

        if let Some(image) = self.filter_previews.get_mut(&filter_name) {
            photon_rs::putImageData(canvas, ctx, image);
        } else {
            if self.base_preview_image.is_none() {
                self.gen_filter_preview(canvas.width(), canvas.height());
            }

            if let Some(ref base_image) = self.base_preview_image {
                let mut copy = PhotonImage::new(
                    base_image.get_raw_pixels(),
                    base_image.get_width(),
                    base_image.get_height(),
                );
                photon_rs::filters::filter(&mut copy, &filter_name);

                self.filter_previews
                    .insert(filter_name.clone().into(), copy);

                let copy = self.filter_previews.get_mut(&filter_name).unwrap();

                canvas.set_width(copy.get_width());
                canvas.set_height(copy.get_height());

                photon_rs::putImageData(canvas, ctx, copy);
            }
        }
    }
}

impl ImageManager {
    fn get_context_from_canvas(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
        let ctx: JsValue = canvas.get_context("2d").unwrap().unwrap().into();
        ctx.into()
    }
}
