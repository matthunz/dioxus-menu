use dioxus::core::{Component, ElementId, Mutation, TemplateNode, VirtualDom};
use slotmap::{DefaultKey, SlotMap};
use std::{collections::HashMap, fmt, mem};

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
}

pub struct Menu {
    vdom: VirtualDom,
    nodes: SlotMap<DefaultKey, MenuTemplateNode>,
    templates: HashMap<String, MenuTemplate>,
    elements: HashMap<ElementId, MenuElement>,
}

impl Menu {
    pub fn new(app: Component) -> Self {
        Self {
            vdom: VirtualDom::new(app),
            nodes: SlotMap::new(),
            templates: HashMap::new(),
            elements: HashMap::new(),
        }
    }

    pub fn rebuild(&mut self) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);

        for template in &mutations.templates {
            enum Op<'a> {
                PushChild(TemplateNode<'a>),
                PopParent,
            }
            let mut stack: Vec<_> = template
                .roots
                .iter()
                .rev()
                .copied()
                .map(Op::PushChild)
                .collect();
            let mut parents = Vec::new();
            let mut child_stack = Vec::new();
            while let Some(op) = stack.pop() {
                match op {
                    Op::PushChild(node) => match node {
                        TemplateNode::Text { text } => {
                            let key = self.nodes.insert(MenuTemplateNode::Text(text.to_owned()));
                            child_stack.push(key);
                        }
                        TemplateNode::Element {
                            tag: _,
                            namespace: _,
                            attrs: _,
                            children,
                        } => {
                            parents.push(ElementKind::Item);
                            stack.push(Op::PopParent);
                            stack.extend(children.iter().copied().map(Op::PushChild));
                        }
                        _ => todo!(),
                    },
                    Op::PopParent => {
                        let kind = parents.pop().unwrap();
                        let key = self.nodes.insert(MenuTemplateNode::Element {
                            kind,
                            children: mem::take(&mut child_stack),
                        });
                        child_stack.push(key);
                    }
                }
            }

            let template = MenuTemplate {
                name: template.name.to_owned(),
                roots: child_stack,
            };
            self.templates.insert(template.name.clone(), template);
        }

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
                        }
                    }
                }
                _ => {}
            }
        }
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
