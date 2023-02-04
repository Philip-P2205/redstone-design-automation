use crate::ui::canvas::{CanvasContextRenderer, CanvasSVGImage};

/// Wrapper for the SVG of an And Gate
pub struct AndGate {
    position: (f64, f64),
    image: CanvasSVGImage,
}

impl AndGate {
    pub fn new(position: (f64, f64)) -> AndGate {
        AndGate {
            image: CanvasSVGImage::new(Self::get_svg_string()),
            position,
        }
    }
    fn get_svg_string() -> &'static str {
        r#"<svg width="125" height="100" xmlns="http://www.w3.org/2000/svg"><path style="fill: none; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M0,25 H25 V1 H100 V50 H125 H100 V99 H25 V75 H0 H25 V25"></path><text x="75" y="25" style="font-family: Arial; font-size: 20px;">&amp;</text></svg>"#
    }
}
impl CanvasContextRenderer for AndGate {
    fn render(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.draw_image_with_html_image_element(&self.image.image, self.position.0, self.position.1)
            .unwrap();
    }
}
