use stylist::style;
use yew::{function_component, html, Html, Properties};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct HorizontalProps {
    /// The width of the line
    #[prop_or("1px")]
    pub thickness: &'static str,
    /// The length of the line
    #[prop_or("90%")]
    pub length: &'static str,
    /// The height of the entire element including the line width
    #[prop_or("11px")]
    pub height: &'static str,
    /// The width of the entire element
    #[prop_or("100%")]
    pub width: &'static str,
    /// The Style of the line
    #[prop_or("solid")]
    pub style: &'static str,
    ///The color of the line
    #[prop_or("gray")]
    pub color: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct VerticalProps {
    /// The width of the line
    #[prop_or("1px")]
    pub thickness: &'static str,
    /// The length of the line
    #[prop_or("90%")]
    pub length: &'static str,
    /// The height of the entire element including the line width
    #[prop_or("100%")]
    pub height: &'static str,
    /// The width of the entire element
    #[prop_or("11px")]
    pub width: &'static str,
    /// The Style of the line
    #[prop_or("solid")]
    pub style: &'static str,
    ///The color of the line
    #[prop_or("gray")]
    pub color: &'static str,
}

#[function_component]
pub fn BarHorizontal(props: &HorizontalProps) -> Html {
    let container_style = style!(
        r#"
        background: none;
        width: ${w};
        margin: calc((${h} - ${s}) / 2) 0;
        display: flex;
        flex-direction: row;
        justify-content: center;
    "#,
        s = props.thickness,
        h = props.height,
        w = props.width,
    )
    .unwrap();

    let bar_style = style!(
        r#"
        height: 0;
        width: ${w};
        border-bottom: ${t} ${s} ${c};
    "#,
        t = props.thickness,
        s = props.style,
        c = props.color,
        w = props.length
    )
    .unwrap();
    html! {
        <div class={ container_style }>
            <div class={ bar_style }>
            </div>
        </div>
    }
}

#[function_component]
pub fn BarVertical(props: &VerticalProps) -> Html {
    let container_style = style!(
        r#"
        background: none;
        width: 0;
        height: ${h};
        margin: 0 calc((${w} - ${s}) / 2);
        display: flex;
        flex-direction: column;
        justify-content: center;
    "#,
        s = props.thickness,
        h = props.height,
        w = props.width,
    )
    .unwrap();

    let bar_style = style!(
        r#"
        height: ${h};
        width: 0;
        border-right: ${t} ${s} ${c};
    "#,
        t = props.thickness,
        s = props.style,
        c = props.color,
        h = props.length
    )
    .unwrap();
    html! {
        <div class={ container_style }>
            <div class={ bar_style }>
            </div>
        </div>
    }
}
