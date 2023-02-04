use std::fmt::{Debug, Display};

use log::debug;
use stylist::style;
use yew::prelude::*;
use yew_icons::IconId;

use crate::ui::bar::BarHorizontal;

use super::{header_menu::HeaderMenu, menu_option::MenuOption};

pub enum MainHeaderMsg {
    HeaderClicked(HeaderOptionType),
    HeaderHovered(HeaderOptionType),
    HeaderClosed,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MainHeaderProps {
    #[prop_or_default]
    pub class: Classes
}

pub struct MainHeader {
    header_active: Option<HeaderOptionType>,
}

impl Component for MainHeader {
    type Message = MainHeaderMsg;
    type Properties = MainHeaderProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            header_active: None,
        }
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainHeaderMsg::HeaderClicked(header) => {
                if self.is_header_active(header.clone()) {
                    self.header_active = None;
                    debug!("New active header: {:?}", self.header_active);
                } else {
                    self.header_active = Some(header);
                    debug!("New active header: {:?}", self.header_active);
                }
                true
            }
            MainHeaderMsg::HeaderHovered(header) => {
                if self.header_active.is_some() {
                    self.header_active = Some(header);
                    true
                } else {
                    false
                }
            }
            MainHeaderMsg::HeaderClosed => {
                self.header_active = None;
                true
            }
        }
    }
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let classes = ctx.props().class.clone();
        let header = style!(
            r#"
            margin-left: 10px;
            margin-right: 10px;
            display: flex;
            flex-direction: row;
            align-items: center;
            align-content: center;
        "#
        )
        .unwrap();

        let option_cancel = style!(
            r#"
            position: absolute;
            left: 0;
            top: 25px;
            width: 100vw;
            height: calc(100vh - 25px);
            z-index: 1;
            overflow: hidden;
        "#
        )
        .unwrap();

        let onclick = ctx.link().callback(MainHeaderMsg::HeaderClicked);
        let onhover = ctx.link().callback(MainHeaderMsg::HeaderHovered);

        html! {
            <div class={ classes }>
                <div class={ header }>
                    if self.header_active.is_some() {
                        // FIXME: This div makes it impossible to interact with the rest of the program as long as it is active.
                        // This div sets the active_header to none if you click outside of the MainHeader/HeaderMenu
                        <div class={ option_cancel } onclick={ ctx.link().callback(|_| MainHeaderMsg::HeaderClosed) }></div>
                    }

                //TODO: Include logo
                    <HeaderOption typ={ HeaderOptionType::File } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active.clone() } />
                        <HeaderMenu width="200px" position_x="0px" active={ self.is_header_active(HeaderOptionType::File) }>
                            <MenuOption text="New" shortcut="Ctrl+N"/>
                            <MenuOption text="Open" shortcut="Ctrl+O"/>
                            <BarHorizontal />
                            <MenuOption text="Save" shortcut="Ctrl+S"/>
                            <MenuOption text="Save As" shortcut=""/>
                            <BarHorizontal />
                            <MenuOption text="Import" shortcut=""/>
                            <MenuOption text="Export" shortcut=""/>
                            <BarHorizontal />
                            <MenuOption text="Print" shortcut="Ctrl+P"/>
                            <BarHorizontal />
                            <MenuOption text="Close" shortcut="Ctrl+W"/>
                            <MenuOption text="Exit" shortcut="Alt+F4"/>
                        </HeaderMenu>

                    <HeaderOption typ={ HeaderOptionType::Edit } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active.clone() } />
                    <HeaderMenu width="200px" position_x="52px" active={ self.is_header_active(HeaderOptionType::Edit) }>
                        <MenuOption text="Undo" shortcut="Ctrl+Z" />
                        <MenuOption text="Redo" shortcut="Ctrl+Y" />
                        <BarHorizontal />
                        <MenuOption text="Copy" shortcut="Ctrl+C" />
                        <MenuOption text="Cut" shortcut="Ctrl+X" />
                        <MenuOption text="Pase" shortcut="Ctrl+V" />
                        <BarHorizontal />
                        <MenuOption text="Delete" shortcut="Del" />
                    </HeaderMenu>

                    <HeaderOption typ={ HeaderOptionType::View } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active.clone() } />
                    <HeaderMenu width="200px" position_x="95px" active={ self.is_header_active(HeaderOptionType::View) }>
                        <MenuOption text="Zoom In" />
                        <MenuOption text="Zoom Out" />
                        <MenuOption text="Fit Window" />
                        <MenuOption text="Show/Hide Grid" />
                    </HeaderMenu>

                    <HeaderOption typ={ HeaderOptionType::Tools } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active.clone() } />
                    <HeaderMenu width="250px" position_x="146px" active={ self.is_header_active(HeaderOptionType::Tools) }>
                        <MenuOption icon_id={ IconId::LucideRotateCcw } text="Rotate Left" shortcut="Ctrl+R"/>
                        <MenuOption text="Rotate Right" shortcut="Ctrl+Shift+R"/>
                        <MenuOption text="Mirror Vertical" shortcut="Ctrl+M"/>
                        <MenuOption text="Mirror Horizontal" shortcut="Ctrl+Shift+M"/>
                        <BarHorizontal />
                        <MenuOption text="Make Connections"/>
                        <BarHorizontal />
                        <MenuOption text="Start Simulation" />
                        <MenuOption text="Run Simulation" disabled=false />
                        <MenuOption text="Stop Simulation" disabled=false />
                        <MenuOption text="Step Simulation" disabled=false />
                        <MenuOption text="Restart Simulation" disabled=false />
                        <MenuOption text="Pause At Change" disabled=false />
                    </HeaderMenu>

                    <HeaderOption disabled=true typ={ HeaderOptionType::Options } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active.clone() } />
                    <HeaderOption disabled=true typ={ HeaderOptionType::Help } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active.clone() } />
                </div>
            </div>
        }
    }
}

impl MainHeader {
    fn is_header_active(&self, header: HeaderOptionType) -> bool {
        self.header_active.clone().map_or(false, |h| h == header)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
        write!(f, "{:?}", self)
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

pub struct HeaderOption {}

impl Component for HeaderOption {
    type Message = ();
    type Properties = HeaderOptionProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let active = ctx.props().active_header.as_ref().map_or(false, |h|h.eq(&ctx.props().typ));

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
            color = if ctx.props().disabled { "gray" } else {if active { "#7988ff" } else { "inherit" }},
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
        let typ = ctx.props().typ.clone();
        let onclick = ctx.props().onclick.reform(move |_| typ.clone());
        let typ = ctx.props().typ.clone();
        let onmouseover = ctx.props().onhover.reform(move |_| typ.clone());
        html! {
            <>
                <span class={ style } { onclick } { onmouseover }>{&ctx.props().typ}</span>
            </>
        }
    }
}
