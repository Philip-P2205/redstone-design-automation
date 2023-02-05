use wasm_bindgen::JsValue;

pub trait ConsoleOption<T> {
    fn expect_to_console(self, msg: &str) -> T;
    #[warn(clippy::unwrap_used)]
    fn unwrap_to_console(self) -> T;
}

impl<T, E> ConsoleOption<T> for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn expect_to_console(self, msg: &str) -> T {
        match self {
            Err(e) => {
                gloo::console::error!(JsValue::from(format!("{msg}: {e:?}")));
                panic!("{msg}: {e:?}");
            }
            Ok(o) => o,
        }
    }
    fn unwrap_to_console(self) -> T {
        self.expect_to_console("called `Result::unwrap_to_console()` on an `Err` value")
    }
    
}

impl<T> ConsoleOption<T> for Option<T> {
    fn expect_to_console(self, msg: &str) -> T {
        self.map_or_else(
            || {
                gloo::console::error!(msg);
                panic!("{msg}");
            },
            |v| v,
        )
    }
    fn unwrap_to_console(self) -> T {
        self.expect_to_console("called `Option::unwrap_to_console()` on a `None` value")
    }
}
