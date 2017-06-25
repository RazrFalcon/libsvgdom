// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use {
    Attribute,
    AttributeId,
    AttributeValue,
    Document,
    Name,
    Node,
    NodeType,
};

#[derive(Clone,Copy,PartialEq)]
enum XmlSpace {
    Default,
    Preserve,
}

trait StrTrim {
    fn remove_first(&mut self);
    fn remove_last(&mut self);
}

impl StrTrim for String {
    fn remove_first(&mut self) {
        debug_assert!(self.is_char_boundary(0));

        // There is no other way to modify a String in place...
        let mut bytes = unsafe { self.as_mut_vec() };
        bytes.remove(0);
    }

    fn remove_last(&mut self) {
        debug_assert!(self.len() > 0);

        let pos = self.len() - 1;

        debug_assert!(self.is_char_boundary(pos));

        // There is no other way to modify a String in place...
        let mut bytes = unsafe { self.as_mut_vec() };
        bytes.remove(pos);
    }
}

// Prepare text nodes according to the spec: https://www.w3.org/TR/SVG11/text.html#WhiteSpace
//
// This function handles:
// - 'xml:space' processing
// - tabs and newlines removing/replacing
// - spaces trimming
pub fn prepare_text(dom: &Document) {
    _prepare_text(&dom.root(), XmlSpace::Default);

    // Remove invisible 'xml:space' attributes created during text processing.
    for node in dom.descendants().filter(|n| n.node_type() == NodeType::Element) {
        node.attributes_mut().retain(|attr| attr.visible == true);
    }
}

fn _prepare_text(parent: &Node, parent_xmlspace: XmlSpace) {
    let mut xmlspace = parent_xmlspace;

    for node in parent.children().filter(|n| n.node_type() == NodeType::Element) {
        xmlspace = get_xmlspace(&node, xmlspace);

        if let Some(child) = node.first_child() {
            if child.node_type() == NodeType::Text {
                prepare_text_children(&node, xmlspace);
                continue;
            }
        }

        _prepare_text(&node, xmlspace);
    }
}

fn get_xmlspace(node: &Node, default: XmlSpace) -> XmlSpace {
    {
        let attrs = node.attributes();
        let v = attrs.get_value(AttributeId::XmlSpace);
        if let Some(&AttributeValue::String(ref s)) = v {
            if s == "preserve" {
                return XmlSpace::Preserve;
            } else {
                return XmlSpace::Default;
            }
        }
    }

    // 'xml:space' is not set - set it manually.
    set_xmlspace(node, default);

    default
}

fn set_xmlspace(node: &Node, xmlspace: XmlSpace) {
    let xmlspace_str = match xmlspace {
        XmlSpace::Default => "default",
        XmlSpace::Preserve => "preserve",
    };

    let attr = Attribute {
        name: Name::Id(AttributeId::XmlSpace),
        value: AttributeValue::String(xmlspace_str.to_owned()),
        visible: false,
    };

    node.set_attribute(attr);
}

