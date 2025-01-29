// specification: https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_4.htm
// original: https://github.com/Thinkofname/rust-quake/blob/master/src/bsp/mod.rs
use crate::parse::*;
use anyhow::{anyhow as e, Error, Result};
use binrw::{BinRead, BinResult};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;

pub trait FromReader {
    type OutputType;
    fn num_elements(size: u32) -> u32;
    fn from_reader<R: Read + Seek>(reader: &mut R) -> BinResult<Self::OutputType>;
}

impl<T: BinRead + for<'a> BinRead<Args<'a> = ()>> FromReader for T {
    type OutputType = T;

    fn num_elements(size: u32) -> u32 {
        size / (size_of::<T>() as u32)
    }

    fn from_reader<R: Read + Seek>(reader: &mut R) -> BinResult<Self::OutputType> {
        T::read_le(reader)
    }
}

pub fn read_vec<T: FromReader>(
    reader: &mut (impl Read + Seek),
    entry: &Entry,
) -> BinResult<Vec<T::OutputType>> {
    reader.seek(SeekFrom::Start(entry.offset as u64))?;
    let count = T::num_elements(entry.size);
    let mut elements = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let element = T::from_reader(reader)?;
        elements.push(element);
    }
    Ok(elements)
}

#[derive(Debug)]
pub struct BspFile {
    pub version: Version,
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
}

impl BspFile {
    pub fn parse<R>(r: &mut R) -> Result<BspFile>
    where
        R: Read + Seek,
    {
        // 0. BSP version
        let version = {
            let mut version_bytes = [0; 4];
            r.read_exact(&mut version_bytes)?;
            Version::try_from(version_bytes)?
        };

        let h = BspHeader::read(r)?;

        // 1. Entities
        let entities = parse_entities(&read_vec::<u8>(r, &h.entities)?)?;

        // 2. Planes
        let planes = read_vec::<Plane>(r, &h.planes)?;

        // 3. Wall Textures
        r.seek(SeekFrom::Start(h.textures.offset as u64))?;
        let textures = Texture::parse(r)?;

        // 4. Map Vertices
        // println!("4. Map Vertices");
        /*let vertices = {
            let mut vertices = Vec::with_capacity(h.vertices.size as usize / SIZE_VERTEX);
            r.seek(SeekFrom::Start(h.vertices.offset as u64))?;
            for _ in 0..vertices.capacity() {
                vertices.push(Vector3::from(r.read_vector3_float()?));
            }
            vertices
        };*/

        // 5. Leaves Visibility lists.
        // 6. Nodes
        // (skipped)

        // 7. Texture Info
        let texture_info = read_vec::<TextureInfo>(r, &h.texture_info)?;

        // 8. Faces
        let faces = read_vec::<Face>(r, &h.faces)?;

        // 9. Light Maps
        let lightmaps = read_vec::<u8>(r, &h.lightmaps)?;

        // 10. Clip Nodes
        // 11. Leaves
        // 12. Face List
        // (skipped)

        // 13. Edges
        let edges = read_vec::<Edge>(r, &h.edges)?;

        // 14. Edge List
        let edge_list = read_vec::<i32>(r, &h.edge_list)?;

        // 15. Models
        let models = read_vec::<Model>(r, &h.models)?;

        // Done!
        Ok(BspFile {
            version,
            header: h,
            entities,
            lightmaps,
            textures,
            texture_info,
            edges,
            edge_list,
            planes,
            faces,
            models,
        })
    }
}

/*
#[derive(Debug, BinRead)]
#[br(little)]
pub enum BspVersion {
    #[br(magic(29u32))]
    Bsp29,
    #[br(magic(844124994u32))]
    Bsp2,
}
*/

