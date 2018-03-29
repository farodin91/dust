# Dust - data driven tests in Rust



For example:
```rust
#![feature(plugin, decl_macro)]
#![plugin(dust)]

#[theory]
#[data(1,1)]
#[data(2,2)]
#[data(3,3)]
#[data(4,4)]
fn test_integer(a: i32, b: i32) {
    assert!(a==b);
}

#[theory]
#[data("test", "test")]
#[data("test2", "test2")]
fn test_str(a: &str, b: &str) {
    assert!(a==b);
}


#[test]
fn bla() {
    assert!(1==1);
}
```