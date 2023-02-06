use gloo::console::debug;
use stylist::{css, style, yew::Global};
use yew::prelude::*;

use super::{
    component_list::{self, ComponentList},
    console_option::ConsoleOption,
    main_header::{self, MainHeader},
    toolbar::{self, Toolbar},
    workspace::{self, Workspace},
};
pub enum ApplicationMsg {
    MainHeaderCallback(main_header::CallbackReason),
    ToolbarCallback(toolbar::CallbackReason),
    ComponentListCallback(component_list::CallbackReason),
    WorkspaceCallback(workspace::CallbackReason),
}

pub struct Application {}

impl Component for Application {
    type Message = ApplicationMsg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        use ApplicationMsg::{
            ComponentListCallback, MainHeaderCallback, ToolbarCallback, WorkspaceCallback,
        };
        match msg {
            ComponentListCallback(reason) => {
                debug!(
                    "ComponentList emitted callback with reason: ",
                    reason.to_string()
                );
            }
            MainHeaderCallback(reason) => {
                debug!(
                    "MainHeader emitted callback with reason: ",
                    reason.to_string()
                );
            }
            ToolbarCallback(reason) => {
                debug!("Toolbar emitted callback with reason: ", reason.to_string());
            }
            WorkspaceCallback(reason) => {
                debug!(
                    "Workspace emitted callback with reason: ",
                    reason.to_string()
                );
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

        let main_header_callback = ctx.link().callback(ApplicationMsg::MainHeaderCallback);
        let toolbar_callback = ctx.link().callback(ApplicationMsg::ToolbarCallback);
        let component_list_callback = ctx.link().callback(ApplicationMsg::ComponentListCallback);
        let workspace_callback = ctx.link().callback(ApplicationMsg::WorkspaceCallback);

        html! {
            <div id ="page">
                <Global css={ css!(r#"font-family: Arial,"Microsoft YaHei";"#) }/>

                <div class={page}>
                    <MainHeader callback={ main_header_callback } class={ main_header }/>
                    <Toolbar callback={ toolbar_callback } class={toolbar}/>
                    <ComponentList callback={ component_list_callback }class={component_list}/>
                    <Workspace callback={ workspace_callback } class={ workspace }/>
                </div>
            </div>
        }
    }
}
