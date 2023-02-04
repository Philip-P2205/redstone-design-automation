use ui::application::Application;

mod ui;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Application>::new().render();
}