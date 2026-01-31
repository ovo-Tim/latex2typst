use latex2typst::convert_markdown;

fn main() {
    let markdown = r#"# My Document

This is a **Markdown** document with _formatting_.

## Features

- Unordered lists
- Code blocks
- Links

## Example Code

```rust
fn main() {
    println!("Hello, Typst!");
}
```

## More Info

Check out [Typst](https://typst.app/) for more information.
"#;

    match convert_markdown(markdown) {
        Ok(typst) => {
            println!("=== Input (Markdown) ===");
            println!("{}", markdown);
            println!("\n=== Output (Typst) ===");
            println!("{}", typst);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
