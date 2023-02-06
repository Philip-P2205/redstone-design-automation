use stylist::style;
use yew::{html, Classes, Component, Properties};

use super::{super::{
    canvas::{element::Element, Canvas},
    console_option::ConsoleOption,
}, workarea::Workarea};

pub const GRID_SIZE: f64 = 25.0;
pub const GRID_SIZE_PROPS: &str = "25px";

pub struct Workspace {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    /// The size of the grid
    #[prop_or(GRID_SIZE_PROPS)]
    pub grid_size: &'static str,
    #[prop_or_default]
    pub selected_tool: Option<Element>,
}

impl Component for Workspace {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, _msgg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        // TODO: Make background responsive
        let style_workspace = style!(
            r#"
            width: 100%;
            height: 100%;
            display: grid;
            grid-template-rows: 22px 1fr;
            grid-template-columns: 22px 1fr;
            grid-template-areas: "ruler_corner ruler_top"
                                 "ruler_side workarea";
        "#
        )
        .unwrap_to_console();
        let mut classes = ctx.props().class.clone();

        let style_workarea = style!(r#"
            width: 100%;
            height: 100%;
            grid-area: workarea;
            overflow: hidden;
            background-image: linear-gradient(rgba(247, 247, 247, 1.0) .1em, transparent .1em), linear-gradient(90deg, rgba(247, 247, 247, 1.0) .1em, transparent .1em);
            background-size: ${grid_size} ${grid_size};
        "#,
        grid_size= ctx.props().grid_size
    ).unwrap_to_console();
        classes.push(style_workspace);
        let workarea = Workarea::new().unwrap_to_console();

        html! (
            <div class={ classes }>
                <div class={ style_workarea }>
                    <Canvas<Workarea> renderer={ Box::new(workarea) } width={ Workarea::get_width() } height={ Workarea::get_height() }>
                    </Canvas<Workarea>>
                </div>
            </div>
        )
    }
}
