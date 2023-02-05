use stylist::style;
use yew::{html, html_nested, virtual_dom::VChild, Callback, Classes, Component, MouseEvent, Properties};
use yew_icons::{Icon, IconId};

use super::bar::BarVertical;
#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes
}
pub struct Toolbar {}

impl Component for Toolbar {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let classes = ctx.props().class.clone();
        let style_toolbar = style!(
            r#"
            margin-left: 10px;
            margin-right: 10px;
            display: flex;
        "#
        )
        .unwrap();
        html! {
            <div class={ classes }>
                <div class={ style_toolbar}>
                    { Self::get_icon_for_tool(IconId::LucideFilePlus2, "New Project", false) }
                    { Self::get_icon_for_tool(IconId::LucideFolderOpen, "Open", false) }
                    { Self::get_icon_for_tool(IconId::LucideFileInput, "Import", false) }
                    { Self::get_icon_for_tool(IconId::LucideFileOutput, "Export", false) }
                    { Self::get_icon_for_tool(IconId::LucideSave, "Save", false) }
                    <BarVertical length="100%" thickness="2px" />
                    { Self::get_icon_for_tool(IconId::LucideUndo2, "Undo", false) }
                    { Self::get_icon_for_tool(IconId::LucideRedo2, "Redo", false) }
                    { Self::get_icon_for_tool(IconId::LucideCopy, "Copy", false) }
                    { Self::get_icon_for_tool(IconId::LucideScissors, "Cut", false) }
                    { Self::get_icon_for_tool(IconId::LucideClipboardList, "Paste", false) }
                    { Self::get_icon_for_tool(IconId::LucideTrash2, "Delete", false) }
                    // { Self::get_icon_for_tool(IconId::LucideX, "Stop action", false) }
                    <BarVertical length="100%" thickness="2px" />
                    { Self::get_icon_for_tool(IconId::LucideZoomIn, "Zoom in", false) }
                    { Self::get_icon_for_tool(IconId::LucideZoomOut, "Zoom out", false) }
                    { Self::get_icon_for_tool(IconId::LucideCrop, "Fit on screen", false) }
                    { Self::get_icon_for_tool(IconId::LucideGrid, "Toggle Grid", false) }
                    <BarVertical length="100%" thickness="2px" />
                    { Self::get_icon_for_tool(IconId::LucideRotateCcw, "Rotate left", false) }
                    { Self::get_icon_for_tool(IconId::LucideRotateCw, "Rotate right", false) }
                    { Self::get_icon_for_tool(IconId::LucideFlipHorizontal2, "Flip horizontal", false) }
                    { Self::get_icon_for_tool(IconId::LucideFlipVertical2, "Flip vertical", false) }
                    // { Self::get_icon_for_tool(IconId::LucideGitFork, "Connections", false) }
                    { Self::get_icon_for_tool(IconId::LucideNetwork, "Connections", false) }
                    { Self::get_icon_for_tool(IconId::LucideType, "Add Text", false) }
                    <BarVertical length="100%" thickness="2px" />
                    { Self::get_icon_for_tool(IconId::LucidePower, "Start Simulation", false) }
                    { Self::get_icon_for_tool(IconId::LucidePlay, "Continue Simulation", false) }
                    { Self::get_icon_for_tool(IconId::LucideStopCircle, "Stop Simulation", false) }
                    // { Self::get_icon_for_tool(IconId::LucidePause, "Pause Simulation", false) }
                    { Self::get_icon_for_tool(IconId::LucideSkipForward, "Step Simulation", false) }
                    { Self::get_icon_for_tool(IconId::LucideRewind, "Restart Simulation", false) }
                    { Self::get_icon_for_tool(IconId::LucideTimerReset, "Run Simulation until next change", false) }
                </div>
            </div>
        }
    }
}

impl Toolbar {
    fn get_icon_for_tool(icon_id: IconId, title: &'static str, active: bool) -> VChild<Icon> {
        let mut classes = Classes::with_capacity(2);
        let tool = style!(
            r#"
            margin: 0px 5px;

            :hover {
                cursor: pointer;
            }
        "#
        )
        .unwrap();

        let deactivated = style!(
            r#"
            color: lightgray;
            
            :hover {
                cursor: default;
            }
        "#
        )
        .unwrap();

        let onclick = Self::get_onclick_for_tool(icon_id);
        classes.push(tool);

        if onclick.is_none() || !active {
            classes.push(deactivated);
        }

        html_nested! { <Icon class={ classes } { icon_id } { title } { onclick }/> }
    }

    #[allow(clippy::match_same_arms)]
    ///This is a utility function providing the different onlick functions for the tool icons
    const fn get_onclick_for_tool(icon_id: IconId) -> Option<Callback<MouseEvent>> {
        match icon_id {
            //Some(Callback::from(|_| {}))
            IconId::LucideFilePlus2 => None,
            IconId::LucideFolderOpen => None,
            IconId::LucideSave => None,
            IconId::LucideUndo2 => None,
            IconId::LucideRedo2 => None,
            IconId::LucideCopy => None,
            IconId::LucideScissors => None,
            IconId::LucideClipboardList => None,
            IconId::LucideTrash2 => None,
            IconId::LucideX => None,
            IconId::LucideZoomIn => None,
            IconId::LucideZoomOut => None,
            IconId::LucideCrop => None,
            IconId::LucideRotateCcw => None,
            IconId::LucideRotateCw => None,
            IconId::LucideFlipHorizontal2 => None,
            IconId::LucideFlipVertical2 => None,
            // IconId::LucideGitFork => None,
            IconId::LucideNetwork => None,
            IconId::LucideType => None,
            IconId::LucidePower => None,
            IconId::LucidePlay => None,
            IconId::LucideStopCircle => None,
            IconId::LucidePause => None,
            IconId::LucideSkipForward => None,
            IconId::LucideRewind => None,
            IconId::LucideTimerReset => None,
            _ => None,
        }
    }
}
