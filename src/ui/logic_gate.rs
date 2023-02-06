use wasm_bindgen::JsValue;

use crate::ui::canvas::{CanvasContextRenderer, CanvasSVGImage};

use super::{
    canvas::{CanvasElement, IntoCanvasElement},
    connection_point::ConnectionPoint,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LogicGateType {
    And,
    Or,
    Xor,
    Nand,
    Nor,
}

impl LogicGateType {
    fn get_svg_string(&self, inputs_inverted: (bool, bool)) -> String {
        let (text, text_x) = self.get_svg_text();
        let (input_path1, input_path2) = self.get_input_svg_path(inputs_inverted);
        let output_path = self.get_output_svg_path();
        format!(
            r#"
            <svg width="125" height="100" xmlns="http://www.w3.org/2000/svg">
                <path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 {input_path1} V1 H100 V50 {output_path} V99 H25 V75 {input_path2} V25"></path>
                <text x="{text_x}" y="25" style="font-family: Arial; font-size: 20px;">{text}</text>
            </svg>"#
        )
    }

    const fn get_svg_text(&self) -> (&'static str, i32) {
        use LogicGateType::{And, Nand, Nor, Or, Xor};
        match self {
            And | Nand => ("&amp;", 75),
            Or | Nor => ("&gt;=1", 55),
            Xor => ("=1", 65),
        }
    }

    const fn get_input_svg_path(
        &self,
        inputs_inverted: (bool, bool),
    ) -> (&'static str, &'static str) {
        let input1 = if !inputs_inverted.0 {
            "H25"
        } else {
            "H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25"
        };
        let input2 = if !inputs_inverted.1 {
            "H0 H25"
        } else {
            "A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75"
        };
        (input1, input2)
    }

    const fn get_output_svg_path(&self) -> &'static str {
        use LogicGateType::{And, Nand, Nor, Or, Xor};
        match self {
            And | Or | Xor => "H125 H100",
            Nand | Nor => "A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50",
        }
    }
}

#[derive(Clone)]
/// A Simple 2 input 1 ouput logic gate
pub struct LogicGate {
    _gate_type: LogicGateType,
    image: CanvasSVGImage,
}
impl LogicGate {
    pub fn new(gate_type: LogicGateType) -> Result<Self, JsValue> {
        Self::new_with_inverted_inputs(gate_type, (false, false))
    }

    pub fn new_with_inverted_inputs(
        gate_type: LogicGateType,
        inputs_inverted: (bool, bool),
    ) -> Result<Self, JsValue> {
        let image = CanvasSVGImage::new(gate_type.get_svg_string(inputs_inverted))?;
        Ok(Self {
            _gate_type: gate_type,
            image,
        })
    }
    pub const fn get_connection_points() -> [ConnectionPoint; 3] {
        [
            ConnectionPoint::new(0.0, 25.0, [true, false, false, true]),
            ConnectionPoint::new(0.0, 75.0, [false, false, true, true]),
            ConnectionPoint::new(125.0, 50.0, [true, true, true, false]),
        ]
    }
}
impl CanvasContextRenderer for LogicGate {
    fn render_at_position(
        &self,
        ctx: &web_sys::CanvasRenderingContext2d,
        position: (f64, f64),
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_html_image_element(&self.image.image, position.0, position.1)
    }
}

impl TryFrom<LogicGateType> for LogicGate {
    type Error = JsValue;
    fn try_from(value: LogicGateType) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl IntoCanvasElement for LogicGate {
    fn into_canvas_element(self, position: (f64, f64)) -> CanvasElement {
        CanvasElement::new(Box::new(self), position, &Self::get_connection_points())
    }
}
