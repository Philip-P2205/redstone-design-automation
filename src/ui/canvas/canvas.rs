use std::marker::PhantomData;

use gloo::utils::window;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlCanvasElement;
use yew::{html, Children, Component, NodeRef, Properties};

use super::{super::console_option::ConsoleOption, renderer::CanvasRenderer};

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
            _p: PhantomData::default(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CanvasMsg::Render => {
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