#[derive(Debug, BinRead)]
#[br(little)]
pub struct BspHeader {
    // pub version: BspVersion,
    pub entities: Entry,
    pub planes: Entry,
    pub textures: Entry,
    pub vertices: Entry,
    pub visibility: Entry,
    pub nodes: Entry,
    pub texture_info: Entry,
    pub faces: Entry,
    pub lightmaps: Entry,
    pub clipnodes: Entry,
    pub leaves: Entry,
    pub face_list: Entry,
    pub edges: Entry,
    pub edge_list: Entry,
    pub models: Entry,
}

fn parse_entities(bytes: &[u8]) -> Result<Vec<HashMap<String, String>>> {
    let entities_str = quake_text::bytestr::to_unicode(bytes);
    let mut entities = Vec::new();
    let mut current_entity = HashMap::new();

    for line in entities_str.lines() {
        let line = line.trim();

        if line == "{" {
            current_entity = HashMap::new();
        } else if line == "}" {
            entities.push(current_entity.clone());
        } else {
            let (key, value) = line
                .trim_matches('"')
                .split_once("\" \"")
                .unwrap_or_default();
            current_entity.insert(key.to_string(), value.to_string());
        }
    }
    Ok(entities)
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V29,
    BSP2,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::V29 => write!(f, "29"),
            Version::BSP2 => write!(f, "BSP2"),
        }
    }
}

impl TryFrom<[u8; 4]> for Version {
    type Error = Error;

