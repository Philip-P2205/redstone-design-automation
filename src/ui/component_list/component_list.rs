use stylist::style;
/**
 * This struct shows a list of available components in the application
 */
use yew::{html, Component, Properties, Classes};

use super::redstone_component::RedstoneComponent;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ComponentListProps {
    #[prop_or_default]
    pub class: Classes
}

pub struct ComponentList {}

impl Component for ComponentList {
    type Message = ();
    type Properties = ComponentListProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut classes = ctx.props().class.clone();
        let style_component_list_outer = style!(r#"
            height: 100%;
            border-right: 1px solid gray;
        "#).unwrap();
        classes.push(style_component_list_outer);
        let style_component_list_inner = style!(
            r#"
            padding-left: 7.5px;
            padding-right: 7.5px;
            display: grid;
            grid-template-columns: 1fr 1fr;
        "#
        )
        .unwrap();
        html! {
            <div class={ classes }>
                <div class={ style_component_list_inner }>
                    <RedstoneComponent/>
                    <RedstoneComponent/>
                    <RedstoneComponent/>
                    <RedstoneComponent/>
                    <RedstoneComponent/>
                    <RedstoneComponent/>
                </div>
            </div>
        }
    }
}
