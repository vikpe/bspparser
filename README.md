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

## Fields

```rust
pub struct BspFile {
    pub version: BspVersion,
    pub header: BspHeader,
    pub edge_list: Vec<i32>,
    pub edges: Vec<Edge>,
    pub entities: Vec<HashMap<String, String>>,
    pub faces: Vec<Face>,
    pub lightmaps: Vec<u8>,
    pub models: Vec<Model>,
    pub planes: Vec<Plane>,
    pub texture_info: Vec<TextureInfo>,
    pub textures: Vec<Texture>,
    pub vertices: Vec<Vertex>,
}
```

## Helpers

```rust
pub fn get_face_texture(bsp: &BspFile, face: &Face) -> Texture
pub fn get_face_vertices(bsp: &BspFile, face: &Face) -> Vec<Vertex>
pub fn get_face_vertice_indexes(bsp: &BspFile, face: &Face) -> Vec<u32>
pub fn read_texture_image<R>( reader: &mut R, texture: &Texture, scale: TextureScale, ) -> Result<TextureImage>
```