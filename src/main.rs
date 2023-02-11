use ui::application::Application;

mod ui;

#[macro_export]
macro_rules! impl_display_with_debug {
    () => {};
    ($item: ident) => {
        impl std::fmt::Display for $item {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }
    };
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Application>::new().render();
}
