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

# Running the example

1. Clone the repo
2. Navigate to `sample/`
3. Run `just run`

# Roadmap

- [x] Document object with writeable pub `.title` field.
- [x] `println!` goes to console.log.
- [x] Make `println!` go to console.log from a web worker / in a loop.
- [ ] Figure out the best API & implementation to do DOM updates
- [ ] Figure out the best API for defining components.
- [ ] Remove the dependency on webpack (webpack needs to die painfully in a large inferno)
- [ ] Server side rendering (make sure everything compiles on regular targets, besides wasm32-unknown-unknown
- [ ] `html!` should support multiple children
- [ ] IntelliJ should realize we're writing almost html inside `html!`.

My previous dabbling on Rust frontend is 
[https://github.com/kurtbuilds/rust-frontend](https://github.com/kurtbuilds/rust-frontend)
which contains more notes on desired properties for a frontend framework.


# Why not Yew?

List of things to change about yew:
- [ ] classes! should be implicit: https://yew.rs/docs/next/concepts/basic-web-technologies/css . Verbose otherwise.
- [ ] Bare words in html macro.
- [ ] Route should take url string or enum (https://yew.rs/docs/next/concepts/router)
- [ ] Yew is slow. Better than React but worse than Solid or wasm-bindgen.
- [ ] Something about it just feels very non-Rusty but also non-Javascripty? https://yew.rs/docs/next/concepts/html/components
    - [ ] Decide how you'd rewrite each of their examples to be less verbose?



### Yew Replacement

From

```rust
#[function_component]
fn MyComponent() -> Html {
    html! {
        { "This component has no properties!" }
    }
}
```

to 

```
pub fn MyComponent() -> Html {
    html! {
        This component has no properties!
    }
}
```

From

```rust
#[derive(Clone, PartialEq, Properties)]
struct Props {
    first_name: String,
    last_name: String,
}

#[function_component]
fn MyComponentWithProps(props: &Props) -> Html {
    let Props { first_name, last_name } = props;
    html! {
        <>first_name: {first_name} and last_name: {last_name}</>
    }
}
```

to

```rust
pub fn MyComponentWithProps(
    first_name: String,
    last_name: String,
) -> Html {
    html! {
        first_name: {first_name} and last_name: {last_name}
    }
}
```

From

```rust

#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Children,
}

#[function_component]
fn Container(props: &Props) -> Html {
    html! {
        <div id={props.id.clone()}>
            { props.children.clone() }
        </div>
    }
}
```

to

```rust
fn Container(
    id: String,
    children: Children,
) -> Html {
    html! {
        <div id={id}>
            { children }
        </div>
    }
}
```

They have some weird "For" syntax, see: https://yew.rs/docs/next/concepts/html/components#basic
Instead, Iterable<Html> should be templateable within html! macro. (Don't need to `.collect()`. )

Seems like they don't have keying always turned on? https://yew.rs/docs/next/concepts/html/lists

Is there a way to have intellij understand the html! macros?
- https://intellij-rust.github.io/2021/12/20/changelog-162.html
- https://github.com/intellij-rust/intellij-rust/issues/6732
- https://github.com/yewstack/yew/issues/439
- https://github.com/yewstack/yew/issues/1671
- https://github.com/intellij-rust/intellij-rust/issues/8076


Compiler should build by the item, not by the crate.
https://github.com/rust-lang/rust/issues/94878
https://github.com/rust-lang/rust/issues?q=is%3Aopen+author%3Acjgillot+
https://rust-lang.zulipchat.com/#narrow/stream/182449-t-compiler.2Fhelp

Fixing this would be *massively* transformative, because it would also unlock parallelizing the build of a crate, whereas right now its single core.
