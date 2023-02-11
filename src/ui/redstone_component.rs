use wasm_bindgen::JsValue;
use yew::Html;

use crate::ui::{components::logic_gate::LogicGate};

use super::{
    canvas::IntoCanvasElement, components::{logic_gate::LogicGateType, pin::Pin},
    connection_point::ConnectionPoint,
};

pub trait RedstoneComponent: IntoCanvasElement {
    fn get_connection_points(&self) -> Vec<ConnectionPoint>;
    fn get_component_type(&self) -> ComponentType;
    fn get_component_list_item_title(&self) -> String;
    fn get_component_list_item_icon(&self) -> Html;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ComponentType {
    LogicGate(LogicGateType),
    Pin
}

impl ComponentType {
    pub fn try_as_component(&self) -> Result<Box<dyn RedstoneComponent>, JsValue> {
        match self {
            ComponentType::LogicGate(gate_type) => {
                Ok(Box::new(LogicGate::new(*gate_type)?))
            }
            ComponentType::Pin => {
                Ok(Box::new(Pin::new()?))
            }
        }
    }
}
