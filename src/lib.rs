/*!

*svgdom* is an [SVG Full 1.1](https://www.w3.org/TR/SVG/) processing library,
which allows you to parse, manipulate, generate and write an SVG content.

## Deprecation

This library was an attempt to create a generic SVG DOM which can be used by various applications.
But it the end it turned out that it's easier to use
[roxmltree](https://github.com/RazrFalcon/roxmltree) + [svgtypes](https://github.com/RazrFalcon/svgtypes)
to extract only the data you need.

There are two main problems with `svgdom`:

1. You can't make a nice API with a Vec-based tree and you can't have a safe API
   with an Rc-tree.

   The current implementation uses so-called Rc-tree, which provides a nice API,
   but all the checks are done in the runtime, so you can get a panic quite easily.
   It's also hard/verbose to make immutable nodes. You essentially need two types of nodes:
   one for immutable and one for mutable "references".
   A Vec-based tree would not have such problems, but you can't implement the simplest
   operations with it, like copying an attribute from one node to another
   since you have to have a mutable and an immutable references for this.
   And Rust forbids this. So you need some sort of generational indexes and so on.
   This solution is complicated in its own way.
   Performance is also in question, since inserting/removing an object in the middle of a Vec is expensive.
2. The SVG parsing itself is pretty complex too. There are a lot of ways you can implement it.

   `svgdom` creates a custom Rc-tree where all the attributes are stored as owned data.
   This requires a lot of allocations (usually unnecessary).
   The parsing/preprocessing algorithm itself can be found in [docs/preprocessor.md](docs/preprocessor.md)
   The problem with it is that you can't tweak it. And in many cases, it produces results
   that you do not need or do not expect.
   `svgdom` was originally used by [svgcleaner](https://github.com/RazrFalcon/svgcleaner)
   and [resvg](https://github.com/RazrFalcon/resvg) and both of these projects are no longer using it.

## Purpose

*svgdom* is designed to simplify generic SVG processing and manipulations.
Unfortunately, an SVG is very complex format (PDF spec is 826 pages long),
with lots of features and implementing all of them will lead to an enormous library.

That's why *svgdom* supports only a static subset of an SVG. No scripts, external resources
and complex CSS styling.
Parser will convert as much as possible data to a simple doc->elements->attributes structure.

For example, the `fill` parameter of an element can be set: as an element's attribute,
as part of a `style` attribute, inside a `style` element as CSS2, inside an `ENTITY`,
using a JS code and probably with lots of other methods.

Not to mention, that the `fill` attribute supports 4 different types of data.

With `svgdom` you can just use `node.has_attribute(AttributeId::Fill)` and don't worry where this
attribute was defined in the original file.

Same goes for transforms, paths and other SVG types.

The main downside of this approach is that you can't save an original formatting and some data.

See the [preprocessor](https://github.com/RazrFalcon/svgdom/blob/master/docs/preprocessor.md)
doc for details.

## Benefits

- The element link(IRI, FuncIRI) is not just a text, but an actual link to another node.
- At any time you can check which elements linked to the specific element.
  See `Node`'s doc for details.
- Support for many SVG specific data types like paths, transforms, IRI's, styles, etc.
  Thanks to [svgtypes](https://github.com/RazrFalcon/svgtypes).
- A complete support of text nodes: XML escaping, `xml:space`.
- Fine-grained control over the SVG output.

## Limitations

- Only SVG elements and attributes will be parsed.
- Attribute values, CDATA with CSS, DOCTYPE, text data and whitespaces will not be preserved.
- UTF-8 only.
- Only most popular attributes are parsed, other stored as strings.
- No compressed SVG (.svgz). You should decompress it by yourself.
- CSS support is minimal.
- SVG 1.1 Full only (no 2.0 Draft, Basic, Tiny subsets).

## Differences between svgdom and SVG spec

- Library follows SVG spec in the data parsing, writing, but not in the tree structure.
- Everything is a `Node`. There are no separated `ElementNode`, `TextNode`, etc.
  You still have all the data, but not in the specific *struct's*.
  You can check the node type via `Node::node_type()`.

*/

#![doc(html_root_url = "https://docs.rs/svgdom/0.18.0")]

#![forbid(unsafe_code)]
#![warn(missing_docs)]


mod attribute;
mod document;
mod node;
mod tree;
mod element_type;
mod error;
mod name;
mod names;
mod parser;
mod writer;
mod attribute_type;
mod attribute_value;
mod attributes;


pub use crate::attribute::*;
pub use crate::attribute_type::AttributeType;
pub use crate::attribute_value::AttributeValue;
pub use crate::attributes::*;
pub use crate::document::Document;
pub use crate::element_type::ElementType;
pub use crate::error::*;
pub use crate::name::*;
pub use crate::names::*;
pub use crate::node::*;
pub use crate::tree::iterator::*;
pub use crate::writer::*;

pub use svgtypes::{
    Align,
    Angle,
    AngleUnit,
    AspectRatio,
    Color,
    FuzzyEq,
    FuzzyZero,
    Length,
    LengthList,
    LengthUnit,
    ListSeparator,
    NumberList,
    PaintFallback,
    Path,
    PathCommand,
    PathSegment,
    Points,
    Transform,
    ViewBox,
    WriteBuffer,
    WriteOptions as ValueWriteOptions,
};


/// Type alias for `QNameRef<ElementId>`.
pub type TagNameRef<'a> = QNameRef<'a, ElementId>;
/// Type alias for `QName<ElementId>`.
pub type TagName = QName<ElementId>;

/// Type alias for `QName<AttributeId>`.
pub type AttributeQName = QName<AttributeId>;
/// Type alias for `QNameRef<AttributeId>`.
pub type AttributeQNameRef<'a> = QNameRef<'a, AttributeId>;


/// List of supported node types.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NodeType {
    /// The root node of the `Document`.
    ///
    /// Constructed with `Document`. Unavailable to the user.
    Root,
    /// An element node.
    ///
    /// Only an element can have attributes, ID and tag name.
    Element,
    /// A comment node.
    Comment,
    /// A text node.
    Text,
}


/// Node's data.
pub struct NodeData {
    storage_key: Option<usize>,
    node_type: NodeType,
    tag_name: TagName,
    id: String,
    attributes: Attributes,
    linked_nodes: Vec<Node>,
    text: String,
}
