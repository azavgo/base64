## Utility to encode into Base64 url-safe format

### Features: 
1. Encodes UTF-8 strings into Base64 url-safe format;

***Note:*** Base64 padding support will be added in the future versions;

### How to use this library:

1. Add to Cargo.toml:

```toml
    [dependencies]
    base64 = {git = "https://github.com/azavgo/base64", branch = "main"}
```

2. 

```rust
    use base64::*; 

    fn main() {     
        let input = "Many hands make light work."; 
        let output = base64_encode(input); 
        
        assert_eq!("TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu".to_string(), output);
    }
```