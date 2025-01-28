# bspparser [![Test](https://github.com/vikpe/bspparser/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/bspparser/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/bspparser)](https://crates.io/crates/bspparser) [![docs.rs](https://img.shields.io/docsrs/bspparser)](https://docs.rs/bspparser/)

> Parse Quake .bsp files

## Usage

```rust
let file = &mut fs::File::open("tests/files/povdmm4.bsp")?;
let bsp = BspFile::parse(file)?;
println!("{:?}", bsp.entities);
```

```json
[
    {
        "wad": "gfx/base.wad", 
        "worldtype": "2", 
        "sounds": "6", 
        "message": "The Abandoned Base", 
        "classname": "worldspawn"
    },
    {
        "classname": "light_fluoro",
        "origin": "264 -32 88"
    }
    // ...
]
```