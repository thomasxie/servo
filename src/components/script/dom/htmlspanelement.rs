/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLSpanElementBinding;
use dom::document::AbstractDocument;
use dom::element::HTMLSpanElementTypeId;
use dom::htmlelement::HTMLElement;
use dom::node::{AbstractNode, Node, ScriptView};

pub struct HTMLSpanElement {
    htmlelement: HTMLElement
}

impl HTMLSpanElement {
    pub fn new_inherited(localName: ~str, document: AbstractDocument) -> HTMLSpanElement {
        HTMLSpanElement {
            htmlelement: HTMLElement::new_inherited(HTMLSpanElementTypeId, localName, document)
        }
    }

    pub fn new(localName: ~str, document: AbstractDocument) -> AbstractNode<ScriptView> {
        let element = HTMLSpanElement::new_inherited(localName, document);
        Node::reflect_node(@mut element, document, HTMLSpanElementBinding::Wrap)
    }
}
