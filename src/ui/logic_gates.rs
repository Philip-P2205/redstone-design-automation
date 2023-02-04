use crate::ui::canvas::{CanvasContextRenderer, CanvasSVGImage};

#[derive(Debug)]
pub enum LogicGateType {
    And,
    Or,
    Xor,
    Nand,
    Nor,
}

impl LogicGateType {
    fn get_svg_string(&self, inputs_inverted: (bool, bool)) -> &'static str {
        use LogicGateType::*;
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
            _ => "",
        }
    }
}

/// A Simple 2 input 1 ouput logic gate
pub struct LogicGate {
    gate_type: LogicGateType,
    position: (f64, f64),
    image: CanvasSVGImage,
}
impl LogicGate {
    pub fn new(gate_type: LogicGateType, position: (f64, f64)) -> LogicGate {
        Self::new_with_inverted_inputs(gate_type, position, (false, false))
    }

    pub fn new_with_inverted_inputs(
        gate_type: LogicGateType,
        position: (f64, f64),
        inputs_inverted: (bool, bool),
    ) -> LogicGate {
        let image = CanvasSVGImage::new(gate_type.get_svg_string(inputs_inverted));
        LogicGate {
            gate_type,
            position,
            image,
        }
    }
}
impl CanvasContextRenderer for LogicGate {
    fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        use LogicGateType::*;
        match self.gate_type {
            And => ctx
                .draw_image_with_html_image_element(
                    &self.image.image,
                    self.position.0,
                    self.position.1,
                )
                .unwrap(),
            Or => ctx
                .draw_image_with_html_image_element(
                    &self.image.image,
                    self.position.0,
                    self.position.1,
                )
                .unwrap(),
            Xor => ctx
                .draw_image_with_html_image_element(
                    &self.image.image,
                    self.position.0,
                    self.position.1,
                )
                .unwrap(),
            Nand => ctx
                .draw_image_with_html_image_element(
                    &self.image.image,
                    self.position.0,
                    self.position.1,
                )
                .unwrap(),
            Nor => ctx
                .draw_image_with_html_image_element(
                    &self.image.image,
                    self.position.0,
                    self.position.1,
                )
                .unwrap(),
            _ => (),
        }
    }
}
