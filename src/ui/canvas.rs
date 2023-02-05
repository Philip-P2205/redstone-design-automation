use std::marker::PhantomData;

use gloo::utils::window;
use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    Blob, BlobPropertyBag, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement,
};
use yew::{html, Children, Component, NodeRef, Properties};

use super::{connection_point::ConnectionPoint, console_option::ConsoleOption};

#[derive(Debug, PartialEq, Properties)]
pub struct Props<T>
where
    T: Renderer,
{
    #[prop_or_default]
    pub style: &'static str,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(200)]
    pub width: i32,
    #[prop_or(100)]
    pub height: i32,
    pub renderer: Box<T>,
}

pub enum Msg {
    Init,
    Render,
}

pub struct Canvas<T>
where
    T: Renderer,
{
    canvas: NodeRef,
    callback: Closure<dyn FnMut()>,
    _p: PhantomData<T>,
}

impl<T> Component for Canvas<T>
where
    T: Renderer + 'static,
{
    type Message = Msg;
    type Properties = Props<T>;
    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link().clone();
        let cb: Box<dyn FnMut()> = Box::new(move || link.send_message(Msg::Render));

        let cb = Closure::wrap(cb);

        ctx.link().send_message(Msg::Init);

        Self {
            canvas: NodeRef::default(),
            callback: cb,
            _p: PhantomData::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render => {
                let renderer = &ctx.props().renderer;
                let canvas: HtmlCanvasElement = self
                    .canvas
                    .cast()
                    .expect_to_console("Could not get HtmlCanvasElement");
                window()
                    .request_animation_frame(self.callback.as_ref().unchecked_ref())
                    .expect_to_console("Could not get animation frame");
                renderer.render(&canvas).unwrap_to_console();
            }
            Msg::Init => {
                ctx.link().send_message(Msg::Render);
            }
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let width = ctx.props().width.to_string();
        let height = ctx.props().height.to_string();
        let style = ctx.props().style;
        let children = ctx.props().children.clone();

        html! {
            <canvas ref={ self.canvas.clone() } { style } { width } { height }>
                { for children }
            </canvas>
        }
    }
}

pub trait Renderer: PartialEq {
    fn render(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue>;
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SVGImage {
    pub image: HtmlImageElement,
    _onload: Function,
}

impl SVGImage {
    pub fn new(svg: &'static str) -> Result<Self, JsValue> {
        let image = HtmlImageElement::new()?;

        let array = js_sys::Array::new_with_length(1); // The blob needs an array of the data
        array.set(0, JsValue::from_str(svg));
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

impl ContextRenderer for SVGImage {
    fn render_at_position(
        &self,
        ctx: &CanvasRenderingContext2d,
        position: (f64, f64),
    ) -> Result<(), JsValue> {
        ctx.draw_image_with_html_image_element(&self.image, position.0, position.1)
    }
}

impl From<SVGImage> for HtmlImageElement {
    fn from(value: SVGImage) -> Self {
        value.image
    }
}

#[derive(Clone)]
pub struct Element {
    element: Box<dyn ContextRenderer>,
    position: (f64, f64),
    connection_points: Vec<ConnectionPoint>,
}

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
