# returns
> ### compile now, figure it out later

> turning red squiggles into future problems,
because sometimes you just want the compiler to shut the <span style="color: #FF0000;">fun</span> up

# Install
add this to your `Cargo.toml`
```toml
[dependencies]
returns = "0.1.0"
```

# Problem
![Image](/img/issue.png)

```console
> cargo run                                                                         
```
![Image](/img/error.png)

# Solution
> ### satisfy compiler now, implement later
```rust
fn main() {
    let _ = diagnose::<i32>();
}
pub fn diagnose<T>() -> T 
{
    // ..
    returns::return_shit!()
}
```

## `returns::return_shit!()` Behaviour
 - ### panics in *debug-mode*
 `cargo run` is *debug* mode by default, if you forget to write the valid return type, it will panic with information about the line you forget to implement
 ```console
 > cargo run
 ```
 ![Image](/img/s_debug_panic.png)

 - ### compile error in *release-mode*
 `return_shit!` <span style="color: #FF0000;">can not</span> reach production or release versions, because it will trigger **compiling error**
 ```console
 > cargo run --release
 ```
![Image](/img/s_re_ce.png)



# Another solution
### `returns::return_default!()`
```rust
pub fn diagnose<T>() -> T 
where 
    T:Default
{
    // ..
    returns::return_default!()
}
```
but `T` must implement `Default`

### `returns::return_default!()` behaviour
- **debug-mode** 
```console
> cargo run
```
it will compile normally and returns the `<T as Default>::default()` value
- **release-mode**
it will trigger **compiling error** about the default implementation you forgot to adjust 
![Image](/img/d_r_ce.png)

if you insist to use the `<T as Default>::default()` in release mode you just can use `returns::return_default!(force)`
```rust
pub fn diagnose<T>() -> T 
where 
    T:Default
{
    // ..
    returns::return_default!(force)
}
```


```markdown

> and you are all green again, go for it
```