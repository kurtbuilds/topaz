# Topaz

This is a proof of concept library that demonstrates a Rust interface almost identical to 
writing Plain Old Javascript.

##### Example 
```rust
#[wasm_bindgen]
pub fn start() {
    topaz::start();
    
    // We achieve setting the document title purely from setting the `.title` field
    // by having the `.title` referenced via DerefMut and then checking for changes
    // on the `Drop` for the `Document` struct.
    // The consequence is that the document title is updated when the struct drops, rather
    // than immediately. 
    let mut doc = global::document();
    doc.title = "Topaz".to_string();

    // setInterval that's plain Rust. Literally nothing fancy for the end user here.
    let mut z = 0;
    global::set_interval(move || {
        z += 1;
        // To my knowledge, no other wasm crate captures println.
        // It's possible with the `internal_output_capture` rust feature, so it requires the nightly compiler
        // and is behind the capture-print crate flag.
        println!("Hello, world! {}", z);
        // What happens behind the scenes is println gets redirected to a Vec<u8>
        // and using the setInterval API, we flush the Vec<u8> to the console.
        // So the flushing isn't immediate.
    }, 1000);

    // alert that's also plain Rust. Also nothing fancy.
    alert(&format!("Hello, {}!", "Foobar"));
}
```

# Roadmap

- [x] Document object with writeable pub `.title` field.
- [x] `println!` goes to console.log.
- [x] Make `println!` go to console.log from a web worker / in a loop.
- [ ] Figure out the best way to do DOM updates
- [ ] Remove the dependency on webpack (webpack needs to burn in a large inferno)