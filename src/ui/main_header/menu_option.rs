use log::error;
use stylist::style;
use yew::{function_component, html, Callback, MouseEvent, Properties};
use yew_icons::IconId;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuOptionProperties {
    pub text: &'static str,
    #[prop_or(None)]
    pub icon_id: Option<IconId>,
    #[prop_or(None)]
    pub shortcut: Option<&'static str>,
    #[prop_or(None)]
    pub callback: Option<Callback<MouseEvent>>,
    #[prop_or(false)]
    pub disabled: bool,
}
#[function_component(MenuOption)]
pub fn menu_option(props: &MenuOptionProperties) -> yew::Html {
    let style_outer = style!(
        r#"
        padding: 0px 10px;
        height: 25px;
        display: flex;
        flex-direction: column;
        justify-content: center;

        border-radius: 5px;

        :hover {
            cursor: ${cursor_hover};
            background: ${background_hover};
        }
    "#,
        cursor_hover = if props.disabled { "default" } else { "pointer" },
        background_hover = if props.disabled {
            "inherit"
        } else {
            "dodgerblue"
        }
    )
    .unwrap();
    let style_inner = style!(
        r#"
        display: grid;
        grid-template-columns: ${grid_template};
        color: ${txt_color};
    "#,
        grid_template = if props.shortcut.is_none() {
            "1fr"
        } else {
            "2fr 1fr"
        },
        txt_color = if props.disabled { "gray" } else { "inherit" }
    )
    .unwrap();
    let style_text = style!(
        r#"
        user-select: none;
    "#
    )
    .unwrap();
    let style_shortcut = style!(
        r#"
        text-align: right;
        user-select: none;
    "#
    )
    .unwrap();
    let callback = props.callback.clone().unwrap_or(Callback::from(|_| {
        error!(
            r#"Missing callback for MenuOption "{}"!"#,
            props.text.to_owned()
        );
    }));
    html! {
        <div class={ style_outer } onclick={ callback }>
            <div class={ style_inner }>
                    <span class={ style_text }>{ props.text }</span>
                if let Some(shortcut) = props.shortcut {
                    <span class={ style_shortcut }>{ shortcut }</span>
                }
            </div>
        </div>
    }
}
