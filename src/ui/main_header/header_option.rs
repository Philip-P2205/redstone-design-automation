use std::fmt::Display;

use stylist::style;
use yew::{html, Callback, Component, Properties};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeaderOptionType {
    File,
    Edit,
    View,
    Tools,
    Options,
    Help,
}
impl Display for HeaderOptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct HeaderOptionProps {
    pub typ: HeaderOptionType,
    pub onclick: Callback<HeaderOptionType>,
    #[prop_or_default]
    pub onhover: Callback<HeaderOptionType>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub active_header: Option<HeaderOptionType>,
}

pub struct HeaderOption;

impl Component for HeaderOption {
    type Message = ();
    type Properties = HeaderOptionProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let active = ctx
            .props()
            .active_header
            .as_ref()
            .map_or(false, |h| h.eq(&ctx.props().typ));

        let style = style!(
            r#"
            cursor: ${cursor};
            padding: 3px 8px;
            margin: 0;
            border-radius: 5px;
            user-select: none;

            background: ${background};
            color: ${color};

        :hover {
            background: ${background_hover};
            color: ${color_hover};
        }
        "#,
            cursor = if ctx.props().disabled {
                "default"
            } else {
                "pointer"
            },
            background = if active { "#f5f5f5" } else { "inherit" },
            color = if ctx.props().disabled {
                "gray"
            } else if active {
                "#7988ff"
            } else {
                "inherit"
            },
            background_hover = if ctx.props().disabled {
                "inherit"
            } else {
                "#f5f5f5"
            },
            color_hover = if ctx.props().disabled {
                "gray"
            } else {
                "#7988ff"
            }
        )
        .unwrap();
        let typ = ctx.props().typ;
        let onclick = ctx.props().onclick.reform(move |_| typ);
        let onmouseover = ctx.props().onhover.reform(move |_| typ);
        html! {
            <>
                <span class={ style } { onclick } { onmouseover }>{&ctx.props().typ}</span>
            </>
        }
    }
}
