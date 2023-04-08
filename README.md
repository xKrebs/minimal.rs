
# Minimal

A web_sys utils for get Element and HtmlElement more convenient and easier.



## Usage/Examples

```rust
...
use Minimal::{MinimalDocument, MinimalList};
...
fn main() {
    let document = Minimal::document();
    let all_p = document.query_selector_list("p"); //NodeList
    let all_p_clone = all_p.clone();
    for i in 0..all_p.length(){
        let element = all_p_clone.get_html(i); //HtmlElement
        //do something
    }
}
```

