use std::{cell::RefCell, rc::Rc};

use gloo::utils::document;
use js_sys::Function;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::KeyboardEvent;
use yew::Callback;

use super::{application::Command, console_option::ConsoleOption};

#[derive(Debug, Clone)]
struct KeyboardHandlerData {
    callback: Callback<Command>,
}

impl KeyboardHandlerData {
    fn new(callback: Callback<Command>) -> Self {
        Self { callback }
    }
}

#[derive(Debug)]
pub struct KeyboardInputHandler {
    _data: Rc<RefCell<KeyboardHandlerData>>,
}

impl KeyboardInputHandler {
    pub fn new(callback: Callback<Command>) -> Self {
        let data = Rc::new(RefCell::new(KeyboardHandlerData::new(callback.clone())));
        let keyup: Function = {
            let data = data.clone();
            Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                Self::keyup(event, data.clone())
            })
            .into_js_value()
            .unchecked_into()
        };
        let keydown: Function = {
            let data = data.clone();
            Closure::<dyn FnMut(KeyboardEvent)>::new(move |event| {
                Self::keydown(event, data.clone())
            })
            .into_js_value()
            .unchecked_into()
        };
        let keypress: Function = {
            let data = data.clone();
            Closure::<dyn FnMut(KeyboardEvent)>::new(move |event| {
                Self::keypress(event, data.clone())
            })
            .into_js_value()
            .unchecked_into()
        };

        document()
            .add_event_listener_with_callback("keyup", &keyup)
            .unwrap_to_console();
        document()
            .add_event_listener_with_callback("keypress", &keypress)
            .unwrap_to_console();
        document()
            .add_event_listener_with_callback("keydown", &keydown)
            .unwrap_to_console();
        Self { _data: data }
    }
    fn keyup(_event: KeyboardEvent, _data: Rc<RefCell<KeyboardHandlerData>>) {}
    fn keypress(_event: KeyboardEvent, _data: Rc<RefCell<KeyboardHandlerData>>) {}
    fn keydown(event: KeyboardEvent, data: Rc<RefCell<KeyboardHandlerData>>) {
        // gloo::console::info!(&event);
        let ctrl = event.ctrl_key();
        let alt = event.alt_key();
        let shift = event.shift_key();
        let key = event.key().to_uppercase();
        let cmd = SHORTCUTS
            .iter()
            .find(|s| s.ctrl == ctrl && s.alt == alt && s.shift == shift && s.key == key)
            .map(|s| s.cmd);
        if let Some(cmd) = cmd {
            event.prevent_default();
            data.borrow().callback.emit(cmd);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct KeyboardShortcut {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: &'static str,
    pub cmd: Command,
}

impl KeyboardShortcut {
    pub const fn new(ctrl: bool, alt: bool, shift: bool, key: &'static str, cmd: Command) -> Self {
        Self {
            ctrl,
            alt,
            shift,
            key,
            cmd,
        }
    }
}

const SHORTCUTS: &[KeyboardShortcut] = &[
    KeyboardShortcut::new(true, false, false, "S", Command::Save),
    KeyboardShortcut::new(true, false, false, "N", Command::New),
    KeyboardShortcut::new(true, false, false, "O", Command::Open),
    KeyboardShortcut::new(true, false, false, "P", Command::Print),
    KeyboardShortcut::new(true, false, false, "W", Command::Close),
    KeyboardShortcut::new(false, true, false, "F4", Command::Exit),
    KeyboardShortcut::new(true, false, false, "Z", Command::Undo),
    KeyboardShortcut::new(true, false, false, "Y", Command::Redo),
    KeyboardShortcut::new(true, false, false, "C", Command::Copy),
    KeyboardShortcut::new(true, false, false, "V", Command::Paste),
    KeyboardShortcut::new(true, false, false, "X", Command::Cut),
    KeyboardShortcut::new(false, false, false, "DELETE", Command::Delete),
    KeyboardShortcut::new(true, false, false, "+", Command::ZoomIn),
    KeyboardShortcut::new(true, false, false, "-", Command::ZoomOut),
    KeyboardShortcut::new(true, false, true, "F", Command::FitWindow),
    KeyboardShortcut::new(true, false, true, "G", Command::ToggleGrid),
    KeyboardShortcut::new(false, false, false, "ESCAPE", Command::ExitCurrentTool),
    KeyboardShortcut::new(false, false, false, "R", Command::RotateLeft),
    KeyboardShortcut::new(false, false, true, "R", Command::RotateRight),
    KeyboardShortcut::new(true, false, false, "M", Command::MirrorVertical),
    KeyboardShortcut::new(true, false, true, "M", Command::MirrorHorizontal),
    KeyboardShortcut::new(false, false, true, "C", Command::MakeConnections),
    KeyboardShortcut::new(true, false, false, "T", Command::PlaceText),
];