fn prepare_text_children(parent: &Node, xmlspace: XmlSpace) {
    // Trim all descendant text nodes.
    for child in parent.descendants() {
        if child.node_type() == NodeType::Text {
            let child_xmlspace = get_xmlspace(&child.parent().unwrap(), xmlspace);
            let mut text = child.text_mut();
            trim_text(&mut text, child_xmlspace);
        }
    }

    // Collect all descendant text nodes.
    let nodes: Vec<Node> = parent.descendants()
                                 .filter(|n| n.node_type() == NodeType::Text)
                                 .collect();

    // 'trim_text' already collapsed all spaces into a single one,
    // so we have to check only for one leading or trailing space.

    if nodes.len() == 1 {
        // Process element with a single text node child.

        let node = &nodes[0];

        // Do nothing when xml:space=preserve.
        if xmlspace == XmlSpace::Default {
            let mut text = node.text_mut();

            match text.len() {
                0 => {} // An empty string. Do nothing.
                1 => {
                    // If string has only one character and it's a space - clear this string.
                    if text.as_bytes()[0] == b' ' {
                        text.clear();
                    }
                }
                _ => {
                    // 'text' has at least 2 bytes, so indexing is safe.
                    let c1 = text.as_bytes()[0];
                    let c2 = text.as_bytes()[text.len() - 1];

                    if c1 == b' ' {
                        text.remove_first();
                    }

                    if c2 == b' ' {
                        text.remove_last();
                    }
                }
            }
        }
    } else {
        // Process element with a lot text node children.

        // We manage all text nodes as a single text node
        // and trying to remove duplicated spaces across nodes.
        //
        // For example    '<text>Text <tspan> text </tspan> text</text>'
        // is the same is '<text>Text <tspan>text</tspan> text</text>'

        let mut i = 0;
        let len = nodes.len() - 1;
        while i < len {
            // Process pairs.
            let node1 = &nodes[i];
            let node2 = &nodes[i + 1];

            // Parent of the text node is always an element node and always exist,
            // so unwrap is safe.
            let xmlspace1 = get_xmlspace(&node1.parent().unwrap(), xmlspace);
            let xmlspace2 = get_xmlspace(&node2.parent().unwrap(), xmlspace);

            let mut text1 = node1.text_mut();
            let mut text2 = node2.text_mut();

            // 'text' + 'text'
            //  1  2     3  4
            let c1 = text1.as_bytes().first().cloned();
            let c2 = text1.as_bytes().last().cloned();
            let c3 = text2.as_bytes().first().cloned();
            let c4 = text2.as_bytes().last().cloned();

            // Remove space from the second text node if both nodes has bound spaces.
            // From: '<text>Text <tspan> text</tspan></text>'
            // To:   '<text>Text <tspan>text</tspan></text>'
            if xmlspace1 == XmlSpace::Default && xmlspace2 == XmlSpace::Default {
                if c2 == Some(b' ') && c2 == c3 {
                    text2.remove_first();
                }
            }

            let is_first = i == 0;
            let is_last  = i == len - 1;

            if is_first && c1 == Some(b' ') && xmlspace1 == XmlSpace::Default {
                // Remove leading space of the first text node.
                text1.remove_first();
            } else if    is_last && c4 == Some(b' ') && !text2.is_empty()
                      && xmlspace2 == XmlSpace::Default {
                // Remove trailing space of the last text node.
                // Also check that 'text2' is not empty already.
                text2.remove_last();
            }

            i += 1;
        }
    }
}

fn trim_text(text: &mut String, xmlspace: XmlSpace) {
    // In place map() alternative.
    fn replace_if<P>(data: &mut Vec<u8>, p: P, new: u8)
        where P: Fn(u8) -> bool
    {
        for c in data.iter_mut() {
            if p(*c) {
                *c = new;
            }
        }
    }

    // There is no other way to modify a String in place...
    let mut bytes = unsafe { text.as_mut_vec() };

    // Process whitespaces as described in: https://www.w3.org/TR/SVG11/text.html#WhiteSpace
    match xmlspace {
        XmlSpace::Default => {
            // 'First, it will remove all newline characters.'
            bytes.retain(|c| *c != b'\n' && *c != b'\r');

            // 'Then it will convert all tab characters into space characters.'
            replace_if(&mut bytes, |c| c == b'\t', b' ');

            // 'Then, it will strip off all leading and trailing space characters.'
            //
            // But we do not trim spaces here, because it depend on sibling nodes.

            // 'Then, all contiguous space characters will be consolidated.'
            if bytes.len() > 1 {
                let mut pos = 0;
                while pos < bytes.len() - 1 {
                    if bytes[pos] == b' ' && bytes[pos + 1] == b' ' {
                        bytes.remove(pos);
                    } else {
                        pos += 1;
                    }
                }
            }
        }
        XmlSpace::Preserve => {
            // 'It will convert all newline and tab characters into space characters.'

            // '\r\n' should be converted into a single space.
            if bytes.len() > 1 {
                let mut pos = 0;
                while pos < bytes.len() - 1 {
                    if bytes[pos] == b'\r' && bytes[pos + 1] == b'\n' {
                        bytes.remove(pos);
                        bytes[pos] = b' ';
                    }

                    pos += 1;
                }
            }

            replace_if(&mut bytes, |c| c == b'\t' || c == b'\n' || c == b'\r', b' ');
        }
    }
}