use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gloo::utils::window;
use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::ui::{
    application::ApplicationState,
    canvas::{CanvasElement, CanvasRenderer},
    connection_point::ConnectionPoint,
    console_option::ConsoleOption,
};

use super::GRID_SIZE;

#[derive(Clone, PartialEq)]
pub struct Workarea {
    mouse_position: Rc<Cell<(i32, i32)>>,
    grid_position: Rc<Cell<(f64, f64)>>,
    width: Rc<Cell<i32>>,
    height: Rc<Cell<i32>>,
    initialized: Cell<bool>,
    onclick: Function,
    onmousemove: Function,
    canvas_elements: Rc<RefCell<Vec<CanvasElement>>>,
    connection_points: Rc<RefCell<Vec<ConnectionPoint>>>,
    application_state: Rc<RefCell<ApplicationState>>,
    // selected_tool: Rc<RefCell<Option<CanvasElement>>>,
}

impl Workarea {
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(application_state: Rc<RefCell<ApplicationState>>) -> Result<Self, JsValue> {
        let width = Rc::new(Cell::new(Self::get_width()));
        let height = Rc::new(Cell::new(Self::get_height()));
        let mouse_position = Rc::new(Cell::new((0, 0)));
        let grid_position = Rc::new(Cell::new((0.0, 0.0)));
        let canvas_elements: Rc<RefCell<Vec<CanvasElement>>> = Rc::new(RefCell::new(Vec::new()));
        let connection_points = Rc::new(RefCell::new(Vec::new()));

        let onclick = {
            let grid_position = grid_position.clone();
            let canvas_elements = canvas_elements.clone();
            let application_state = application_state.clone();
            let connection_points = connection_points.clone();
            let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
                Closure::new(move |_event: web_sys::MouseEvent| {
                    if let Some(tool) = application_state.borrow().tool_active.as_ref() {
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

        Ok(Self {
            mouse_position,
            grid_position,
            width,
            height,
            initialized: Cell::new(false),
            onclick,
            onmousemove,
            canvas_elements,
            application_state,
            connection_points,
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn get_width() -> i32 {
        window()
            .inner_width()
            .expect_to_console("Could not get windows inner width")
            .as_f64()
            .expect_to_console("Could not get windows inner width as f64") as i32
            - 247
    }
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_height() -> i32 {
        window()
            .inner_height()
            .expect_to_console("Could not get windows inner height")
            .as_f64()
            .expect_to_console("Could not get windows inner height as f64") as i32
            - 97
    }

    pub fn init(&self, canvas: &web_sys::HtmlCanvasElement) {
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
        if let Some(tool) = self
            .application_state
            .borrow()
            .tool_active
            .as_ref()
            .as_ref()
        {
            // FIXME: Connections are currently made trough elements, which is not wanted
            tool.render_at_position(context, self.grid_position.clone().get())?;
            for cp in tool.get_connection_points() {
                let cp1 = &cp.get_absolute_at_position(self.grid_position.clone().get());
                for cp2 in self.connection_points.borrow().iter() {
                    if self.check_connection_points(cp1, cp2) {
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
    // FIXME: Connections are currently made trough elements, which is not wanted
    fn render_connections(&self, context: &CanvasRenderingContext2d) {
        for cp1 in self.connection_points.borrow().iter() {
            for cp2 in self.connection_points.borrow().iter() {
                if self.check_connection_points(cp1, cp2) {
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
    fn check_connection_points(&self, cp1: &ConnectionPoint, cp2: &ConnectionPoint) -> bool {
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
            for canvas_element in self.canvas_elements.borrow().iter() {
                if Self::check_if_crosses(cp1, cp2, canvas_element) {
                    return false;
                }
            }
            return true;
        }
        false
    }

    fn check_if_crosses(
        cp1: &ConnectionPoint,
        cp2: &ConnectionPoint,
        canvas_element: &CanvasElement,
    ) -> bool {
        let x1 = cp1.get_position_x();
        let y1 = cp1.get_position_y();
        let x2 = cp2.get_position_x();
        let y2 = cp2.get_position_y();
        let (x3, y3) = canvas_element.get_position();
        let h3 = canvas_element.get_height();
        let w3 = canvas_element.get_width();

        const ERROR_ROOM: f64 = 0.01f64;
        x1 < x3 && x2 > x3 && y1 > (y3-ERROR_ROOM) && y1 < (y3 + h3+ERROR_ROOM)// line crosses horizontally
            || x2 < x3 && x1 > x3 && y2 > (y3-ERROR_ROOM) && y2 < (y3 + h3+ERROR_ROOM) // line crosses horizontally
            || y1 < y3 && y2 > y3 && x1 > (x3-ERROR_ROOM) && x1 < (x3 + w3+ERROR_ROOM)// line crosses vertically
            || y2 < y3 && y1 > y3 && x2 > (x3-ERROR_ROOM) && x2 < (x3 + w3+ERROR_ROOM) // line crosses vertically
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

impl CanvasRenderer for Workarea {
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
