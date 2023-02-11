use std::{cell::RefCell, rc::Rc};

use stylist::{css, style, yew::Global};
use yew::prelude::*;

use crate::impl_display_with_debug;

use super::{
    canvas::CanvasElement, component_list::ComponentList, console_option::ConsoleOption,
    keybard_input_handler::KeyboardInputHandler, main_header::MainHeader, toolbar::Toolbar,
    workspace::Workspace,
};

#[derive(Clone)]
pub enum CallbackReason {
    ToolChanged(Option<CanvasElement>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    // File
    New,
    Open,
    Save,
    _SaveAs,
    _Import,
    _Export,
    Print,
    Close,
    Exit,

    // Edit
    Undo,
    Redo,
    Copy,
    Cut,
    Paste,
    Delete,

    // View
    ZoomIn,
    ZoomOut,
    FitWindow,
    ToggleGrid,

    // Tools
    ExitCurrentTool,
    RotateLeft,
    RotateRight,
    MirrorVertical,
    MirrorHorizontal,
    MakeConnections,
    PlaceText,
    // - Simulation
    _StartSimulation,
    _RunSimulaiton,
    _StopSimulation,
    _StepSimulaiton,
    _RestartSimulation,
    _PauseAtChangeSimulation,
}
impl_display_with_debug!(Command);

pub enum ApplicationMsg {
    Callback(CallbackReason),
    Command(Command),
}

#[derive(Default, Clone, PartialEq)]
pub struct ApplicationState {
    pub tool_active: Option<CanvasElement>,
}

pub struct Application {
    application_state: Rc<RefCell<ApplicationState>>,
    _keyboard_handler: KeyboardInputHandler,
}

impl Component for Application {
    type Message = ApplicationMsg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let keyboard_handler = {
            let callback = ctx.link().callback(ApplicationMsg::Command);
            KeyboardInputHandler::new(callback)
        };
        Self {
            application_state: Rc::default(),
            _keyboard_handler: keyboard_handler,
        }
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApplicationMsg::Callback(_reason) => match _reason {
                CallbackReason::ToolChanged(tool) => {
                    self.application_state.borrow_mut().tool_active = tool;
                }
            },
            ApplicationMsg::Command(cmd) => {
                log::debug!("Command issued: {}", cmd);

                match cmd {
                    Command::ExitCurrentTool => {
                        self.application_state.borrow_mut().tool_active = None;
                    }
                    _ => {},
                }
            }
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        //TODO: Iron out the exact positions of the elements
        let page = style!(
            r#"
            display: grid;
            grid-template-columns: 225px auto;
            grid-template-rows: 25px 50px 40fr;
            grid-template-areas: "main_header main_header"
                                 "toolbar toolbar"
                                 "component_list workspace";
            width: 100%;
            height: 100%;
        "#
        )
        .unwrap_to_console();
        let main_header = style!(
            r#"
            background: #fff;
            grid-area: main_header;
            display: flex;
            flex-direction: column;
            justify-content: center;
        "#
        )
        .unwrap_to_console();
        let toolbar = style!(
            r#"
            background: #fff;
            grid-area: toolbar;
            display: flex;
            flex-direction: column;
            justify-content: center;
            border-bottom: 1px solid gray;
        "#
        )
        .unwrap_to_console();
        let component_list = style!(
            r#"
            grid-area: component_list;
        "#
        )
        .unwrap_to_console();
        let workspace = style!(
            r#"
            grid-area: workspace;
        "#
        )
        .unwrap_to_console();

        let main_header_callback = ctx.link().callback(ApplicationMsg::Callback);
        let toolbar_callback = ctx.link().callback(ApplicationMsg::Callback);
        let component_list_callback = ctx.link().callback(ApplicationMsg::Callback);
        let workspace_callback = ctx.link().callback(ApplicationMsg::Callback);

        html! {
            <div id ="page">
                <Global css={ css!(r#"font-family: Arial,"Microsoft YaHei";"#) }/>

                <div class={page}>
                    <MainHeader callback={ main_header_callback } class={ main_header }/>
                    <Toolbar callback={ toolbar_callback } class={toolbar}/>
                    <ComponentList callback={ component_list_callback }class={component_list}/>
                    <Workspace callback={ workspace_callback } application_state={ self.application_state.clone() } class={ workspace }/>
                </div>
            </div>
        }
    }
}
