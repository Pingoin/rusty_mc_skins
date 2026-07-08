use dioxus::prelude::*;

pub(crate) fn alert(message: String) {
let js_string=format!("alert(\"{}\")", message);
    document::eval(&js_string);
}