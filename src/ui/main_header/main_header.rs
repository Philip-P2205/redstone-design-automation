use std::fmt::Debug;

use log::debug;
use stylist::style;
use yew::prelude::*;
use yew_icons::IconId;

use crate::ui::bar::BarHorizontal;

use super::{
    header_menu::HeaderMenu,
    header_option::{self, HeaderOption},
    menu_option::MenuOption,
};

pub enum Msg {
    Clicked(header_option::Type),
    Hovered(header_option::Type),
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

pub struct MainHeader {
    header_active: Option<header_option::Type>,
}

impl Component for MainHeader {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            header_active: None,
        }
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(header) => {
                if self.is_header_active(header) {
                    self.header_active = None;
                    debug!("New active header: {:?}", self.header_active);
                } else {
                    self.header_active = Some(header);
                    debug!("New active header: {:?}", self.header_active);
                }
                true
            }
            Msg::Hovered(header) => {
                if self.header_active.is_some() {
                    self.header_active = Some(header);
                    true
                } else {
                    false
                }
            }
            Msg::Closed => {
                self.header_active = None;
                true
            }
        }
    }

    #[allow(clippy::cognitive_complexity)]
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

        let onclick = ctx.link().callback(Msg::Clicked);
        let onhover = ctx.link().callback(Msg::Hovered);

        html! {
            <div class={ classes }>
                <div class={ header }>
                    if self.header_active.is_some() {
                        // FIXME: This div makes it impossible to interact with the rest of the program as long as it is active.
                        // This div sets the active_header to none if you click outside of the MainHeader/HeaderMenu
                        <div class={ option_cancel } onclick={ ctx.link().callback(|_| Msg::Closed) }></div>
                    }

                //TODO: Include logo
                    <HeaderOption typ={ header_option::Type::File } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active } />
                        <HeaderMenu width="200px" position_x="0px" active={ self.is_header_active(header_option::Type::File) }>
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

                    <HeaderOption typ={ header_option::Type::Edit } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active } />
                    <HeaderMenu width="200px" position_x="52px" active={ self.is_header_active(header_option::Type::Edit) }>
                        <MenuOption text="Undo" shortcut="Ctrl+Z" />
                        <MenuOption text="Redo" shortcut="Ctrl+Y" />
                        <BarHorizontal />
                        <MenuOption text="Copy" shortcut="Ctrl+C" />
                        <MenuOption text="Cut" shortcut="Ctrl+X" />
                        <MenuOption text="Pase" shortcut="Ctrl+V" />
                        <BarHorizontal />
                        <MenuOption text="Delete" shortcut="Del" />
                    </HeaderMenu>

                    <HeaderOption typ={ header_option::Type::View } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active } />
                    <HeaderMenu width="200px" position_x="95px" active={ self.is_header_active(header_option::Type::View) }>
                        <MenuOption text="Zoom In" />
                        <MenuOption text="Zoom Out" />
                        <MenuOption text="Fit Window" />
                        <MenuOption text="Show/Hide Grid" />
                    </HeaderMenu>

                    <HeaderOption typ={ header_option::Type::Tools } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active } />
                    <HeaderMenu width="250px" position_x="146px" active={ self.is_header_active(header_option::Type::Tools) }>
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

                    <HeaderOption disabled=true typ={ header_option::Type::Options } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active } />
                    <HeaderOption disabled=true typ={ header_option::Type::Help } onclick={ onclick.clone() } onhover={ onhover.clone() } active_header={ self.header_active } />
                </div>
            </div>
        }
    }
}

impl MainHeader {
    fn is_header_active(&self, header: header_option::Type) -> bool {
        self.header_active.map_or(false, |h| h == header)
    }
}
