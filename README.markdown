# JMU

![JMU CI](https://github.com/alyxshang/jmu/actions/workflows/rust.yml/badge.svg)

***A parser and deserializer for a data markup language inspired by Jirai Kei.***

## ABOUT

This repository contains the source code for a parser for a data markup
language inspired by [Jirai Kei](https://aesthetics.fandom.com/wiki/Jirai_Kei). The parser is written
without any external dependencies and was made to be light and fast. The
language itself was designed by me and is an abbreviation for
***J***irai ***M***ark***U***p language. The crate does depend on the
[`serde`](https://serde.rs) library to handle deserialization of JMU data into Rust data
structures.

## SPECIFICATION

A specification of the ***J***irai ***M***ark***U***p Language can be found [here](https://alyxshang.boo/specs/jmu).

## USAGE

To add the ***JMU*** crate to your Rust project and to start
deserializing text written in the ***JMU*** format, add 
the following lines to the `dependencies` section of your 
project's `Cargo.toml`:

```TOML
serde = { version = "1.0.228", features = ["derive"] }
jmu = { git = "https://github.com/alyxshang/jmu", tag = "v.0.2.0" }
```

You would use the ***JMU*** crate in your code in a manner similar to
the one outlined in the code sample below:

```Rust
use jmu::from_str;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct MyData{
    pub name: String,
    pub age: usize,
    pub pi: f64,
    pub is_awesome: bool,
    pub is_political: bool,
    pub hobbies: Vec<String>,
    pub truth_table: Vec<bool>,
    pub nested_assignment: NestedAssignment
}

#[derive(Deserialize, Debug)]
struct NestedAssignment{
    pub favourite_fashion: String,
    pub also_interested_in: String,
    pub is_unhinged: bool
}

fn main(){
    let slice = r#"
        "name" >~< "Alyx"
        "age" >~< 2026
        "pi" >~< 3.1415
        "is_awesome" >~< true
        "is_political" >~< false
        "hobbies" >~< ["code", "art", "music"]
        "truth_table" >~< [true, false, true]
        "nested_assignment" >~< |
            "favourite_fashion" >~< "goth"
            "also_interested_in" >~< "Jirai Kei"
            "is_unhinged" >~< true
        |
    "#;
    let deserialized: MyData = from_str(&slice.to_string())
        .expect("Unable to deserialize the string slice.");
    println!("{:?}", deserialized); // should output the structure "MyData"
}
```

More information on the entities inside this crate can be obtained 
by cloning this repository and running the `cargo doc --open` command from
the root of the repository.

## CHANGELOG

### Version 0.1.0

- Initial release.
- Initial upload to GitHub.

### Version 0.2.0

- The `Position` structure was exported to its own module.
- More verbose error messages with exact position information.

## NOTE

- *JMU* by *Alyx Shang*.
- Licensed under the [FSL v1](https://alyxshang.boo/content/fair-software-license).
