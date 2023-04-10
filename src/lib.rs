//! # minimal
//! `minimal` is a collection of utilities to get Element and HtmlElement
//!  more convenient and easier.

pub use utils::document;
pub use utils::window;
pub use utils::MinimalDocument;
pub use utils::MinimalElement;
pub use utils::MinimalHtml;
pub use utils::MinimalList;
pub use utils::MinimalWindow;
pub mod utils {
    use wasm_bindgen::prelude::*;
    use web_sys::{Location, Attr, Document, Element, HtmlElement, HtmlSlotElement, NodeList, Window};
    /// Create a Window.
    ///
    /// # Examples
    ///
    /// ```
    /// let window = Minimal::window();
    ///
    /// assert_eq!(<Window>, window);
    /// ```
    ///

    pub fn window() -> Window {
        web_sys::window().expect("no window found")
    }

    /// Create a Document.
    ///
    /// # Examples
    ///
    /// ```
    /// let document = Minimal::document();
    ///
    /// assert_eq!(<Document>, document);
    ///

    pub fn document() -> Document {
        web_sys::window().expect("no window found").document_page()
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
    pub trait MinimalWindow {
        fn document_page(&self) -> Document;
        fn document_element_el(&self) -> Element;
        fn document_element_html(&self) -> HtmlElement;
        fn get_name(&self) -> String;
        fn get_scroll_x(&self) -> f64;
        fn get_scroll_y(&self) -> f64;
        fn get_page_x_offset(&self) -> f64;
        fn get_page_y_offset(&self) -> f64;
        fn get_screen_x(&self) -> JsValue;
        fn get_screen_y(&self) -> JsValue;
        fn get_outer_width(&self) -> JsValue;
        fn get_outer_height(&self) -> JsValue;
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

    pub trait MinimalDocument {
        fn get_element_by_id_el<'a>(&self, s: &'a str) -> Element;
        fn get_element_by_id_html<'a>(&self, s: &'a str) -> HtmlElement;
        fn query_selector_list<'a>(&self, s: &'a str) -> NodeList;
        fn query_selector_el<'a>(&self, s: &'a str) -> Element;
        fn query_selector_html<'a>(&self, s: &'a str) -> HtmlElement;
        fn document_element_el(&self) -> Element;
        fn document_element_html(&self) -> HtmlElement;
        fn get_url(&self) -> String;
        fn get_location(&self) -> Location;
        fn get_hash(&self) -> String;
        fn get_host(&self) -> String;
        fn get_hostname(&self) -> String;
        fn get_href(&self) -> String;
        fn set_new_body(&self, e: HtmlElement);
        fn get_default_view(&self) -> Window;
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

    pub trait MinimalElement {
        fn to_html(&self) -> HtmlElement;
        fn has_class<'a>(&self, s: &'a str) -> bool;
        fn add_class<'a>(&self, s: &'a str);
        fn remove_class<'a>(&self, s: &'a str);
        fn toggle_class<'a>(&self, s: &'a str);
        fn get_namespace_uri(&self) -> String;
        fn get_pref(&self) -> String;
        fn get_assigned_slot(&self) -> HtmlSlotElement;
        fn prev_element_sibling_el(&self) -> Element;
        fn next_element_sibling_el(&self) -> Element;
        fn first_element_child_el(&self) -> Element;
        fn last_element_child_el(&self) -> Element;
        fn prev_element_sibling_html(&self) -> HtmlElement;
        fn next_element_sibling_html(&self) -> HtmlElement;
        fn first_element_child_html(&self) -> HtmlElement;
        fn last_element_child_html(&self) -> HtmlElement;
        fn get_attr<'a>(&self, s: &'a str) -> String;
        fn get_attr_node<'a>(&self, s: &'a str) -> Attr;
        fn get_attr_ns<'a>(&self, ns: &'a str, ln: &'a str) -> String;
        fn insert_adj_element<'a, 's>(&self, s: &'s str, e: &Element) -> Element;
        fn toggle_attr<'a>(&self, s: &'a str) -> bool;
        fn remove_attr<'a>(&self, s: &'a str);
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
        fn to_el(&self) -> Element;
        fn has_class<'a>(&self, s: &'a str) -> bool;
        fn add_class<'a>(&self, s: &'a str);
        fn remove_class<'a>(&self, s: &'a str);
        fn toggle_class<'a>(&self, s: &'a str);
        fn closest_el<'a>(&self, s: &'a str) -> Element;
        fn closest_html<'a>(&self, s: &'a str) -> HtmlElement;
        fn query_selector_html<'a>(&self, s: &'a str) -> HtmlElement;
        fn query_selector_list<'a>(&self, s: &'a str) -> NodeList;
        fn parent_element_html(&self) -> HtmlElement;
        fn offset_parent_el(&self) -> Element;
        fn offset_parent_html(&self) -> HtmlElement;
        fn prev_element_sibling_el(&self) -> Element;
        fn next_element_sibling_el(&self) -> Element;
        fn first_element_child_el(&self) -> Element;
        fn last_element_child_el(&self) -> Element;
        fn prev_element_sibling_html(&self) -> HtmlElement;
        fn next_element_sibling_html(&self) -> HtmlElement;
        fn first_element_child_html(&self) -> HtmlElement;
        fn last_element_child_html(&self) -> HtmlElement;
        fn get_attr<'a>(&self, s: &'a str) -> String;
        fn get_attr_node<'a>(&self, s: &'a str) -> Attr;
        fn get_attr_ns<'a>(&self, ns: &'a str, ln: &'a str) -> String;
        fn insert_adj_element<'a, 's>(&self, s: &'s str, e: &Element) -> Element;
        fn toggle_attr<'a>(&self, s: &'a str) -> bool;
        fn remove_attr<'a>(&self, s: &'a str);
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

    impl MinimalWindow for Window {
        fn document_page(&self) -> Document {
            self.document().expect("no document found")
        }
        fn document_element_el(&self) -> Element {
            self.document_page()
                .document_element()
                .expect("no document element found")
        }
        fn document_element_html(&self) -> HtmlElement {
            self.document_page()
                .document_element()
                .expect("no document element found")
                .dyn_into::<HtmlElement>()
                .expect("no document element to convert")
        }
        fn get_name(&self) -> String {
            self.name().expect("No name found")
        }
        fn get_scroll_x(&self) -> f64 {
            self.scroll_x().expect("No scroll x found")
        }
        fn get_scroll_y(&self) -> f64 {
            self.scroll_y().expect("No scroll y found")
        }
        fn get_page_x_offset(&self) -> f64 {
            self.page_x_offset().expect("No page x offset found")
        }
        fn get_page_y_offset(&self) -> f64 {
            self.page_y_offset().expect("No page y offset found")
        }
        fn get_screen_x(&self) -> JsValue {
            self.screen_x().expect("No screen x found")
        }
        fn get_screen_y(&self) -> JsValue {
            self.screen_y().expect("No screen y found")
        }
        fn get_outer_height(&self) -> JsValue {
            self.outer_height().expect("No outer height found")
        }
        fn get_outer_width(&self) -> JsValue {
            self.outer_width().expect("No outer width found")
        }
    }

    impl MinimalDocument for Document {
        fn get_element_by_id_el<'a>(&self, s: &'a str) -> Element {
            self.get_element_by_id(s)
                .expect(&("no element with id: ".to_owned() + s))
        }
        fn get_element_by_id_html<'a>(&self, s: &'a str) -> HtmlElement {
            self.get_element_by_id(s)
                .expect(&("no element with id: ".to_owned() + s))
                .dyn_into::<HtmlElement>()
                .expect("It's not possible convert no element")
        }
        fn query_selector_html<'a>(&self, s: &'a str) -> HtmlElement {
            let value = match self
                .query_selector(s)
                .expect(&("No element with this selector found: ".to_owned() + s))
            {
                Some(e) => Some(
                    e.dyn_into::<HtmlElement>()
                        .expect("The HTML convert failed!"),
                ),
                None => None,
            };
            value.expect("There's no an element")
        }
        fn query_selector_el<'a>(&self, s: &'a str) -> Element {
            let value = match self
                .query_selector(s)
                .expect(&("No element with this selector found: ".to_owned() + s))
            {
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
            self.document_element()
                .expect("no document_element found")
                .dyn_into::<HtmlElement>()
                .expect("no document_element to convert")
        }
        fn get_url(&self) -> String {
            self.url().expect("No url found")
        }
        fn get_location(&self) -> Location {
            self.location().expect("No location found")
        }
        fn get_hash(&self) -> String {
            self.get_location().hash().expect("No hash found")
        }
        fn get_host(&self) -> String {
            self.get_location().host().expect("No host found")
        }
        fn get_hostname(&self) -> String {
            self.get_location().hostname().expect("No hostname found")
        }
        fn get_href(&self) -> String {
            self.get_location().href().expect("No href found")
        }
        fn set_new_body(&self, e: HtmlElement) {
            self.set_body(Some(&e))
        }
        fn get_default_view(&self) -> Window {
            self.default_view().expect("There's not a window")
        }
    }

    impl MinimalElement for Element {
        fn to_html(&self) -> HtmlElement {
            self.clone()
                .dyn_into::<HtmlElement>()
                .expect("It's not possible get an HtmlElement of Null")
        }
        fn has_class<'a>(&self, s: &'a str) -> bool {
            if self.class_name().contains(&("".to_owned() + s)) {
                true
            } else {
                false
            }
        }
        fn toggle_class<'a>(&self, s: &'a str) {
            if self.has_class(s) {
                self.remove_class(s);
            } else {
                self.add_class(s);
            }
        }
        fn add_class<'a>(&self, s: &'a str) {
            self.set_class_name(&(self.class_name() + &" ".to_owned() + s));
        }
        fn remove_class<'a>(&self, s: &'a str) {
            self.set_class_name(&(self.class_name().replace(&(" ".to_owned() + s), "")));
        }
        fn get_namespace_uri(&self) -> String {
            self.namespace_uri().expect("No namespace found")
        }
        fn get_assigned_slot(&self) -> HtmlSlotElement {
            self.assigned_slot().expect("No assigned slot found")
        }
        fn first_element_child_el(&self) -> Element {
            self.first_element_child().expect("No first child found")
        }
        fn last_element_child_el(&self) -> Element {
            self.last_element_child().expect("No last child found")
        }
        fn next_element_sibling_el(&self) -> Element {
            self.next_element_sibling().expect("No next sibling found")
        }
        fn prev_element_sibling_el(&self) -> Element {
            self.previous_element_sibling()
                .expect("No prev sibling found")
        }
        fn first_element_child_html(&self) -> HtmlElement {
            self.first_element_child()
                .expect("No first child found")
                .to_html()
        }
        fn last_element_child_html(&self) -> HtmlElement {
            self.last_element_child()
                .expect("No last child found")
                .to_html()
        }
        fn next_element_sibling_html(&self) -> HtmlElement {
            self.next_element_sibling()
                .expect("No next sibling found")
                .to_html()
        }
        fn prev_element_sibling_html(&self) -> HtmlElement {
            self.previous_element_sibling()
                .expect("No prev sibling found")
                .to_html()
        }
        fn get_attr_node<'a>(&self, s: &'a str) -> Attr {
            self.get_attribute_node(s).expect("No attribute node found")
        }
        fn get_attr<'a>(&self, s: &'a str) -> String {
            self.get_attribute(s).expect("No attribute found")
        }
        fn get_attr_ns<'a>(&self, ns: &'a str, ln: &'a str) -> String {
            self.get_attribute_ns(Some(ns), ln).expect("There's been an error")
        }
        fn toggle_attr<'a>(&self, s: &'a str) -> bool {
            self.toggle_attribute(s)
                .expect("It's not possible toggle this attribute")
        }
        fn get_pref(&self) -> String {
            self.prefix().expect("No prefix found")
        }
        fn insert_adj_element<'s, 'a>(&self, s: &'s str, e: &Element) -> Element {
            self.insert_adjacent_element(s, e)
                .expect("It has not been possible insert element")
                .expect("There's been a problem")
        }
        fn remove_attr<'a>(&self, s: &'a str) {
            self.remove_attribute(s).expect("It's no possible remove attribute")
        }
    }

    impl MinimalHtml for HtmlElement {
        fn to_el(&self) -> Element {
            self.clone()
                .dyn_into::<Element>()
                .expect("It's not possible get an Element of Null")
        }
        fn has_class<'a>(&self, s: &'a str) -> bool {
            if self.class_name().contains(&("".to_owned() + s)) {
                true
            } else {
                false
            }
        }
        fn toggle_class<'a>(&self, s: &'a str) {
            if self.has_class(s) {
                self.remove_class(s);
            } else {
                self.add_class(s);
            }
        }
        fn add_class<'a>(&self, s: &'a str) {
            self.set_class_name(&(self.class_name() + &" ".to_owned() + s));
        }
        fn remove_class<'a>(&self, s: &'a str) {
            self.set_class_name(&(self.class_name().replace(&(" ".to_owned() + s), "")));
        }
        fn query_selector_list<'a>(&self, s: &'a str) -> NodeList {
            self.query_selector_all(s).expect("no one element found")
        }
        fn query_selector_html<'a>(&self, s: &'a str) -> HtmlElement {
            let value = match self
                .query_selector(s)
                .expect(&("No element with this selector found: ".to_owned() + s))
            {
                Some(e) => Some(
                    e.to_html(),
                ),
                None => None,
            };
            value.expect("There's no an element")
        }
        fn closest_html<'a>(&self, s: &'a str) -> HtmlElement {
            self.closest(s)
                .expect(&("No element with this selector found: ".to_owned() + s))
                .expect("there's no an element to get.")
                .to_html()
        }
        fn closest_el<'a>(&self, s: &'a str) -> Element {
            self.closest(s)
                .expect(&("No element with this selector found: ".to_owned() + s))
                .expect("there's no an element to get.")
        }
        fn parent_element_html(&self) -> HtmlElement {
            self.parent_element()
                .expect("No parent element found")
                .to_html()
        }
        fn first_element_child_el(&self) -> Element {
            self.first_element_child().expect("No first child found")
        }
        fn last_element_child_el(&self) -> Element {
            self.last_element_child().expect("No last child found")
        }
        fn next_element_sibling_el(&self) -> Element {
            self.next_element_sibling().expect("No next sibling found")
        }
        fn prev_element_sibling_el(&self) -> Element {
            self.previous_element_sibling()
                .expect("No prev sibling found")
        }
        fn first_element_child_html(&self) -> HtmlElement {
            self.first_element_child()
                .expect("No first child found")
                .to_html()
        }
        fn last_element_child_html(&self) -> HtmlElement {
            self.last_element_child()
                .expect("No last child found")
                .to_html()
        }
        fn next_element_sibling_html(&self) -> HtmlElement {
            self.next_element_sibling()
                .expect("No next sibling found")
                .to_html()
        }
        fn prev_element_sibling_html(&self) -> HtmlElement {
            self.previous_element_sibling()
                .expect("No prev sibling found")
                .to_html()
        }
        fn offset_parent_el(&self) -> Element {
            self.offset_parent().expect("No offset parent found")
        }
        fn offset_parent_html(&self) -> HtmlElement {
            self.offset_parent()
                .expect("No offset parent found")
                .to_html()
        }
        fn get_attr<'a>(&self, s: &'a str) -> String {
            self.get_attribute(s).expect("No attribute found")
        }
        fn toggle_attr<'a>(&self, s: &'a str) -> bool {
            self.toggle_attribute(s)
                .expect("It's not possible toggle this attribute")
        }
        fn insert_adj_element<'s, 'a>(&self, s: &'s str, e: &Element) -> Element {
            self.insert_adjacent_element(s, e)
                .expect("It has not been possible insert element")
                .expect("There's been a problem")
        }
        fn get_attr_node<'a>(&self, s: &'a str) -> Attr {
            self.get_attribute_node(s).expect("No attribute node found")
        }
        fn get_attr_ns<'a>(&self, ns: &'a str, ln: &'a str) -> String {
            self.get_attribute_ns(Some(ns), ln).expect("There's been an error")
        }
        fn remove_attr<'a>(&self, s: &'a str) {
            self.remove_attribute(s).expect("It's no possible remove attribute")
        }
    }

    impl MinimalList for NodeList {
        fn get_el(&self, n: u32) -> Element {
            self.item(n)
                .expect(&("no element in position ".to_owned() + &n.to_string()))
                .dyn_into::<Element>()
                .expect("It's not possible get an Element of Null")
        }
        fn get_html(&self, n: u32) -> HtmlElement {
            self.item(n)
                .expect(&("no element in position ".to_owned() + &n.to_string()))
                .dyn_into::<HtmlElement>()
                .expect("It's not possible get an HtmlElement of Null")
        }
        fn add_list_class<'a>(&self, s: &'a str) {
            for i in 0..self.length() {
                if !self.get_html(i).has_class(s) {
                    self.get_html(i).add_class(s);
                }
            }
        }
        fn remove_list_class<'a>(&self, s: &'a str) {
            for i in 0..self.length() {
                if self.get_html(i).has_class(s) {
                    self.get_html(i).remove_class(s);
                }
            }
        }
    }
}
