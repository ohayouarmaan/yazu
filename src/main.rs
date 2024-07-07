use leptos::*;
use crate::components::App;
mod components;
mod parser;
mod lexer;
mod transpiler;

fn main() {
    mount_to_body(|| view! {
        <App/>
    });
}
