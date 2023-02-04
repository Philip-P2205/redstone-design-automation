use std::marker::PhantomData;

use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    window, Blob, BlobPropertyBag, CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement,
};
use yew::{html, Children, Component, NodeRef, Properties};

#[derive(Debug, PartialEq, Properties)]
pub struct CanvasProps<T>
where
    T: CanvasRenderer,
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

pub enum CanvasMsg {
    Init,
    Render,
}

pub struct Canvas<T>
where
    T: CanvasRenderer,
{
    canvas: NodeRef,
    callback: Closure<dyn FnMut()>,
    _p: PhantomData<T>,
}

impl<T> Component for Canvas<T>
where
    T: CanvasRenderer + 'static,
{
    type Message = CanvasMsg;
    type Properties = CanvasProps<T>;
    fn create(ctx: &yew::Context<Self>) -> Self {
        let link = ctx.link().clone();
        let cb: Box<dyn FnMut()> = Box::new(move || link.send_message(CanvasMsg::Render));

        let cb = Closure::wrap(cb);

        ctx.link().send_message(CanvasMsg::Init);

        Self {
            canvas: NodeRef::default(),
            callback: cb,
            _p: Default::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CanvasMsg::Render => {
                let renderer = &ctx.props().renderer;
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                window()
                    .unwrap()
                    .request_animation_frame(self.callback.as_ref().unchecked_ref())
                    .unwrap();
                renderer.render(&canvas);
            }
            CanvasMsg::Init => {
                ctx.link().send_message(CanvasMsg::Render);
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

pub trait CanvasRenderer: PartialEq {
    fn render(&self, canvas: &HtmlCanvasElement);
}

pub trait CanvasContextRenderer {
    fn render(&self, ctx: &CanvasRenderingContext2d);
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasSVGImage {
    pub image: HtmlImageElement,
    _onload: Function,
}

impl CanvasSVGImage {
    pub fn new(svg: &'static str) -> CanvasSVGImage {
        let image = HtmlImageElement::new().unwrap();

        let array = js_sys::Array::new_with_length(1); // The blob needs an array of the data
        array.set(0, JsValue::from_str(svg));
        let mut options = BlobPropertyBag::new();
        options.type_("image/svg+xml");
        let blob = Blob::new_with_buffer_source_sequence_and_options(&array, &options).unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        image.set_src(&url);

        let closure: Closure<dyn FnMut()> = Closure::new(move || {
            web_sys::Url::revoke_object_url(&url).unwrap();
            // info!("Drawing image!", &url);
        });
        let _onload: Function = closure.into_js_value().dyn_into().unwrap();
        image.set_onload(Some(&_onload));
        CanvasSVGImage { image, _onload }
    }
}

impl CanvasContextRenderer for CanvasSVGImage {
    fn render(&self, context: &CanvasRenderingContext2d) {
        context
            .draw_image_with_html_image_element(&self.image, 10.0, 10.0)
            .unwrap();
    }
}

impl Into<HtmlImageElement> for CanvasSVGImage {
    fn into(self) -> HtmlImageElement {
        self.image
    }
}

pub struct CanvasElement {
    element: Box<dyn CanvasContextRenderer>,
}

impl CanvasElement {
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        self.element.render(ctx)
    }
}

impl<T> From<T> for CanvasElement
where
    T: 'static + CanvasContextRenderer,
{
    fn from(value: T) -> Self {
        Self {
            element: Box::new(value),
        }
    }
}

impl From<Box<dyn CanvasContextRenderer>> for CanvasElement {
    fn from(value: Box<dyn CanvasContextRenderer>) -> Self {
        Self { element: value }
    }
}

impl Clone for CanvasElement {
    fn clone(&self) -> Self {
        panic!("Cannot clone CanvasElement!");
    }
}

impl PartialEq for CanvasElement {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
    fn ne(&self, _other: &Self) -> bool {
        false
    }
}
