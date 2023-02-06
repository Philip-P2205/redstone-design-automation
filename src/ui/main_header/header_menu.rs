use stylist::style;
use yew::{function_component, html, Children, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct HeaderMenuProps {
    pub children: Children,
    #[prop_or("100%")]
    pub width: &'static str,
    #[prop_or("25px")]
    pub position_y: &'static str,
    #[prop_or("0px")]
    pub position_x: &'static str,
    #[prop_or(false)]
    pub active: bool
}

#[function_component]
pub fn HeaderMenu(props: &HeaderMenuProps) -> Html {
    let style = style!(
        r#"
        width: ${w};
        padding: 5px;
        background: #f9f9f9;
        border-radius: 5px;
        position: absolute;
        top: ${y};
        left: ${x};
        z-index: 2;
    "#,
        w = props.width,
        y = props.position_y,
        x = props.position_x
    )
    .unwrap();
    html! {
        if props.active {
            <div class={ style }>
                { props.children.clone() }
            </div>
        }
    }
}
