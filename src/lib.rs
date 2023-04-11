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
pub use utils::MinimalNode;
pub mod utils {
    use wasm_bindgen::prelude::*;
    use web_sys::{Location, Attr, Document, Element, HtmlElement, HtmlSlotElement, NodeList, Window, Node};
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
        /// Get document of window.
        fn document_page(&self) -> Document;
        /// Get document_element as Element.
        fn document_element_el(&self) -> Element;
        /// Get document_element as HtmlElement.
        fn document_element_html(&self) -> HtmlElement;
        /// Get name of window, same as name() function.
        fn get_name(&self) -> String;
        /// Get scroll_x of window, same as scroll_x() function.
        fn get_scroll_x(&self) -> f64;
        /// Get scroll_y of window, same as scroll_y() function.
        fn get_scroll_y(&self) -> f64;
        /// Get page_x_offset of window, same as page_x_offset() function.
        fn get_page_x_offset(&self) -> f64;
        /// Get page_y_offset of window, same as page_y_offset() function.
        fn get_page_y_offset(&self) -> f64;
        /// Get screen_x of window, same as screen_x() function.
        fn get_screen_x(&self) -> JsValue;
        /// Get screen_y of window, same as screen_y() function.
        fn get_screen_y(&self) -> JsValue;
        /// Get outer_width of window, same as outer_width() function.
        fn get_outer_width(&self) -> JsValue;
        /// Get outer_height of window, same as outer_height() function.
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
        /// Get element by id as Element.
        fn get_element_by_id_el<'a>(&self, value: &'a str) -> Element;
        /// Get element by id as HtmlElement.
        fn get_element_by_id_html<'a>(&self, value: &'a str) -> HtmlElement;
        /// Get a List of all elements by selector.
        fn query_selector_list<'a>(&self, value: &'a str) -> NodeList;
        /// Get element by selector as Element.
        fn query_selector_el<'a>(&self, value: &'a str) -> Element;
        /// Get element by selector as HtmlElement.
        fn query_selector_html<'a>(&self, value: &'a str) -> HtmlElement;
        /// Get document_element as Element.
        fn document_element_el(&self) -> Element;
        /// Get document_element as HtmlElement.
        fn document_element_html(&self) -> HtmlElement;
        /// Get url of document, same as url().
        fn get_url(&self) -> String;
        /// Get location of document, same as location().
        fn get_location(&self) -> Location;
        /// Get hash of document, same as hash().
        fn get_hash(&self) -> String;
        /// Get host of document, same as host().
        fn get_host(&self) -> String;
        /// Get hostname of document, same as hostname().
        fn get_hostname(&self) -> String;
        /// Get href of document, same as href().
        fn get_href(&self) -> String;
        /// Set body of document, same as set_body().
        fn set_new_body(&self, e: HtmlElement);
        /// Get default_view of document, same as default_view().
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
        /// Convert Element to HtmlElement.
        fn to_html(&self) -> HtmlElement;
        /// Match if element has a class.
        fn has_class<'a>(&self, value: &'a str) -> bool;
        /// Add a class to Element.
        fn add_class<'a>(&self, value: &'a str);
        /// Remove a class to Element.
        fn remove_class<'a>(&self, value: &'a str);
        /// Toggle a class to Element.
        fn toggle_class<'a>(&self, value: &'a str);
        /// Get namespace_uri, same as namespace_uri().
        fn get_namespace_uri(&self) -> String;
        /// Get prefix, same as prefix().
        fn get_pref(&self) -> String;
        /// Get assigned_slot, same as assigned_slot().
        fn get_assigned_slot(&self) -> HtmlSlotElement;
        /// Get a List of all elements by selector.
        fn query_selector_list<'a>(&self, value: &'a str) -> NodeList;
        /// Get element by selector as Element.
        fn query_selector_el<'a>(&self, value: &'a str) -> Element;
        /// Get element by selector as HtmlElement.
        fn query_selector_html<'a>(&self, value: &'a str) -> HtmlElement;
        /// Get parent_element as Element.
        fn parent_element_el(&self) -> Element;
        /// Get parent_element as HtmlElement.
        fn parent_element_html(&self) -> HtmlElement;
        /// Get first_child as Element.
        fn first_child_el(&self) -> Element;
        /// Get first_child as HtmlElement.
        fn first_child_html(&self) -> HtmlElement;
        /// Get last_child as Element.
        fn last_child_el(&self) -> Element;
        /// Get last_child as HtmlElement.
        fn last_child_html(&self) -> HtmlElement;
        /// Get previous_sibling as Element.
        fn prev_sibling_el(&self) -> Element;
        /// Get previous_sibling as HtmlElement.
        fn prev_sibling_html(&self) -> HtmlElement;
        /// Get previous_element_sibling as Element.
        fn prev_element_sibling_el(&self) -> Element;
        /// Get next_element_sibling as Element.
        fn next_element_sibling_el(&self) -> Element;
        /// Get first_element_child as Element.
        fn first_element_child_el(&self) -> Element;
        /// Get last_element_child as Element.
        fn last_element_child_el(&self) -> Element;
        /// Get previous_element_sibling as HtmlElement.
        fn prev_element_sibling_html(&self) -> HtmlElement;
        /// Get next_element_sibling as HtmlElement.
        fn next_element_sibling_html(&self) -> HtmlElement;
        /// Get first_element_child as HtmlElement.
        fn first_element_child_html(&self) -> HtmlElement;
        /// Get last_element_child as HtmlElement.
        fn last_element_child_html(&self) -> HtmlElement;
        /// Get closest element as Element.
        fn closest_el<'a>(&self, value: &'a str) -> Element;
        /// Get closest element as HtmlElement.
        fn closest_html<'a>(&self, value: &'a str) -> HtmlElement;
        /// Get attribute of Element.
        fn get_attr<'a>(&self, value: &'a str) -> String;
        /// Get attribute node of Element.
        fn get_attr_node<'a>(&self, value: &'a str) -> Attr;
        /// Get attribute ns of Element.
        fn get_attr_ns<'a>(&self, namespace: &'a str, localname: &'a str) -> String;
        /// Insert adjacent element to Element.
        fn insert_adj_el<'a, 's>(&self, where_: &'s str, element: Element) -> Element;
        /// Toggle attribute to Element.
        fn toggle_attr<'a>(&self, value: &'a str) -> bool;
        /// Remove attribute to Element.
        fn remove_attr<'a>(&self, value: &'a str);
        /// Match Element with selector, same as matches().
        fn has_match<'a>(&self, value: &'a str) -> bool;
        /// Get node value of Element.
        fn get_node_value(&self) -> String;
        /// Get text content of Element.
        fn get_text_content(&self) -> String;
        /// Append a Node to Element, same as append_child().
        fn app_child(&self, node: Node) -> Node;
    }

    /// Some function for a HtmlElement element.
    ///
    /// # Examples
    ///
    /// ```
    /// let document = Minimal::document();
    /// let h1_html = document.query_selector_html("h1");
    /// if h1_html.has_class("test"){
    ///     h1_html.set_prop("display", "none");
    /// }
    /// ```

    pub trait MinimalHtml {
        /// Convert HtmlElement to Element.
        fn to_el(&self) -> Element;
        /// Get offset parent as Element.
        fn offset_parent_el(&self) -> Element;
        /// Get offset parent as HtmlElement.
        fn offset_parent_html(&self) -> HtmlElement;
        /// Set property to Element, same as set_property().
        fn set_prop<'a, 's>(&self, property: &'a str, value: &'s str);
        /// Get property of Element, same as get_property().
        fn get_prop<'a>(&self, property: &'a str) -> String;
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
        /// Get Node of a List.
        fn get_node(&self, index: u32) -> Node;
        /// Get Node of a List as Element.
        fn get_el(&self, index: u32) -> Element;
        /// Get Node of a List as HtmlElement.
        fn get_html(&self, index: u32) -> HtmlElement;
        /// Add a class to all Nodes of a List.
        fn add_list_class<'a>(&self, value: &'a str);
        /// Remove a class to all Nodes of a List.
        fn remove_list_class<'a>(&self, value: &'a str);
    }

    /// Some function for a Node element.
    ///
    /// # Examples
    ///
    /// ```
    /// let document = Minimal::document();
    /// let h1 = document.query_selector_list("h1");
    /// let first_child_html = h1.get_node(0).to_html();
    /// assert_eq!(<HtmlElement>, first_child_html);
    /// let first_child_html = h1.first_child_html();
    /// assert_eq!(<HtmlElement>, first_child_html);
    /// let first_child_html = h1.get_html(0);
    /// assert_eq!(<HtmlElement>, first_child_html);
    /// ```
    ///
    pub trait MinimalNode {
        /// Convert a Node to Element.
        fn to_el(&self) -> Element;
        /// Convert a Node to HtmlElement.
        fn to_html(&self) -> HtmlElement;
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
        fn get_element_by_id_el<'a>(&self, value: &'a str) -> Element {
            self.get_element_by_id(value)
                .expect(&("no element with id: ".to_owned() + value))
        }
        fn get_element_by_id_html<'a>(&self, value: &'a str) -> HtmlElement {
            self.get_element_by_id(value)
                .expect(&("no element with id: ".to_owned() + value))
                .dyn_into::<HtmlElement>()
                .expect("It's not possible convert no element")
        }
        fn query_selector_html<'a>(&self, value: &'a str) -> HtmlElement {
            let value = match self
                .query_selector(value)
                .expect(&("No element with this selector found: ".to_owned() + value))
            {
                Some(e) => Some(
                    e.to_html(),
                ),
                None => None,
            };
            value.expect("There's no an element")
        }
        fn query_selector_el<'a>(&self, value: &'a str) -> Element {
            let value = match self
                .query_selector(value)
                .expect(&("No element with this selector found: ".to_owned() + value))
            {
                Some(e) => Some(e),
                None => None,
            };
            value.expect("no element found")
        }
        fn query_selector_list<'a>(&self, value: &'a str) -> NodeList {
            self.query_selector_all(value).expect("no one element found")
        }
        fn document_element_el(&self) -> Element {
            self.document_element().expect("no document element found")
        }
        fn document_element_html(&self) -> HtmlElement {
            self.document_element()
                .expect("no document element found")
                .to_html()
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
        fn has_class<'a>(&self, value: &'a str) -> bool {
            if self.class_name().contains(&("".to_owned() + value)) {
                true
            } else {
                false
            }
        }
        fn toggle_class<'a>(&self, value: &'a str) {
            if self.has_class(value) {
                self.remove_class(value);
            } else {
                self.add_class(value);
            }
        }
        fn add_class<'a>(&self, value: &'a str) {
            self.set_class_name(&(self.class_name() + &" ".to_owned() + value));
        }
        fn remove_class<'a>(&self, value: &'a str) {
            self.set_class_name(&(self.class_name().replace(&(" ".to_owned() + value), "")));
        }
        fn parent_element_el(&self) -> Element {
            self.parent_element().expect("No parent element found")
        }
        fn parent_element_html(&self) -> HtmlElement {
            self.parent_element_el()
                .to_html()
        }
        fn query_selector_el<'a>(&self, value: &'a str) -> Element {
            self.query_selector(value).expect("No element found").expect("there's been a problem")
        }
        fn query_selector_html<'a>(&self, value: &'a str) -> HtmlElement {
            self.query_selector_el(value).to_html()
        }
        fn query_selector_list<'a>(&self, value: &'a str) -> NodeList {
            self.query_selector_all(value).expect("No element found")
        }
        fn first_child_el(&self) -> Element {
            self.first_child().expect("No first child found").to_el()
        }
        fn first_child_html(&self) -> HtmlElement {
            self.first_child().expect("No first child found").to_html()
        }
        fn last_child_el(&self) -> Element {
            self.last_child().expect("No last child found").to_el()
        }
        fn last_child_html(&self) -> HtmlElement {
            self.last_child().expect("No last child found").to_html()
        }
        fn prev_sibling_el(&self) -> Element {
            self.previous_sibling().expect("No prev sibling found").to_el()
        }
        fn prev_sibling_html(&self) -> HtmlElement {
            self.previous_sibling().expect("No prev element found").to_html()
        }
        fn closest_el<'a>(&self, value: &'a str) -> Element {
            self.closest(value).expect("No closest element found").expect("There's been a problem")
        }
        fn closest_html<'a>(&self, value: &'a str) -> HtmlElement {
            self.closest_el(value).to_html()
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
        fn get_attr_node<'a>(&self, value: &'a str) -> Attr {
            self.get_attribute_node(value).expect("No attribute node found")
        }
        fn get_attr<'a>(&self, value: &'a str) -> String {
            self.get_attribute(value).expect("No attribute found")
        }
        fn get_attr_ns<'a>(&self, namespace: &'a str, localname: &'a str) -> String {
            self.get_attribute_ns(Some(namespace), localname).expect("There's been an error")
        }
        fn toggle_attr<'a>(&self, value: &'a str) -> bool {
            self.toggle_attribute(value)
                .expect("It's not possible toggle this attribute")
        }
        fn get_pref(&self) -> String {
            self.prefix().expect("No prefix found")
        }
        fn insert_adj_el<'s, 'a>(&self, where_: &'s str, element: Element) -> Element {
            self.insert_adjacent_element(where_, &element)
                .expect("It has not been possible insert element")
                .expect("There's been a problem")
        }
        fn remove_attr<'a>(&self, value: &'a str) {
            self.remove_attribute(value).expect("It's no possible remove attribute")
        }
        fn has_match<'a>(&self, value: &'a str) -> bool {
            self.matches(value).expect("There's been a problem")
        }
        fn get_node_value(&self) -> String {
            self.node_value().expect("No node value found")
        }
        fn get_text_content(&self) -> String {
            self.text_content().expect("No text content found")
        }
        fn app_child(&self, node: Node) -> Node {
            self.append_child(&node).expect("Failed to append child")
        }
    }

    impl MinimalHtml for HtmlElement {
        fn to_el(&self) -> Element {
            self.clone()
                .dyn_into::<Element>()
                .expect("It's not possible get an Element of Null")
        }
        fn offset_parent_el(&self) -> Element {
            self.offset_parent().expect("No offset parent found")
        }
        fn offset_parent_html(&self) -> HtmlElement {
            self.offset_parent()
                .expect("No offset parent found")
                .to_html()
        }
        fn set_prop<'a, 's>(&self, property: &'a str, value: &'s str) {
            self.style().set_property(property, value).expect("There's been a problem")
        }
        fn get_prop<'a>(&self, property: &'a str) -> String {
            self.style().get_property_value(property).expect("There's been a problem")
        }
    }

    impl MinimalList for NodeList {
        fn get_node(&self, index: u32) -> Node {
            self.item(index).expect(&("no element in position ".to_owned() + &index.to_string()))
        }
        fn get_el(&self, index: u32) -> Element {
            self.item(index)
                .expect(&("no element in position ".to_owned() + &index.to_string()))
                .dyn_into::<Element>()
                .expect("It's not possible get an Element of Null")
        }
        fn get_html(&self, index: u32) -> HtmlElement {
            self.item(index)
                .expect(&("no element in position ".to_owned() + &index.to_string()))
                .dyn_into::<HtmlElement>()
                .expect("It's not possible get an HtmlElement of Null")
        }
        fn add_list_class<'a>(&self, value: &'a str) {
            for i in 0..self.length() {
                if !self.get_html(i).has_class(value) {
                    self.get_html(i).add_class(value);
                }
            }
        }
        fn remove_list_class<'a>(&self, value: &'a str) {
            for i in 0..self.length() {
                if self.get_html(i).has_class(value) {
                    self.get_html(i).remove_class(value);
                }
            }
        }
    }

    impl MinimalNode for Node{
        fn to_el(&self) -> Element {
            self.clone().dyn_into::<Element>().expect("No Node element found")
        }
        fn to_html(&self) -> HtmlElement {
            self.clone().dyn_into::<HtmlElement>().expect("No Node element found")
        }
    }
}
