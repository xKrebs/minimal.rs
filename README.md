
# Minimal

A web_sys utils for get Element and HtmlElement more convenient and easier.

# Install
If you don't like uppercase, add this in Cargo.toml
minimal = { package = "Minimal", version = "0.1.2" }
instead of
Minimal = "0.1.2"

## Usage/Examples

```rust
...
use Minimal::{MinimalDocument, MinimalList};
...
fn main() {
    let document = Minimal::document();
    let container = document.query_selector_html(".container"); //HtmlElement
    let all_p = container.query_selector_list("p"); //NodeList
    let all_p_clone = all_p.clone();
    for i in 0..all_p.length(){
        let element = all_p_clone.get_html(i); //HtmlElement
        //do something
    }
}
```

