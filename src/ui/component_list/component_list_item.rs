use stylist::style;
use yew::{html, Callback, Properties};
use yew_icons::IconId;

use crate::ui::{
    console_option::ConsoleOption,
    redstone_component::ComponentType,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ComponentListItemProps {
    #[prop_or(IconId::LucideFileImage)]
    pub icon_id: IconId,
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or_default]
    pub onclick: Callback<ComponentType>,
    pub component_type: ComponentType,
}

pub struct ComponentListItem;

impl yew::html::Component for ComponentListItem {
    type Message = ();
    type Properties = ComponentListItemProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let style_component_list_item_inner = style!(
            r#"
            width: 104px;
            height: 94px;
            padding-bottom: 10px;
            border-right: 1px solid #eee;
            border-bottom: 1px solid #eee;
            display: flex;
            flex-direction: column;
            justify-content: end;
            cursor: pointer;

            :hover {
                background: #f5f5f5;
            }
        "#
        )
        .unwrap();
        let style_name = style!(
            r#"
            width: 100%;
            text-align: center;
            align-self: end;
            user-select: none;
            margin-top: 10px;
        "#
        )
        .unwrap();

        let onclick = ctx.props().onclick.clone();
        let component_type = ctx.props().component_type;

        let component = component_type.try_as_component().unwrap_to_console();

        let title = if ctx.props().title.is_empty() {
            component.get_component_list_item_title()
        } else {
            ctx.props().title.to_owned()
        };

        
        html! {
            <div class={ style_component_list_item_inner } onclick={ move |_| onclick.emit(component_type) }>

                // <Icon icon_id={ ctx.props().icon_id } title="Missing icon" width="50px" height="50px" class={ style_image }/>
                { component.get_component_list_item_icon() }
                <span class={ style_name }>{ title }</span>
            </div>
        }
    }
}
