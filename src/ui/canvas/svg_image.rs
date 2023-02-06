use js_sys::Function;
use wasm_bindgen::{JsValue, prelude::Closure, JsCast};
use web_sys::{HtmlImageElement, BlobPropertyBag, CanvasRenderingContext2d, Blob};

use crate::ui::console_option::ConsoleOption;

use super::renderer::CanvasContextRenderer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanvasSVGImage {
    pub image: HtmlImageElement,
    _onload: Function,
}

impl CanvasSVGImage {
    pub fn new(svg: String) -> Result<Self, JsValue> {
        let image = HtmlImageElement::new()?;

        let array = js_sys::Array::new_with_length(1); // The blob needs an array of the data
        array.set(0, JsValue::from(svg));
        let mut options = BlobPropertyBag::new();
        options.type_("image/svg+xml");
        let blob = Blob::new_with_buffer_source_sequence_and_options(&array, &options)?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)?;
        image.set_src(&url);

        let closure: Closure<dyn FnMut()> = Closure::new(move || {
            web_sys::Url::revoke_object_url(&url)
                .expect_to_console(&format!("Could not revoke object url for {url}"));
            // info!("Drawing image!", &url);
        });
        let onload: Function = closure.into_js_value().dyn_into()?;
        image.set_onload(Some(&onload));
        Ok(Self {
            image,
            _onload: onload,
        })
    }
}

impl CanvasContextRenderer for CanvasSVGImage {
    fn render_at_position(
        &self,
        ctx: &CanvasRenderingContext2d,
        position: (f64, f64),
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_html_image_element(&self.image, position.0, position.1)
    }
}

impl From<CanvasSVGImage> for HtmlImageElement {
    fn from(value: CanvasSVGImage) -> Self {
        value.image
    }
}