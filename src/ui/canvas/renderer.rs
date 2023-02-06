use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

pub trait Renderer: PartialEq {
    fn render(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue>;
}

#[allow(clippy::module_name_repetitions)]
#[dyn_clonable::clonable]
pub trait ContextRenderer: Clone {
    /// This function renders the element at the specified position.
    /// This function does not have to be implemented and does nothing by default.
    fn render_at_position(
        &self,
        ctx: &CanvasRenderingContext2d,
        position: (f64, f64),
    ) -> Result<(), JsValue>;
}