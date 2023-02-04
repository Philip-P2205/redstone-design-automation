/// Wrapper for the SVG of an And Gate
pub struct AndGate;

impl AndGate {
    pub fn new() -> AndGate {
        AndGate
    }
    pub fn get_svg_string(&self) -> &'static str {
        r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 H125 H100 V99 H25 V75 H0 H25 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
    }
}
