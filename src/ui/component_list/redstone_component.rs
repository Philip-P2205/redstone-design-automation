use stylist::style;
use yew::{html, Component, Properties};
use yew_icons::{Icon, IconId};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RedstoneComponentProps {
    #[prop_or(IconId::LucideFileImage)]
    icon_id: IconId,
    #[prop_or("Component")]
    title: &'static str,
}

pub struct RedstoneComponent;

impl Component for RedstoneComponent {
    type Message = ();
    type Properties = RedstoneComponentProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let style_redstone_component_inner = style!(
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
        let style_image = style!(
            r#"
            align-self: center;
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
        html! {
            <div class={ style_redstone_component_inner }>
                <Icon icon_id={ ctx.props().icon_id } title="Missing icon" width="50px" height="50px" class={ style_image }/>
                <span class={ style_name }>{ ctx.props().title }</span>
            </div>
        }
    }
}
