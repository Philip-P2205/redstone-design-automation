use stylist::style;
/**
 * This struct shows a list of available components in the application
 */
use yew::{html, Callback, Classes, Properties};

use crate::ui::{
    application::CallbackReason,
    canvas::IntoCanvasElement,
    components::{
        logic_gate::{LogicGate, LogicGateType},
        pin::Pin,
    },
    console_option::ConsoleOption,
    redstone_component::ComponentType,
};

use super::component_list_item::ComponentListItem;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ComponentListProps {
    #[prop_or_default]
    pub class: Classes,
    pub callback: Callback<CallbackReason>,
}

pub enum ComponentListMsg {
    ComponentClicked(ComponentType),
}

pub struct ComponentList {}

impl yew::html::Component for ComponentList {
    type Message = ComponentListMsg;
    type Properties = ComponentListProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        use ComponentListMsg::*;
        match _msg {
            ComponentClicked(component_type) => match component_type {
                ComponentType::LogicGate(gate_type) => {
                    _ctx.props().callback.emit(CallbackReason::ToolChanged(Some(
                        LogicGate::try_from(gate_type)
                            .unwrap_to_console()
                            .into_canvas_element((0.0, 0.0)),
                    )));
                }
                ComponentType::Pin => {
                    _ctx.props().callback.emit(CallbackReason::ToolChanged(Some(
                        Pin::new()
                            .unwrap_to_console()
                            .into_canvas_element((0.0, 0.0)),
                    )))
                }
            },
        }
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let mut classes = ctx.props().class.clone();
        let style_component_list_outer = style!(
            r#"
            height: 100%;
            border-right: 1px solid gray;
        "#
        )
        .unwrap();
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
        let onclick = ctx.link().callback(ComponentListMsg::ComponentClicked);
        html! {
            <div class={ classes }>
                <div class={ style_component_list_inner }>
                    <ComponentListItem component_type={ ComponentType::LogicGate(LogicGateType::And) } onclick={ onclick.clone() } />
                    <ComponentListItem component_type={ ComponentType::LogicGate(LogicGateType::Or) } onclick={ onclick.clone() } />
                    <ComponentListItem component_type={ ComponentType::LogicGate(LogicGateType::Xor) } onclick={ onclick.clone() } />
                    <ComponentListItem component_type={ ComponentType::LogicGate(LogicGateType::Nand) } onclick={ onclick.clone() } />
                    <ComponentListItem component_type={ ComponentType::LogicGate(LogicGateType::Nor) } onclick={ onclick.clone() } />
                    <ComponentListItem component_type={ ComponentType::Pin } onclick={ onclick.clone() } />
                </div>
            </div>
        }
    }
}
