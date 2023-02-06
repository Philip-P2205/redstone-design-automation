use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::ui::connection_point::ConnectionPoint;

use super::renderer::ContextRenderer;

#[derive(Clone)]
pub struct Element {
    element: Box<dyn ContextRenderer>,
    position: (f64, f64),
    connection_points: Vec<ConnectionPoint>,
}

#[allow(clippy::module_name_repetitions)]
pub trait IntoCanvasElement {
    fn into_canvas_element(self, position: (f64, f64)) -> Element;
}

impl Element {
    pub fn new(
        element: Box<dyn ContextRenderer>,
        position: (f64, f64),
        connection_points: &[ConnectionPoint],
    ) -> Self {
        Self {
            element,
            position,
            connection_points: connection_points.to_vec(),
        }
    }
    pub fn render(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.render_at_position(ctx, self.position)
    }
    pub fn render_at_position(
        &self,
        ctx: &CanvasRenderingContext2d,
        position: (f64, f64),
    ) -> Result<(), JsValue> {
        self.element.render_at_position(ctx, position)
        // self.connection_points
        //     .iter()
        //     .for_each(|e| e.render_at_position(ctx, position))
    }

    // pub fn set_position(&mut self, position: (f64, f64)) {
    //     self.position = position;
    // }
    // pub fn get_position(&self) -> (f64, f64) {
    //     self.position
    // }
    pub fn at_position(&self, position: (f64, f64)) -> Self {
        Self {
            element: self.element.clone(),
            position,
            connection_points: self.connection_points.clone(),
        }
    }
    pub const fn get_connection_points(&self) -> &Vec<ConnectionPoint> {
        &self.connection_points
    }
}

impl PartialEq for Element {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}