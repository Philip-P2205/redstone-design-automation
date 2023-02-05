use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gloo::utils::window;
use js_sys::Function;
use stylist::style;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{html, Classes, Component, Properties};

use super::{
    canvas::{Canvas, Element, IntoCanvasElement, Renderer},
    connection_point::ConnectionPoint,
    console_option::ConsoleOption,
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
        .unwrap_to_console();
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
    ).unwrap_to_console();
        classes.push(style_workspace);
        let workarea = Workarea::new().unwrap_to_console();

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
    fn new() -> Result<Self, JsValue> {
        let width = Rc::new(Cell::new(Self::get_width()));
        let height = Rc::new(Cell::new(Self::get_height()));
        let mouse_position = Rc::new(Cell::new((0, 0)));
        let grid_position = Rc::new(Cell::new((0.0, 0.0)));
        let canvas_elements: Rc<RefCell<Vec<Element>>> = Rc::new(RefCell::new(Vec::new()));
        let selected_tool: Rc<RefCell<Option<Element>>> = Rc::new(RefCell::new(Some(
            LogicGate::new(LogicGateType::And)
                .unwrap_to_console()
                .into_canvas_element((0.0, 0.0)),
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
            closure.into_js_value().dyn_into()?
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
            closure.into_js_value().dyn_into()?
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

        Ok(Self {
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
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn get_width() -> i32 {
        window()
            .inner_width()
            .expect_to_console("Could not get windows inner width")
            .as_f64()
            .expect_to_console("Could not get windows inner width as f64") as i32
            - 247
    }
    #[allow(clippy::cast_possible_truncation)]
    fn get_height() -> i32 {
        window()
            .inner_height()
            .expect_to_console("Could not get windows inner height")
            .as_f64()
            .expect_to_console("Could not get windows inner height as f64") as i32
            - 97
    }

    fn init(&self, canvas: &web_sys::HtmlCanvasElement) {
        self.initialized.replace(true);
        canvas
            .add_event_listener_with_callback("mousemove", &self.onmousemove)
            .expect_to_console("Could not add event listener mousemove");
        canvas
            .add_event_listener_with_callback("click", &self.onclick)
            .expect_to_console("Could not add event listener click");
    }

    /// This is a simple function to render the currently selected tool
    fn render_selected_tool(&self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        if let Some(tool) = &self.selected_tool.borrow().as_ref() {
            tool.render_at_position(context, self.grid_position.clone().get())?;
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
        Ok(())
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

    fn get_context_from_canvas(
        canvas: &HtmlCanvasElement,
    ) -> Result<CanvasRenderingContext2d, JsValue> {
        Ok(canvas
            .get_context("2d")?
            .ok_or_else(JsValue::null)? // We dont really care about the exact error, just that it didnt work, so null is fine here
            .dyn_into()?)
    }
}

impl Renderer for Workarea {
    fn render(&self, canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
        let _init = self.initialized.get();
        if !self.initialized.get() {
            self.init(canvas);
        }
        let context = Rc::new(Self::get_context_from_canvas(canvas)?);

        context.clear_rect(
            0.0,
            0.0,
            f64::from(canvas.width()),
            f64::from(canvas.height()),
        );
        // context.restore();

        context.begin_path();

        self.render_selected_tool(&context)?;

        for canvas_element in self.canvas_elements.borrow().iter() {
            canvas_element.render(context.clone().as_ref())?;
        }
        self.render_connections(&context);

        context.stroke();
        Ok(())
    }
}
