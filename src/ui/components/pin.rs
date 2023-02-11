use stylist::style;
use wasm_bindgen::JsValue;
use yew::html;

use crate::ui::{
    canvas::{CanvasContextRenderer, CanvasElement, CanvasSVGImage, IntoCanvasElement},
    connection_point::ConnectionPoint,
    console_option::ConsoleOption,
    redstone_component::RedstoneComponent,
};

#[derive(Debug, Clone)]
pub struct Pin {
    image: CanvasSVGImage,
}

impl Pin {
    pub fn new() -> Result<Self, JsValue> {
        let image = CanvasSVGImage::new(Pin::get_svg_string())?;
        Ok(Self { image })
    }

    fn get_svg_string() -> String {
        format!(
            r#"
        <svg width="25" height="50" xmlns="http://www.w3.org/2000/svg">
            <path style="fill: black; stroke: rgb(0, 0, 0); stroke-width: 2px;" d="M1,25 A5,5,0,0,0,11,25 A5,5,0,0,0,0,25 H25"></path>
        </svg>
        "#
        )
    }
}

impl RedstoneComponent for Pin {
    fn get_component_type(&self) -> crate::ui::redstone_component::ComponentType {
        crate::ui::redstone_component::ComponentType::Pin
    }
    fn get_component_list_item_title(&self) -> String {
        "Pin".into()
    }
    fn get_connection_points(&self) -> Vec<crate::ui::connection_point::ConnectionPoint> {
        vec![ConnectionPoint::new(
            25.0,
            0.0,
            [false, true, false, false],
        )]
    }
    fn get_component_list_item_icon(&self) -> yew::Html {
        let style_image = style!(
            r#"
            align-self: center;
            height: 50px;
        "#
        )
        .unwrap_to_console();
        html! {
            <img class={ style_image } src={ self.image.get_url() } />
        } //TODO
    }
}

impl CanvasContextRenderer for Pin {
    fn render_at_position(
        &self,
        ctx: &web_sys::CanvasRenderingContext2d,
        position: (f64, f64),
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_html_image_element(&self.image.image, position.0, position.1 - 25.0)
    }
}

impl IntoCanvasElement for Pin {
    fn into_canvas_element(self, position: (f64, f64)) -> crate::ui::canvas::CanvasElement {
        let connection_points = self.get_connection_points();
        CanvasElement::new(Box::new(self), position, 50.0, 25.0, connection_points)
    }
}
