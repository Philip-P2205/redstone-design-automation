use std::f64::consts::PI;

use wasm_bindgen::JsValue;

use super::canvas::ContextRenderer;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConnectionPoint {
    /// relative position to element origin
    x: f64,
    /// relative position to element origin
    y: f64,
    directions: [bool; 4],
}

impl ConnectionPoint {
    pub const fn new(x: f64, y: f64, directions: [bool; 4]) -> Self {
        Self { x, y, directions }
    }
    // pub fn get_position(&self) -> (f64, f64) {
    //     (self.x, self.y)
    // }
    pub const fn get_position_x(&self) -> f64 {
        self.x
    }
    pub const fn get_position_y(&self) -> f64 {
        self.y
    }
    // pub fn get_directions(&self) -> &[bool; 4] {
    //     &self.directions
    // }
    pub const fn get_direction_x_pos(&self) -> bool {
        self.directions[1]
    }
    pub const fn get_direction_x_neg(&self) -> bool {
        self.directions[3]
    }
    pub const fn get_direction_y_pos(&self) -> bool {
        self.directions[2]
    }
    pub const fn get_direction_y_neg(&self) -> bool {
        self.directions[0]
    }
    pub fn get_absolute_at_position(&self, position: (f64, f64)) -> Self {
        Self {
            x: self.x + position.0,
            y: self.y + position.1,
            directions: self.directions,
        }
    }
}

impl ContextRenderer for ConnectionPoint {
    fn render_at_position(&self, ctx: &web_sys::CanvasRenderingContext2d, position: (f64, f64)) {
        ctx.begin_path();
        ctx.set_stroke_style(&JsValue::from_str("red"));
        ctx.set_fill_style(&JsValue::from_str("red"));
        ctx.arc(self.x + position.0, self.y + position.1, 5.0, 0.0, 2.0 * PI)
            .unwrap();
        ctx.fill();
    }
}
