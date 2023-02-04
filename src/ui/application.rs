use stylist::{css, style, yew::Global};
use yew::prelude::*;

use super::{
    component_list::component_list::ComponentList, main_header::main_header::MainHeader, toolbar::Toolbar,
    workspace::Workspace,
};

pub struct Application {}

impl Component for Application {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, _msgg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
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
        .unwrap();
        let main_header = style!(
            r#"
            background: #fff;
            grid-area: main_header;
            display: flex;
            flex-direction: column;
            justify-content: center;
        "#
        )
        .unwrap();
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
        .unwrap();
        let component_list = style!(
            r#"
            grid-area: component_list;
        "#
        )
        .unwrap();
        let workspace = style!(
            r#"
            grid-area: workspace;
        "#
        )
        .unwrap();
        // TODO: Add application parts
        html! {
            <div id ="page">
                <Global css={ css!(r#"font-family: Arial,"Microsoft YaHei";"#) }/>

                <div class={page}>
                    <MainHeader class={main_header}/>
                    <Toolbar class={toolbar}/>
                    <ComponentList class={component_list}/>
                    <Workspace class={ workspace }/>
                </div>
            </div>
        }
    }
}

pub enum Message {}
