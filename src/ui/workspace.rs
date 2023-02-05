use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use js_sys::Function;
use stylist::style;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, CanvasRenderingContext2d};
use yew::{html, Classes, Component, Properties};

use super::{
    canvas::{Canvas, ContextRenderer, Element, IntoCanvasElement, Renderer},
    connection_point::ConnectionPoint,
    logic_gates::{LogicGate, LogicGateType},
};

const GRID_SIZE: f64 = 25.0;
const GRID_SIZE_PROPS: &str = "25px";

pub struct Workspace {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    /// The size of the grid
    #[prop_or(GRID_SIZE_PROPS)]
    pub grid_size: &'static str,
    #[prop_or_default]
    pub selected_tool: Option<Element>,
}

impl Component for Workspace {
    type Message = ();
    type Properties = Props;

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
    grid_position: Rc<Cell<(f64, f64)>>,
    width: Rc<Cell<i32>>,
    height: Rc<Cell<i32>>,
    initialized: Cell<bool>,
    onclick: Function,
    onmousemove: Function,
    canvas_elements: Rc<RefCell<Vec<Element>>>,
    connection_points: Rc<RefCell<Vec<ConnectionPoint>>>,
    selected_tool: Rc<RefCell<Option<Element>>>,
}

impl Workarea {
    #[allow(clippy::cast_possible_truncation)]
    fn new() -> Self {
        let width = Rc::new(Cell::new(
            window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32,
        ));
        let height = Rc::new(Cell::new(
            window().unwrap().inner_height().unwrap().as_f64().unwrap() as i32,
        ));
        let mouse_position = Rc::new(Cell::new((0, 0)));
        let grid_position = Rc::new(Cell::new((0.0, 0.0)));
        let canvas_elements: Rc<RefCell<Vec<Element>>> = Rc::new(RefCell::new(Vec::new()));
        let selected_tool: Rc<RefCell<Option<Element>>> = Rc::new(RefCell::new(Some(
            LogicGate::new(LogicGateType::And).into_canvas_element((0.0, 0.0)),
        )));
        let connection_points = Rc::new(RefCell::new(Vec::new()));

        let onclick = {
            let grid_position = grid_position.clone();
            let canvas_elements = canvas_elements.clone();
            let selected_tool = selected_tool.clone();
            let connection_points = connection_points.clone();
            let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
                Closure::new(move |_event: web_sys::MouseEvent| {
                    if let Some(tool) = selected_tool.borrow().as_ref() {
                        canvas_elements
                            .borrow_mut()
                            .push(tool.at_position(grid_position.get()));

                        for cp in tool.get_connection_points() {
                            connection_points
                                .borrow_mut()
                                .push(cp.get_absolute_at_position(grid_position.get()));
                        }
                    }
                });
            closure.into_js_value().dyn_into().unwrap()
        };
        let onmousemove = {
            let grid_position = grid_position.clone();
            let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
                Closure::new(move |event: web_sys::MouseEvent| {
                    let x = (f64::from(event.client_x() - 247) / GRID_SIZE).round() * GRID_SIZE;
                    let y = (f64::from(event.client_y() - 97) / GRID_SIZE).round() * GRID_SIZE;
                    // _mouse_position.replace((event.client_x() - 247, event.client_y() - 97));
                    grid_position.replace((x, y));
                });
            closure.into_js_value().dyn_into().unwrap()
        };
        {
            // canvas_elements.borrow_mut().push(
            //     LogicGate::new_with_inverted_inputs(LogicGateType::Nand, (true, true))
            //         .as_canvas_element((0.0, 0.0)),
            // );
            // canvas_elements
            //     .borrow_mut()
            //     .push(LogicGate::new(LogicGateType::Nor).as_canvas_element((100.0, 0.0)));
            // canvas_elements
            //     .borrow_mut()
            //     .push(LogicGate::new(LogicGateType::And).as_canvas_element((200.0, 0.0)));
        }

        Self {
            mouse_position,
            grid_position,
            width,
            height,
            initialized: Cell::new(false),
            onclick,
            onmousemove,
            canvas_elements,
            selected_tool,
            connection_points,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn get_width() -> i32 {
        window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32 - 247
    }
    #[allow(clippy::cast_possible_truncation)]
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

    /// This is a simple function to render the currently selected tool
    fn render_selected_tool(&self, context: &CanvasRenderingContext2d) {
        if let Some(tool) = &self.selected_tool.borrow().as_ref() {
            tool.render_at_position(context, self.grid_position.clone().get());
            for cp in tool.get_connection_points() {
                let cp1 = &cp.get_absolute_at_position(self.grid_position.clone().get());
                for cp2 in self.connection_points.borrow().iter() {
                    if Self::check_connection_points(cp1, cp2) {
                        context.begin_path();
                        context.set_stroke_style(&JsValue::from_str("black"));
                        context.move_to(cp1.get_position_x(), cp1.get_position_y());
                        context.line_to(cp2.get_position_x(), cp2.get_position_y());
                        context.stroke();
                    }
                }
            }
        }
    }
    fn render_connections(&self, context: &CanvasRenderingContext2d) {
        for cp1 in self.connection_points.borrow().iter() {
            for cp2 in self.connection_points.borrow().iter() {
                if Self::check_connection_points(cp1, cp2) {
                    context.begin_path();
                    context.set_stroke_style(&JsValue::from_str("black"));
                    context.move_to(cp1.get_position_x(), cp1.get_position_y());
                    context.line_to(cp2.get_position_x(), cp2.get_position_y());
                    context.stroke();
                }
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn check_connection_points(cp1: &ConnectionPoint, cp2: &ConnectionPoint) -> bool {
        if cp1 == cp2 {
            return false;
        } else if (cp1.get_position_x() as i32 == cp2.get_position_x() as i32
            && (cp1.get_position_y() > cp2.get_position_y()
                && cp1.get_direction_y_neg()
                && cp2.get_direction_y_pos()
                || cp1.get_position_y() < cp2.get_position_y()
                    && cp1.get_direction_y_pos()
                    && cp2.get_direction_y_neg()))
            || (cp1.get_position_y() as i32 == cp2.get_position_y() as i32
                && (cp1.get_position_x() > cp2.get_position_x()
                    && cp1.get_direction_x_neg()
                    && cp2.get_direction_x_pos()
                    || cp1.get_position_x() < cp2.get_position_x()
                        && cp1.get_direction_x_pos()
                        && cp2.get_direction_x_neg()))
        {
            return true;
        }
        false
    }
}

impl Renderer for Workarea {
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

        context.clear_rect(
            0.0,
            0.0,
            f64::from(canvas.width()),
            f64::from(canvas.height()),
        );
        // context.restore();

        context.begin_path();

        self.render_selected_tool(&context);

        self.canvas_elements
            .borrow()
            .iter()
            .for_each(|e| e.render(&context));
        self.render_connections(&context);

        context.stroke();
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub data: (f64, f64, f64, f64),
}

impl ContextRenderer for Rect {
    fn render_at_position(&self, ctx: &CanvasRenderingContext2d, _position: (f64, f64)) {
        let r = &self.data;
        ctx.rect(r.0, r.1, r.2, r.3);
    }
}
