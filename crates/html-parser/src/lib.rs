#![deny(warnings)]

use markup5ever_rcdom::{NodeData, RcDom, SerializableHandle, Handle};
use markup5ever::serialize::TraversalScope;
use html5ever::serialize::SerializeOpts;
use html5ever::{parse_fragment, serialize, QualName};
use xml5ever::tendril::TendrilSink;
use markup5ever::{local_name, namespace_url, ns};
// use std::rc::Rc;
// use std::cell::Cell;

use sauron_core::{
    html::{attributes::*, lookup, *},
    vdom::AttributeValue,
    vdom::Node,
    vdom::Value,
};

/// all the possible error when parsing html string
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid tag: {0}")]
    InvalidTag(String),
    #[error("Unable to find body tag in generated html document")]
    NoBodyInParsedHtml,
}

/// parse escaped html strings like "Hello&#x20;world&#x21;" into "Hello world!" and then into a node tree
pub fn raw_html<MSG>(html: &str) -> Node<MSG> {
    // decode html entitiesd back since it will be safely converted into text
    let html = html_escape::decode_html_entities(html);
    parse_html(&html)
        .expect("must be ok")
        .expect("must have a node")
}

/// parse none-escaped html strings as "Hello world!" into a node tree (see also raw_html(...))
pub fn parse_html<MSG>(html: &str) -> Result<Option<Node<MSG>>, ParseError> {
    let dom: RcDom = parse_fragment(RcDom::default(), Default::default(),
    QualName::new(None, ns!(html), local_name!("div")),
    vec![],
).one(html);
    process_handle(&dom.document)

    // let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(html);
    // if let Some(body) = find_body(&dom.document) {
    //     let new_document = Rc::new(markup5ever_rcdom::Node {
    //         data: NodeData::Document,
    //         parent: Cell::new(None),
    //         children: body.children.clone(),
    //     });
    //     process_handle(&new_document)
    // } else {
    //     Err(ParseError::NoBodyInParsedHtml)
    // }
}

// Recursively find the <body> element
// fn find_body(handle: &Handle) -> Option<Handle> {
//     match &handle.data {
//         NodeData::Element { name, .. } if name.local.as_ref() == "body" => Some(handle.clone()),
//         _ => {
//             for child in handle.children.borrow().iter() {
//                 if let Some(body) = find_body(child) {
//                     return Some(body);
//                 }
//             }
//             None
//         }
//     }
// }

//TODO: This is not dealing with html symbols such as
//   `&#9650;`
//   `&#9660;`
// similar to 	fn build_tree(&self, options: &RenderOptions, status: &mut RenderStatus, result: &mut Vec<char>) {
fn process_handle<MSG>(node: &Handle) -> Result<Option<Node<MSG>>, ParseError> {
    let children: Vec<Node<MSG>> = node
        .children
        .borrow()
        .iter()
        .filter_map(|child| process_handle(child).ok().flatten())
        .collect();

    match &node.data {
        NodeData::Document => {
            let child_nodes_len = children.len();
            match child_nodes_len {
                0 => Ok(Some(node_list([]))),
                1 => Ok(Some(children.into_iter().next().unwrap())),
                _ => Ok(Some(node_list(children))),
            }
        }
        NodeData::Text { contents } => {
            let content = contents.borrow().to_string();
            Ok(Some(text(content)))
        }
        NodeData::Element { name, attrs, .. } => {
            let tag_name = name.local.to_string();
            println!("tag_name: {}", tag_name);
            
            if tag_name == "pre".to_string() {
                let mut buffer: Vec::<u8> = vec![];
                let document: SerializableHandle = node.clone().into();
                let serialize_opts: SerializeOpts = SerializeOpts {
                    scripting_enabled: false,
                    traversal_scope: TraversalScope::ChildrenOnly(None),
                    create_missing_parent: false,
                };
            
                serialize(&mut buffer, &document, serialize_opts).expect("serialization failed");
                let writer_string = String::from_utf8(buffer).expect("Could not write buffer as string");
                println!("--- {} ---", writer_string);
                let content: String = format!("<pre>{}</pre>", writer_string);
                Ok(Some(text(content)))
            }
            else {
            if let Some(html_tag) = lookup::match_tag(&tag_name) {
                let is_self_closing = HTML_SC_TAGS.contains(&html_tag);
                let attributes: Vec<Attribute<MSG>> = attrs
                    .borrow()
                    .iter()
                    .filter_map(|attr| {
                        let key = attr.name.local.to_string();
                        if let Some(attr_key) = lookup::match_attribute(&key) {
                            let value = if key == "style" {
                                let raw_tokens: Vec<String> = attr.value.split(';').map(|s| s.trim().to_string()).collect();
                                let tokens: Vec<Style> = raw_tokens.iter().filter_map(|m| {
                                    let t: Vec<String> = m.split(':').map(|s| s.trim().to_string()).collect();
                                    if t.len() == 2 {
                                        Some(Style::new(t[0].clone(), t[1].clone()))
                                    } else {
                                        None
                                    }
                                }).collect();
                                if !tokens.is_empty() {
                                    AttributeValue::Style(tokens)
                                } else {
                                    AttributeValue::Empty
                                }
                            } else {
                                AttributeValue::Simple(Value::from(attr.value.to_string()))
                            };
                            Some(Attribute::new(None, attr_key, value))
                        } else {
                            log::warn!("Not a standard html attribute: {}", key);
                            None
                        }
                    })
                    .collect();

                Ok(Some(html_element(
                    None, html_tag, attributes, children, is_self_closing,
                )))
            } else {
                log::error!("Invalid tag: {}", tag_name);
                Err(ParseError::InvalidTag(tag_name))
            }
        }}
        _ => Ok(None),
    }
}
