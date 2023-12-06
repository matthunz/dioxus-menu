use dioxus_menu::{prelude::*, Menu};
use tray_icon::TrayIconEvent;
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn menu(cx: Scope) -> Element {
    cx.render(rsx! {
        item {
            accelerator: "CMD+O",
            enabled: true,
            "Open"
        }
        item {
            accelerator: "CMD+S",
            "Save"
        }
        item {
            accelerator: "CMD+Q",
            "Quit"
        }
    })
}

fn main() {
    let event_loop = EventLoopBuilder::new().build().unwrap();

    let mut menu = Menu::new(load_icon(), menu);
    menu.rebuild();
    dbg!(&menu);

    let tray_channel = TrayIconEvent::receiver();

    event_loop
        .run(move |_event, event_loop| {
            event_loop.set_control_flow(ControlFlow::Poll);

            if let Ok(event) = tray_channel.try_recv() {
                println!("{event:?}");
            }
        })
        .unwrap();

    drop(menu);
}

fn load_icon() -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
