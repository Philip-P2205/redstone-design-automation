use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use js_sys::Function;
use stylist::style;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, CanvasRenderingContext2d};
use yew::{html, Classes, Component, Properties};

use super::{
    canvas::{Canvas, CanvasContextRenderer, CanvasElement, CanvasRenderer, CanvasSVGImage},
    logic_gates::and_gate::AndGate,
};
pub struct Workspace {}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct WorkspaceProps {
    #[prop_or_default]
    pub class: Classes,
    /// The size of the grid
    #[prop_or("1.5em")]
    pub grid_size: &'static str,
}

impl Component for Workspace {
    type Message = ();
    type Properties = WorkspaceProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, _msgg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        // TODO: Make background responsive
        let style_workspace = style!(
            r#"
            width: 100%;
            height: 100%;
            display: grid;
            grid-template-rows: 22px 1fr;
            grid-template-columns: 22px 1fr;
            grid-template-areas: "ruler_corner ruler_top"
                                 "ruler_side workarea";
        "#
        )
        .unwrap();
        let mut classes = ctx.props().class.clone();

        let style_workarea = style!(r#"
            width: 100%;
            height: 100%;
            grid-area: workarea;
            overflow: hidden;
            background-image: linear-gradient(rgba(247, 247, 247, 1.0) .1em, transparent .1em), linear-gradient(90deg, rgba(247, 247, 247, 1.0) .1em, transparent .1em);
            background-size: ${grid_size} ${grid_size};
        "#,
        grid_size= ctx.props().grid_size
    ).unwrap();
        classes.push(style_workspace);
        let workarea = Workarea::new();

        html! (
            <div class={ classes }>
                <div class={ style_workarea }>
                    <Canvas<Workarea> renderer={ Box::new(workarea) } width={ Workarea::get_width() } height={ Workarea::get_height() }>
                    </Canvas<Workarea>>
                </div>
            </div>
        )
    }
}

#[derive(Clone, PartialEq)]
struct Workarea {
    mouse_position: Rc<Cell<(i32, i32)>>,
    width: Rc<Cell<i32>>,
    height: Rc<Cell<i32>>,
    initialized: Cell<bool>,
    onclick: Function,
    onmousemove: Function,
    canvas_elements: Rc<RefCell<Vec<CanvasElement>>>,
}

impl Workarea {
    fn new() -> Workarea {
        let width = Rc::new(Cell::new(
            window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32,
        ));
        let height = Rc::new(Cell::new(
            window().unwrap().inner_height().unwrap().as_f64().unwrap() as i32,
        ));
        let mouse_position = Rc::new(Cell::new((0, 0)));
        let canvas_elements: Rc<RefCell<Vec<CanvasElement>>> = Rc::new(RefCell::new(Vec::new()));

        // {
        //     let width = width.clone();
        //     let height = height.clone();
        //     let cb = Closure::wrap(Box::new(move |_: web_sys::Event| {
        //         width.replace(window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32);
        //         height.replace(window().unwrap().inner_height().unwrap().as_f64().unwrap() as i32);
        //         info!("New Window Size!");
        //     }) as Box<dyn FnMut(_)>);
        //     window()
        //         .unwrap()
        //         .document()
        //         .unwrap()
        //         .add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())
        //         .unwrap();
        // }
        let onclick = {
            let mouse_position = mouse_position.clone();
            let canvas_elements = canvas_elements.clone();
            let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
                Closure::new(move |_event: web_sys::MouseEvent| {
                    let rect = Rect {
                        data: (
                            mouse_position.get().0 as f64,
                            mouse_position.get().1 as f64,
                            100.0,
                            100.0,
                        ),
                    };
                    canvas_elements.borrow_mut().push(CanvasElement::from(rect));
                });
            closure.into_js_value().dyn_into().unwrap()
        };
        let onmousemove = {
            let mouse_position = mouse_position.clone();
            let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
                Closure::new(move |event: web_sys::MouseEvent| {
                    mouse_position.replace((event.client_x() - 247, event.client_y() - 97));
                });
            closure.into_js_value().dyn_into().unwrap()
        };

        {
            canvas_elements
                .borrow_mut()
                .push(CanvasSVGImage::new(AndGate::new().get_svg_string()).into());
        }
        Workarea {
            mouse_position,
            width,
            height,
            initialized: Cell::new(false),
            onclick,
            onmousemove,
            canvas_elements,
        }
    }
}
impl Workarea {
    fn get_width() -> i32 {
        window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32 - 247
    }
    fn get_height() -> i32 {
        window().unwrap().inner_height().unwrap().as_f64().unwrap() as i32 - 97
    }

    fn init(&self, canvas: &web_sys::HtmlCanvasElement) {
        self.initialized.replace(true);
        canvas
            .add_event_listener_with_callback("mousemove", &self.onmousemove)
            .unwrap();
        canvas
            .add_event_listener_with_callback("click", &self.onclick)
            .unwrap();
    }
}
impl CanvasRenderer for Workarea {
    fn render(&self, canvas: &web_sys::HtmlCanvasElement) {
        let _init = self.initialized.get();
        if !self.initialized.get() {
            self.init(canvas);
        }
        let context = Rc::new(
            canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap(),
        );

        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        // context.restore();

        context.begin_path();
        context.rect(
            self.mouse_position.clone().get().0 as f64,
            self.mouse_position.clone().get().1 as f64,
            100.0,
            100.0,
        );
        self.canvas_elements
            .borrow()
            .iter()
            .for_each(|e| e.render(&context));

        context.stroke();
    }
}

pub struct Rect {
    pub data: (f64, f64, f64, f64),
}

impl CanvasContextRenderer for Rect {
    fn render(&self, ctx: &CanvasRenderingContext2d) {
        let r = &self.data;
        ctx.rect(r.0, r.1, r.2, r.3)
    }
}
