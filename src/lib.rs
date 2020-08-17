pub mod filter;

use photon_rs::transform::SamplingFilter;
use photon_rs::{PhotonImage, base64_to_image};
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
    filter_layers: HashMap<FilterType, PhotonImage>,
    filter_previews: HashMap<FilterType, PhotonImage>,
    filter_order: Vec<FilterType>
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
            filter_layers: HashMap::new(),
            filter_previews: HashMap::new(),
            filter_order: Vec::new()
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

        if let Some(image) = self.filter_previews.get_mut(&filter) {
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
                    .insert(filter, copy);

                let copy = self.filter_previews.get_mut(&filter).unwrap();

                canvas.set_width(copy.get_width());
                canvas.set_height(copy.get_height());

                photon_rs::putImageData(canvas, ctx, copy);
            }
        }
    }

    pub fn add_filter(&mut self, filter: FilterType) {
        let ctx = Self::get_context_from_canvas(&self.base_canvas);
        let filter_name = get_filter_name(filter);

        let mut copy = if self.filter_order.is_empty() {
            PhotonImage::new(
                self.base_image.get_raw_pixels(),
                self.base_image.get_width(),
                self.base_image.get_height()
            )
        } else {
            if let Some(image) = self.filter_layers.get_mut(self.filter_order.last().unwrap()) {
                PhotonImage::new(
                    image.get_raw_pixels(),
                    image.get_width(),
                    image.get_height()
                )
            } else {
                PhotonImage::new(
                    self.base_image.get_raw_pixels(),
                    self.base_image.get_width(),
                    self.base_image.get_height()
                )
            }
        };

        photon_rs::filters::filter(&mut copy, &filter_name);
        photon_rs::putImageData(self.base_canvas.clone(), ctx, &mut copy);

        self.filter_layers.insert(filter, copy);
        self.filter_order.push(filter);
    }

    pub fn remove_filter(&mut self, filter: FilterType) {
        if let Some((ix, name)) = self.filter_order.iter().enumerate().find(|(ix, filter_type)| **filter_type == filter) {
            let prev = if ix > 0 {
                Some(self.filter_order[ix - 1])
            } else {
                None
            };

            let next = if ix < self.filter_order.len() - 1 {
                Some(self.filter_order[ix + 1])
            } else {
                None
            };

            let ctx = Self::get_context_from_canvas(&self.base_canvas);

            match (prev, next) {
                (None, None) => {
                    self.filter_order.pop();
                    self.filter_layers.remove(&filter);

                    photon_rs::putImageData(self.base_canvas.clone(), ctx, &mut self.base_image);
                },
                (Some(prev_filter), None) => {
                    self.filter_order.pop();
                    self.filter_layers.remove(&filter);

                    if let Some(prev_image) = self.filter_layers.get_mut(&prev_filter) {
                        photon_rs::putImageData(self.base_canvas.clone(), ctx, prev_image);
                    }
                },
                (None, Some(next_filter)) => {
                    unimplemented!()
                },
                (Some(prev_filter), Some(next_filter)) => {
                    unimplemented!()
                }
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
