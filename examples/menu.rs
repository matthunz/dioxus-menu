use dioxus_menu::{prelude::*, Menu};

fn menu(cx: Scope) -> Element {
    cx.render(rsx!{ 
        item {
            accelerator: "CMD+Q",
            "Quit"
        }
    })
}

fn main() {
    let mut menu = Menu::new(menu);
    menu.rebuild();
}