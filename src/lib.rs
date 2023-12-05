use dioxus::core::{Component, ElementId, Mutation, TemplateNode, VirtualDom};
use muda::MenuItemBuilder;
use slotmap::{DefaultKey, SlotMap};
use std::{collections::HashMap, fmt};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

#[derive(Debug)]
pub struct MenuTemplate {
    name: String,
    roots: Vec<DefaultKey>,
}

#[derive(Clone, Copy, Debug)]
pub enum ElementKind {
    Item,
}

#[derive(Debug)]
pub enum MenuTemplateNode {
    Text(String),
    Element {
        kind: ElementKind,
        children: Vec<DefaultKey>,
    },
}

#[derive(Debug)]
pub enum MenuElement {
    Item { text: String },
    Root { children: Vec<ElementId> },
}

pub struct Menu {
    vdom: VirtualDom,
    nodes: SlotMap<DefaultKey, MenuTemplateNode>,
    templates: HashMap<String, MenuTemplate>,
    elements: HashMap<ElementId, MenuElement>,
    tray_icon: Option<TrayIcon>,
    icon: Icon,
}

impl Menu {
    pub fn new(icon: Icon, app: Component) -> Self {
        let mut elements = HashMap::new();
        elements.insert(
            ElementId(0),
            MenuElement::Root {
                children: Vec::new(),
            },
        );

        Self {
            vdom: VirtualDom::new(app),
            nodes: SlotMap::new(),
            templates: HashMap::new(),
            elements,
            tray_icon: None,
            icon,
        }
    }

    pub fn rebuild(&mut self) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);

        for template in &mutations.templates {
            let mut stack: Vec<_> = template.roots.iter().map(|node| (None, *node)).collect();

            let mut roots = Vec::new();
            while let Some((parent, node)) = stack.pop() {
                match node {
                    TemplateNode::Text { text } => {
                        let key = self.nodes.insert(MenuTemplateNode::Text(text.to_owned()));
                        if let Some(parent) = parent {
                            if let MenuTemplateNode::Element { kind: _, children } =
                                &mut self.nodes[parent]
                            {
                                children.push(key);
                            }
                        } else {
                            roots.push(key);
                        }
                    }
                    TemplateNode::Element {
                        tag: _,
                        namespace: _,
                        attrs: _,
                        children,
                    } => {
                        let key = self.nodes.insert(MenuTemplateNode::Element {
                            kind: ElementKind::Item,
                            children: Vec::new(),
                        });
                        stack.extend(children.iter().map(|node| (Some(key), *node)));

                        if let Some(parent) = parent {
                            if let MenuTemplateNode::Element { kind: _, children } =
                                &mut self.nodes[parent]
                            {
                                children.push(key);
                            }
                        } else {
                            roots.push(key);
                        }
                    }
                    _ => todo!(),
                }
            }

            let template = MenuTemplate {
                name: template.name.to_owned(),
                roots,
            };
            self.templates.insert(template.name.clone(), template);
        }

        let mut stack = Vec::new();
        for mutation in mutations.edits {
            match mutation {
                Mutation::LoadTemplate { name, index, id } => {
                    let template = &self.templates[name];
                    let root_key = template.roots[index];
                    let root = &self.nodes[root_key];
                    match root {
                        MenuTemplateNode::Text(_text) => todo!(),
                        MenuTemplateNode::Element { kind: _, children } => {
                            let mut text = String::new();

                            for child_key in children {
                                let child = &self.nodes[*child_key];
                                match child {
                                    MenuTemplateNode::Text(s) => text.push_str(s),
                                    MenuTemplateNode::Element {
                                        kind: _,
                                        children: _,
                                    } => todo!(),
                                }
                            }

                            self.elements.insert(id, MenuElement::Item { text });
                            stack.push(id);
                        }
                    }
                }
                Mutation::AppendChildren { id, m } => {
                    for _ in 0..m {
                        let child_id = stack.pop().unwrap();
                        match self.elements.get_mut(&id).unwrap() {
                            MenuElement::Item { text: _ } => todo!(),
                            MenuElement::Root { children } => {
                                children.push(child_id);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let menu = muda::Menu::new();
        let root = &self.elements[&ElementId(0)];
        match root {
            MenuElement::Root { children } => {
                for child_id in children {
                    let child = &self.elements[child_id];
                    match child {
                        MenuElement::Item { text } => {
                            let item = MenuItemBuilder::new().text(text).build();
                            menu.append(&item).unwrap();
                        }
                        MenuElement::Root { children: _ } => todo!(),
                    }
                }
            }
            _ => todo!(),
        }

        self.tray_icon = Some(
            TrayIconBuilder::new()
                .with_tooltip("system-tray - tray icon library!")
                .with_menu(Box::new(menu))
                .with_icon(self.icon.clone())
                .build()
                .unwrap(),
        );
    }
}

impl fmt::Debug for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Menu")
            .field("nodes", &self.nodes)
            .field("templates", &self.templates)
            .field("elements", &self.elements)
            .finish()
    }
}

pub mod dioxus_elements {
    use dioxus::html::AttributeDiscription;

    #[allow(non_camel_case_types)]
    pub struct item;

    impl item {
        pub const TAG_NAME: &'static str = "item";
        pub const NAME_SPACE: Option<&'static str> = None;

        #[allow(non_upper_case_globals)]
        pub const accelerator: AttributeDiscription = ("accelerator", None, false);
    }
}

pub mod prelude {
    pub use crate::dioxus_elements::{self, *};

    pub use dioxus::prelude::{rsx, Element, Scope};
}
