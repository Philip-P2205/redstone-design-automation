use crate::ui::canvas::{ContextRenderer, SVGImage};

use super::{
    canvas::{Element, IntoCanvasElement},
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
    const fn get_svg_string(&self, inputs_inverted: (bool, bool)) -> &'static str {
        use LogicGateType::{And, Nand, Nor, Or, Xor};
        // TODO: Someone please replace this match statement with something more readable
        match inputs_inverted {
            (false, false) => match self {
                And => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 H125 H100 V99 H25 V75 H0 H25 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
                }
                Or => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 H125 H100 V99 H25 V75 H0 H25 V25"></path><text x="55" y="25" style="font-family: Arial; font-size: 20px;">&gt;=1</text></svg>"#
                }
                Xor => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 H125 H100 V99 H25 V75 H0 H25 V25"></path><text x="65" y="25" style="font-family: Arial; font-size: 20px;">=1</text></svg>"#
                }
                Nand => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50 V99 H25 V75 H0 H25 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
                }
                Nor => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50 V99 H25 V75 H0 H25 V25"></path><text x="55" y="25" style="font-family: Arial; font-size: 20px;">&gt;=1</text></svg>"#
                }
            },
            (true, false) => match self {
                And => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 H125 H100 V99 H25 V75 H0 H25 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
                }
                Or => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 H125 H100 V99 H25 V75 H0 H25 V25"></path><text x="55" y="25" style="font-family: Arial; font-size: 20px;">&gt;=1</text></svg>"#
                }
                Xor => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 H125 H100 V99 H25 V75 H0 H25 V25"></path><text x="65" y="25" style="font-family: Arial; font-size: 20px;">=1</text></svg>"#
                }
                Nand => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50 V99 H25 V75 H0 H25 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
                }
                Nor => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50 V99 H25 V75 H0 H25 V25"></path><text x="55" y="25" style="font-family: Arial; font-size: 20px;">&gt;=1</text></svg>"#
                }
            },
            (false, true) => match self {
                And => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 H125 H100 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="55" y="25" style="font-family: Arial; font-size: 20px;">&gt;=1</text></svg>"#
                }
                Or => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 H125 H100 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
                }
                Xor => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 H125 H100 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="65" y="25" style="font-family: Arial; font-size: 20px;">=1</text></svg>"#
                }
                Nand => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
                }
                Nor => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="55" y="25" style="font-family: Arial; font-size: 20px;">&gt;=1</text></svg>"#
                }
            },
            (true, true) => match self {
                And => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 H125 H100 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="55" y="25" style="font-family: Arial; font-size: 20px;">&gt;=1</text></svg>"#
                }
                Or => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 H125 H100 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
                }
                Xor => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 H125 H100 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="65" y="25" style="font-family: Arial; font-size: 20px;">=1</text></svg>"#
                }
                Nand => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
                }
                Nor => {
                    r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H15 A5,5,0,0,0,25,25 A5,5,0,0,0,15,25 M25,25 V1 H100 V50 A5,5,0,0,0,110,50 A5,5,0,0,0,100,50 M110,50 H125 M100,50 V99 H25 V75 A5,5,0,0,0,15,75 H0 M15,75 A5,5,0,0,0,25,75 V25"></path><text x="55" y="25" style="font-family: Arial; font-size: 20px;">&gt;=1</text></svg>"#
                }
            },
        }
    }
}

#[derive(Clone)]
/// A Simple 2 input 1 ouput logic gate
pub struct LogicGate {
    _gate_type: LogicGateType,
    image: SVGImage,
}
impl LogicGate {
    pub fn new(gate_type: LogicGateType) -> Self {
        Self::new_with_inverted_inputs(gate_type, (false, false))
    }

    pub fn new_with_inverted_inputs(
        gate_type: LogicGateType,
        inputs_inverted: (bool, bool),
    ) -> Self {
        let image = SVGImage::new(gate_type.get_svg_string(inputs_inverted));
        Self { _gate_type: gate_type, image }
    }
    pub const fn get_connection_points() -> [ConnectionPoint; 3] {
        [
            ConnectionPoint::new(0.0, 25.0, [true, false, false, true]),
            ConnectionPoint::new(0.0, 75.0, [false, false, true, true]),
            ConnectionPoint::new(125.0, 50.0, [true, true, true, false]),
        ]
    }
}
impl ContextRenderer for LogicGate {
    fn render_at_position(&self, ctx: &web_sys::CanvasRenderingContext2d, position: (f64, f64)) {
        // use LogicGateType::{And, Nand, Nor, Or, Xor};
        ctx.draw_image_with_html_image_element(&self.image.image, position.0, position.1)
            .unwrap();
        // match self.gate_type {
        //     And => ctx
        //         .draw_image_with_html_image_element(&self.image.image, position.0, position.1)
        //         .unwrap(),
        //     Or => ctx
        //         .draw_image_with_html_image_element(&self.image.image, position.0, position.1)
        //         .unwrap(),
        //     Xor => ctx
        //         .draw_image_with_html_image_element(&self.image.image, position.0, position.1)
        //         .unwrap(),
        //     Nand => ctx
        //         .draw_image_with_html_image_element(&self.image.image, position.0, position.1)
        //         .unwrap(),
        //     Nor => ctx
        //         .draw_image_with_html_image_element(&self.image.image, position.0, position.1)
        //         .unwrap(),
        // }
    }
}

impl From<LogicGateType> for LogicGate {
    fn from(value: LogicGateType) -> Self {
        Self::new(value)
    }
}

impl IntoCanvasElement for LogicGate {
    fn into_canvas_element(self, position: (f64, f64)) -> Element {
        Element::new(Box::new(self), position, &Self::get_connection_points())
    }
}
