// specification: https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_4.htm
// inpiration from: https://github.com/Thinkofname/rust-quake/blob/master/src/bsp/mod.rs
use anyhow::{anyhow as e, Error, Result};
use binrw::{BinRead, BinResult};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;

#[derive(Debug)]
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
    pub vertices: Vec<Vertex>,
}

impl BspFile {
    pub fn parse<R>(r: &mut R) -> Result<BspFile>
    where
        R: Read + Seek,
    {
        let version = {
            let mut bytes = [0; 4];
            r.read_exact(&mut bytes)?;
            BspVersion::try_from(bytes)?
        };

        let h = BspHeader::read(r)?;
        let entities = parse_entities(&read_vec::<u8>(r, &h.entities)?)?;
        let planes = read_vec::<Plane>(r, &h.planes)?;
        let texture_info = read_vec::<TextureInfo>(r, &h.texture_info)?;
        let vertices = read_vec::<Vertex>(r, &h.vertices)?;
        let lightmaps = read_vec::<u8>(r, &h.lightmaps)?;
        let edge_list = read_vec::<i32>(r, &h.edge_list)?;
        let models = read_vec::<Model>(r, &h.models)?;

        // version specific (precision)
        let (faces, edges) = match version {
            BspVersion::V29 => {
                let faces = read_vec::<FaceV1Reader>(r, &h.faces)?;
                let edges = read_vec::<EdgeV1Reader>(r, &h.edges)?;
                (faces, edges)
            }
            BspVersion::BSP2 => {
                let faces = read_vec::<Face>(r, &h.faces)?;
                let edges = read_vec::<Edge>(r, &h.edges)?;
                (faces, edges)
            }
        };

        Ok(BspFile {
            version,
            header: h,
            edge_list,
            edges,
            entities,
            faces,
            lightmaps,
            models,
            planes,
            texture_info,
            vertices,
        })
    }
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct BspHeader {
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
pub enum BspVersion {
    V29,
    BSP2,
}

impl Display for BspVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BspVersion::V29 => write!(f, "29"),
            BspVersion::BSP2 => write!(f, "BSP2"),
        }
    }
}

impl TryFrom<[u8; 4]> for BspVersion {
    type Error = Error;

    fn try_from(version: [u8; 4]) -> Result<Self, Self::Error> {
        match version {
            [29, 0, 0, 0] => Ok(BspVersion::V29),
            [66, 83, 80, 50] => Ok(BspVersion::BSP2),
            _ => Err(e!("Unsupported BSP version")),
        }
    }
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Model {
    pub bounds: BoundingBox,
    pub origin: [f32; 3],
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
    pub plane_index: u32,
    pub side: u32,
    pub edge_index_from: u32,
    pub edge_index_count: u32,
    pub texture_info_index: u32,
    pub type_light: u8,
    pub base_light: u8,
    pub light: [u8; 2],
    pub lightmap: u32,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct FaceV1 {
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

struct FaceV1Reader;

impl FromReader for FaceV1Reader {
    type OutputType = Face;

    fn element_count(size: u32) -> u32 {
        size / (size_of::<FaceV1>() as u32)
    }

    fn from_reader<R: Read + Seek>(reader: &mut R) -> BinResult<Self::OutputType> {
        let v = FaceV1::read_le(reader)?;
        Ok(Face {
            plane_index: v.plane_index as u32,
            side: v.side as u32,
            edge_index_from: v.edge_index_from,
            edge_index_count: v.edge_index_count as u32,
            texture_info_index: v.texture_info_index as u32,
            type_light: v.type_light,
            base_light: v.base_light,
            light: v.light,
            lightmap: v.lightmap,
        })
    }
}

#[derive(Debug, PartialEq, BinRead)]
#[br(little)]
pub struct Plane {
    pub normal: [f32; 3],
    pub distance: f32,
    pub kind: i32,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Edge {
    pub v0: u32,
    pub v1: u32,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct EdgeV1 {
    pub v0: u16,
    pub v1: u16,
}

pub struct EdgeV1Reader;

impl FromReader for EdgeV1Reader {
    type OutputType = Edge;

    fn element_count(size: u32) -> u32 {
        size / (size_of::<EdgeV1>() as u32)
    }

    fn from_reader<R: Read + Seek>(reader: &mut R) -> BinResult<Self::OutputType> {
        let v = EdgeV1::read_le(reader)?;
        Ok(Edge {
            v0: v.v0 as u32,
            v1: v.v1 as u32,
        })
    }
}

#[derive(Debug, BinRead)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Coord {
    pub vec: [f32; 3],
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

#[derive(Debug, BinRead)]
#[br(little)]
pub struct Entry {
    offset: u32,
    size: u32,
}

pub trait FromReader {
    type OutputType;
    fn element_count(size: u32) -> u32;
    fn from_reader<R: Read + Seek>(reader: &mut R) -> BinResult<Self::OutputType>;
}

impl<T: BinRead + for<'a> BinRead<Args<'a> = ()>> FromReader for T {
    type OutputType = T;

    fn element_count(size: u32) -> u32 {
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
    let count = T::element_count(entry.size);
    let mut elements = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let element = T::from_reader(reader)?;
        elements.push(element);
    }
    Ok(elements)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_parse_bsp2() -> Result<()> {
        let file = &mut fs::File::open("tests/files/dust2qw.bsp")?;
        let bsp = BspFile::parse(file)?;
        assert_eq!(bsp.entities.len(), 66);
        assert_eq!(bsp.edge_list.len(), 33556);
        assert_eq!(bsp.edges.len(), 16879);
        assert_eq!(bsp.faces.len(), 7116);
        assert_eq!(bsp.lightmaps.len(), 177828);
        assert_eq!(bsp.models.len(), 5);
        assert_eq!(bsp.planes.len(), 3779);
        assert_eq!(bsp.texture_info.len(), 2133);
        assert_eq!(bsp.vertices.len(), 8825);
        Ok(())
    }

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
            assert_eq!(bsp.edges.len(), 760);
            assert_eq!(bsp.faces.len(), 323);
            assert_eq!(bsp.lightmaps.len(), 15850);
            assert_eq!(bsp.models.len(), 5);
            assert_eq!(bsp.planes.len(), 191);
            assert_eq!(bsp.texture_info.len(), 21);
            assert_eq!(bsp.vertices.len(), 416);
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
            assert_eq!(bsp.edges.len(), 8030);
            assert_eq!(bsp.faces.len(), 3236);
            assert_eq!(bsp.lightmaps.len(), 134639);
            assert_eq!(bsp.models.len(), 7);
            assert_eq!(bsp.planes.len(), 835);
            assert_eq!(bsp.texture_info.len(), 272);
            assert_eq!(bsp.vertices.len(), 4544);
        }
        Ok(())
    }
}
