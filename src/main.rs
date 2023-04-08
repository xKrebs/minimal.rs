//! # Minimal
//!
//! `Minimal` is a collection of utilities to get Element and HtmlElement
//!  more convenient and easier.

/// Adds one to the number given.
// --snip--

use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, Element, Document, Window, NodeList};

/// Create a Window or a Document.
///
/// # Examples
///
/// ```
/// let window = Minimal::window();
/// let document = Minimal::document();
///
/// assert_eq!(<Window>, window);
/// assert_eq!(<Document>, document);
/// ```
pub struct Minimal{}

impl Minimal{
    pub fn window() -> Window{
        web_sys::window().expect("no window found")
    }
    pub fn document() -> Document{
        Minimal::window().document_page()
    }
}

/// Some function for a Window element.
///
/// # Examples
///
/// ```
/// let window = Minimal::window();
/// let document = window.document_page();
///
/// assert_eq!(<Document>, document);
/// ```

//TRAIT
pub trait MinimalWindow{
    fn document_page(&self) -> Document;
    fn document_element_el(&self) -> Element;
    fn document_element_html(&self) -> HtmlElement;
}

/// Some function for a Document element.
///
/// # Examples
///
/// ```
/// let window = Minimal::window();
/// let document = window.document_page();
/// let h1_el = document.query_selector_el("h1");
/// let h1_html = document.query_selector_html("h1");
/// assert_eq!(<Element>, h1_el);
/// assert_eq!(<HtmlElement>, h1_html);
/// ```

pub trait MinimalDocument{
    fn get_element_by_id_el<'a>(&self, s: &'a str) -> Element;
    fn get_element_by_id_html<'a>(&self, s: &'a str) -> HtmlElement;
    fn query_selector_list<'a>(&self, s: &'a str) -> NodeList;
    fn query_selector_el<'a>(&self, s: &'a str) -> Element;
    fn query_selector_html<'a>(&self, s: &'a str) -> HtmlElement;
    fn document_element_el(&self) -> Element;
    fn document_element_html(&self) -> HtmlElement;
}

/// Some function for a Element element.
///
/// # Examples
///
/// ```
/// let document = Minimal::document();
/// let h1_el = document.query_selector_el("h1");
/// assert_eq!(<Element>, h1_el);
/// assert_eq!(<HtmlElement>, h1_el.to_html());
/// ```

pub trait MinimalElement{
    fn to_html(&self) -> HtmlElement;
}

/// Some function for a HtmlElement element.
///
/// # Examples
///
/// ```
/// let document = Minimal::document();
/// let h1_html = document.query_selector_html("h1");
/// if h1_html.has_class("test"){
/// 	h1_html.remove_class("test");
/// }
/// ```

pub trait MinimalHtml {
    fn has_class<'a>(&self, s: &'a str) -> bool;
    fn add_class<'a>(&self, s: &'a str);
    fn remove_class<'a>(&self, s: &'a str);
    fn closest_html<'a>(&self, s: &'a str) -> HtmlElement;
    fn query_selector_html<'a>(&self, s: &'a str) -> HtmlElement;
    fn query_selector_list<'a>(&self, s: &'a str) -> NodeList;
    fn parent_element_html(&self) -> HtmlElement;
}

/// Some function for a NodeList element.
///
/// # Examples
///
/// ```
/// let document = Minimal::document();
/// let h1_list = document.query_selector_list("h1");
/// assert_eq!(<NodeList>, h1_list);
/// let element = h1_list.get_html(5);
/// assert_eq!(<HtmlElement>, element);
/// ```
/// 
pub trait MinimalList {
    fn get_el(&self, n: u32) -> Element;
    fn get_html(&self, n: u32) -> HtmlElement;
    fn add_list_class<'a>(&self, s: &'a str);
    fn remove_list_class<'a>(&self, s: &'a str);
}

//IMPL TRAIT

impl MinimalWindow for Window{
    fn document_page(&self) -> Document {
        self.document().expect("no document found")
    }
    fn document_element_el(&self) -> Element {
        self.document_page().document_element().expect("no document_element found")
    }
    fn document_element_html(&self) -> HtmlElement {
        self.document_page().document_element().expect("no document_element found").dyn_into::<HtmlElement>().expect("no document_element to convert")
    }
}

