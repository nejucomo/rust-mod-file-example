
# Rust Modules and Paths

The `mod foo;` declaration in [rust](http://rust-lang.org) always
searches for `foo.rs` or `foo/mod.rs` *relative to the path of the
source file* where the `mod`-declaration appears.

Here are four different filesystem layouts of a crate with `a::b::x`
to illustrate this in practice:

## case_0_monolithic

### ./root.rs

```
#[link(name = "case_0_monolithic", vers = "0.1", author = "nejucomo@gmail.com")];
#[crate_type = "lib"];


pub mod a {
    pub mod b {
        pub static x: uint = 42u;
    }
}
```

## case_1_external_b

### ./root.rs

```
#[link(name = "case_1_external_b", vers = "0.1", author = "nejucomo@gmail.com")];
#[crate_type = "lib"];


pub mod a {
    pub mod b;
}
```

### ./a/b.rs

```
pub static x: uint = 42u;
```

## case_2_external_monolithic_a

### ./root.rs

```
#[link(name = "case_2_external_monolithic_a", vers = "0.1", author = "nejucomo@gmail.com")];
#[crate_type = "lib"];


pub mod a;
```

### ./a.rs

```

pub mod b {
    pub static x: uint = 42u;
}
```

## case_3_external_a_and_b

### ./root.rs

```
#[link(name = "case_3_external_a_and_b", vers = "0.1", author = "nejucomo@gmail.com")];
#[crate_type = "lib"];


pub mod a;
```

### ./a.rs

```
pub mod b;
```

### ./b.rs

```
pub static x: uint = 42u;
```



## Notes:

This file was generated by `./build.py` which also verifies that the
example directories all build by running:

```bash
$ rust build -o /tmp/junk root.rs
```

The version of rust used to generate this file was:

```bash
$ rust --version
rust 0.8-pre (72f62ab 2013-09-15 14:00:52 -0700)
host: x86_64-unknown-linux-gnu

```
