# bspparser [![Test](https://github.com/vikpe/bspparser/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/bspparser/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/bspparser)](https://crates.io/crates/bspparser) [![docs.rs](https://img.shields.io/docsrs/bspparser)](https://docs.rs/bspparser/)

> Extract information from .bsp files

## Entities

```rust
let data = fs::read("dm3.mvd")?;

pub fn entities_as_hashmaps(data: &[u8]) -> Result<Vec<HashMap<String, String>>> { }
/*
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
    ...
]
*/

pub fn entities_as_string(data: &[u8]) -> Result<String> { }
/*
{
"wad" "gfx/base.wad"
"classname" "worldspawn"
"worldtype" "2"
"sounds" "6"
"message" "The Abandoned Base"
}
{
"classname" "light_fluoro"
"origin" "264 -32 88"
}
...
*/
```