impl MinimalDocument for Document {
    fn get_element_by_id_el<'a>(&self, s: &'a str) -> Element {
        self.get_element_by_id(s).expect(&("no element with id: ".to_owned() +s))
    }
    fn get_element_by_id_html<'a>(&self, s: &'a str) -> HtmlElement {
        self.get_element_by_id(s).expect(&("no element with id: ".to_owned() +s)).dyn_into::<HtmlElement>().expect("It's not possible convert due no element")
    }
    fn query_selector_html<'a>(&self, s: &'a str) -> HtmlElement {
        let value = match self.query_selector(s).expect(&("No element with this selector found: ".to_owned()+s)){
            Some(e) => Some(e.dyn_into::<HtmlElement>().expect("The HTML convert failed!")),
            None => None,
        };
        value.expect("There's no an element")
    }
    fn query_selector_el<'a>(&self, s: &'a str) -> Element {
        let value = match self.query_selector(s).expect(&("No element with this selector found: ".to_owned()+s)){
            Some(e) => Some(e),
            None => None,
        };
        value.expect("no element found")
    }
    fn query_selector_list<'a>(&self, s: &'a str) -> NodeList {
        self.query_selector_all(s).expect("no one element found")
    }
    fn document_element_el(&self) -> Element {
        self.document_element().expect("no document_element found")
    }
    fn document_element_html(&self) -> HtmlElement {
        self.document_element().expect("no document_element found").dyn_into::<HtmlElement>().expect("no document_element to convert")
    }
}

impl MinimalElement for Element{
    fn to_html(&self) -> HtmlElement {
        self.clone().dyn_into::<HtmlElement>().expect("No element found")
    }
}

impl MinimalHtml for HtmlElement{
    fn has_class<'a>(&self, s: &'a str) -> bool{
        if self.class_name().contains(&("".to_owned() + s)){
            true
        }else{
            false
        }
    }
    fn add_class<'a>(&self, s: &'a str) {
        self.set_class_name(&(self.class_name() + &" ".to_owned() + s));
    }
    fn remove_class<'a>(&self, s: &'a str) {
        self.set_class_name(&(self.class_name().replace(&(" ".to_owned() + s), "") ));
    }
    fn query_selector_list<'a>(&self, s: &'a str) -> NodeList {
        self.query_selector_all(s).expect("no one element found")
    }
    fn query_selector_html<'a>(&self, s: &'a str) -> HtmlElement {
        let value = match self.query_selector(s).expect(&("No element with this selector found: ".to_owned()+s)){
            Some(e) => Some(e.dyn_into::<HtmlElement>().expect("The HTML convert failed!")),
            None => None,
        };
        value.expect("There's no an element")
    }
    fn closest_html<'a>(&self, s: &'a str) -> HtmlElement {
        self.closest(s).expect(&("No element with this selector found: ".to_owned() + s)).expect("there's no an element to get.").dyn_into::<HtmlElement>().expect("failed to convert")
    }
    fn parent_element_html(&self) -> HtmlElement {
        self.parent_element().expect("failed").dyn_into::<HtmlElement>().expect("failed")
    }
}

impl MinimalList for NodeList{
    fn get_el(&self, n: u32) -> Element {
        self.item(n).expect(&("no element in position ".to_owned() + &n.to_string())).dyn_into::<Element>().expect("no element")
    }
    fn get_html(&self, n: u32) -> HtmlElement {
        self.item(n).expect(&("no element in position ".to_owned() + &n.to_string())).dyn_into::<HtmlElement>().expect("no element")
    }
    fn add_list_class<'a>(&self, s: &'a str){
        for i in 0..self.length(){
            if !self.get_html(i).has_class(s){
                self.get_html(i).add_class(s);
            }
        }
    }
    fn remove_list_class<'a>(&self, s: &'a str){
        for i in 0..self.length(){
            if self.get_html(i).has_class(s){
                self.get_html(i).remove_class(s);
            }
        }
    }
}