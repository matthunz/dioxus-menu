use dioxus::core::{Component, VirtualDom};

pub struct Menu {
    vdom: VirtualDom,
}

impl Menu {
    pub fn new(app: Component) -> Self {
        Self {
            vdom: VirtualDom::new(app),
        }
    }

    pub fn rebuild(&mut self) {
        let mutations = self.vdom.rebuild();
        dbg!(mutations);
    }
}

pub mod dioxus_elements {
    use dioxus::html::AttributeDiscription;

    #[allow(non_camel_case_types)]
    pub struct item;

    impl item {
        pub const TAG_NAME: &'static str = "item";
        pub const NAME_SPACE: Option<&'static str> = None;

        pub const accelerator: AttributeDiscription =("accelerator", None, false);
    }
}

pub mod prelude {
    pub use crate::dioxus_elements::{self, *};

    pub use dioxus::prelude::{Element, Scope, rsx};
}