    fn try_from(version: [u8; 4]) -> Result<Self, Self::Error> {
        match version {
            [29, 0, 0, 0] => Ok(Version::V29),
            [66, 83, 80, 50] => Ok(Version::BSP2),
            _ => Err(e!("Unsupported BSP version")),
        }
    }
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct BoundingBox {
    pub min: QVec3,
    pub max: QVec3,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Model {
    pub bounds: BoundingBox,
    pub origin: QVec3,
    pub bsp: i32,
    pub clip1: i32,
    pub clip2: i32,
    pub node3: i32,
    pub leaf_count: i32,
    pub face_index_from: i32,
    pub face_count: i32,
}

impl Model {
    pub fn face_indexes(&self) -> Range<usize> {
        (self.face_index_from as usize)..(self.face_index_from as usize + self.face_count as usize)
    }
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Face {
    pub plane_index: u16,
    pub side: u16,
    pub edge_index_from: u32,
    pub edge_index_count: u16,
    pub texture_info_index: u16,
    pub type_light: u8,
    pub base_light: u8,
    pub light: [u8; 2],
    pub lightmap: u32,
}

type QVec3 = [f32; 3];

#[derive(Debug, BinRead, PartialEq)]
#[br(little)]
pub struct Plane {
    pub normal: QVec3,
    pub distance: f32,
    pub kind: i32,
}

#[derive(Debug, BinRead, PartialEq)]
#[br(little)]
pub struct Edge(pub [u16; 3], pub [u16; 3]);

#[derive(Debug, BinRead, PartialEq)]
#[br(little)]
pub struct EdgeV2(pub [u32; 3], pub [u32; 3]);

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Coord {
    pub vec: QVec3,
    pub offset: f32,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct TextureInfo {
    pub u: Coord,
    pub v: Coord,
    pub texture_id: u32,
    pub flags: u32,
}

#[derive(Debug, Default)]
pub struct Texture {
    pub id: i32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub pictures: [Picture; 4],
}

impl Texture {
    pub fn parse<R>(r: &mut R) -> Result<Vec<Texture>>
    where
        R: Read + Seek,
    {
        let base_offset = r.stream_position()?;
        let count = r.read_long()?;

        let mut textures = Vec::with_capacity(count as usize);
        for id in 0..count {
            let offset = r.read_long()?;
            if offset == -1 {
                textures.push(Texture {
                    id: -1,
                    ..Texture::default()
                });
                continue;
            }

            let coffset = r.stream_position()?;

            r.seek(SeekFrom::Start(base_offset + offset as u64))?;
            let name = from_cstring(&read_string!(r, 16))?;
            let width = r.read_ulong()?;
            let height = r.read_ulong()?;
            let offsets = [
                r.read_ulong()?,
                r.read_ulong()?,
                r.read_ulong()?,
                r.read_ulong()?,
            ];

            let mut tex = Texture {
                id,
                name,
                width,
                height,
                pictures: [
                    Picture::default(),
                    Picture::default(),
                    Picture::default(),
                    Picture::default(),
                ],
            };

            for (i, o) in offsets.into_iter().enumerate() {
                r.seek(SeekFrom::Start(base_offset + offset as u64 + o as u64))?;
                let w = width >> i;
                let h = height >> i;
                let mut data = vec![0; (w * h) as usize];
                r.read_exact(&mut data)?;
                tex.pictures[i] = Picture {
                    width: w,
                    height: h,
                    data,
                };
            }

            textures.push(tex);
            r.seek(SeekFrom::Start(coffset))?;
        }

        Ok(textures)
    }
}

#[derive(Debug, Default)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(BinRead, Debug, Default)]
#[br(little)]
pub struct Entry {
    offset: u32,
    size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_parse_v29() -> Result<()> {
        {
            let file = &mut fs::File::open("tests/files/povdmm4.bsp")?;
            let bsp = BspFile::parse(file)?;

            assert_eq!(bsp.entities.len(), 26);
            assert_eq!(
                bsp.entities.first(),
                Some(&HashMap::from([
                    ("classname".to_string(), "worldspawn".to_string()),
                    (
                        "message".to_string(),
                        "DMM4 Arena\\nBy Povo-Hat (http://povo-hat.besmella-quake.com)\\n"
                            .to_string()
                    ),
                    ("sounds".to_string(), "0".to_string()),
                    ("worldtype".to_string(), "1".to_string()),
                ]))
            );
            assert_eq!(
                bsp.entities.last(),
                Some(&HashMap::from([
                    ("classname".to_string(), "light".to_string()),
                    ("origin".to_string(), "192 -128 -128".to_string()),
                ]))
            );

            assert_eq!(bsp.edge_list.len(), 1518);
            assert_eq!(bsp.edges.len(), 253);
            assert_eq!(bsp.faces.len(), 323);
            assert_eq!(bsp.lightmaps.len(), 15850);
            assert_eq!(bsp.models.len(), 5);
            assert_eq!(bsp.planes.len(), 191);
            assert_eq!(bsp.texture_info.len(), 21);
            assert_eq!(bsp.textures.len(), 8);
        }
        {
            let file = &mut fs::File::open("tests/files/dm3_gpl.bsp")?;
            let bsp = BspFile::parse(file)?;

            assert_eq!(bsp.entities.len(), 211);
            assert_eq!(
                bsp.entities.first(),
                Some(&HashMap::from([
                    ("classname".to_string(), "worldspawn".to_string()),
                    ("message".to_string(), "The Abandoned Base".to_string()),
                    ("sounds".to_string(), "6".to_string()),
                    ("wad".to_string(), "gfx/base.wad".to_string()),
                    ("worldtype".to_string(), "2".to_string()),
                ]))
            );
            assert_eq!(
                bsp.entities.last(),
                Some(&HashMap::from([
                    ("classname".to_string(), "info_intermission".to_string()),
                    ("mangle".to_string(), "20 240 0".to_string()),
                    ("origin".to_string(), "1840 256 64".to_string()),
                ]))
            );

            assert_eq!(bsp.edge_list.len(), 16002);
            assert_eq!(bsp.edges.len(), 2676);
            assert_eq!(bsp.faces.len(), 3236);
            assert_eq!(bsp.lightmaps.len(), 134639);
            assert_eq!(bsp.models.len(), 7);
            assert_eq!(bsp.planes.len(), 835);
            assert_eq!(bsp.texture_info.len(), 272);
            assert_eq!(bsp.textures.len(), 59);
        }
        Ok(())
    }
}
