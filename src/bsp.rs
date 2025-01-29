// specification: https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_4.htm
// original: https://github.com/Thinkofname/rust-quake/blob/master/src/bsp/mod.rs
use crate::parse::*;
use anyhow::{anyhow as e, Error, Result};
use cgmath::Vector3;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;

const SIZE_TEXTURE_INFO: usize = 4 * 6 + 4 * 2 + 4 * 2;
const SIZE_VERTEX: usize = 4 * 3;
const SIZE_EDGE: usize = 2 + 2;
const SIZE_PLANE: usize = 4 * 3 + 4 + 4;
const SIZE_FACE: usize = 2 + 2 + 4 + 2 + 2 + 4 + 4;
const SIZE_MODEL: usize = (4 * 3) * 3 + 4 * 4 + 4 + 4 + 4;

#[derive(Debug, PartialEq)]
pub struct BspFile {
    pub version: Version,
    pub edge_list: Vec<i32>,
    pub edges: Vec<Edge>,
    pub entities: Vec<HashMap<String, String>>,
    pub faces: Vec<Face>,
    pub light_maps: Vec<u8>,
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
        // 1. BSP version
        let version = {
            let mut version_bytes = [0; 4];
            r.read_exact(&mut version_bytes)?;
            Version::try_from(version_bytes)?
        };

        let e_entities = Entry::read(r)?;
        let e_planes = Entry::read(r)?;
        let e_wall_textures = Entry::read(r)?;
        let e_vertices = Entry::read(r)?;
        let _e_visibility_list = Entry::read(r)?;
        let _e_nodes = Entry::read(r)?;
        let e_texture_info = Entry::read(r)?;
        let e_faces = Entry::read(r)?;
        let e_light_maps = Entry::read(r)?;
        let _e_clip_nodes = Entry::read(r)?;
        let _e_leaves = Entry::read(r)?;
        let _e_face_list = Entry::read(r)?;
        let e_edges = Entry::read(r)?;
        let e_edge_list = Entry::read(r)?;
        let e_models = Entry::read(r)?;

        // 2. Entities
        println!("2. Entities");
        let entities = {
            let mut entities_buf = vec![0; e_entities.size as usize];
            r.seek(SeekFrom::Start(e_entities.offset as u64))?;
            r.read_exact(&mut entities_buf)?;
            parse_entities(&entities_buf)
        }?;

        // 3. Planes
        println!("3. Planes");
        r.seek(SeekFrom::Start(e_planes.offset as u64))?;
        let planes = Plane::parse(e_planes.size as usize / SIZE_PLANE, r)?;

        // 4. Wall Textures
        println!("4. Wall Textures");
        r.seek(SeekFrom::Start(e_wall_textures.offset as u64))?;
        let textures = Texture::parse(r)?;

        // 5. Map Vertices
        r.seek(SeekFrom::Start(e_vertices.offset as u64))?;
        let vertice_count = e_vertices.size as usize / SIZE_VERTEX;
        let mut vertices = Vec::with_capacity(vertice_count);
        for _ in 0..vertice_count {
            vertices.push(Vector3::from(r.read_vector3_float()?));
        }

        // 5. Leaves Visibility lists.
        // 6. Nodes
        // (skipped)

        // 7. Texture Info
        r.seek(SeekFrom::Start(e_texture_info.offset as u64))?;
        let texture_info = TextureInfo::parse(e_texture_info.size as usize / SIZE_TEXTURE_INFO, r)?;

        // 8. Faces
        r.seek(SeekFrom::Start(e_faces.offset as u64))?;
        let faces = Face::parse(e_faces.size as usize / SIZE_FACE, r)?;

        // 9. Light Maps
        let mut light_maps = vec![0; e_light_maps.size as usize];
        r.seek(SeekFrom::Start(e_light_maps.offset as u64))?;
        r.read_exact(&mut light_maps)?;

        // 10. Clip Nodes
        // 11. Leaves
        // 12. Face List
        // (skipped)

        // 13. Edges
        r.seek(SeekFrom::Start(e_edges.offset as u64))?;
        let edges = Edge::parse(e_edges.size as usize / SIZE_EDGE, vertices, r)?;

        // 14. Edge List
        let edge_list_count = e_edge_list.size as usize / 4;
        let mut edge_list = Vec::with_capacity(edge_list_count);
        r.seek(SeekFrom::Start(e_edge_list.offset as u64))?;
        for _ in 0..edge_list_count {
            edge_list.push(r.read_long()?);
        }

        // 15. Models
        r.seek(SeekFrom::Start(e_models.offset as u64))?;
        let models = Model::parse(e_models.size as usize / SIZE_MODEL, r)?;

        // Done!
        Ok(BspFile {
            version,
            entities,
            light_maps,
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

#[derive(Debug, PartialEq)]
pub struct Model {
    pub bound: (Vector3<f32>, Vector3<f32>),
    pub origin: Vector3<f32>,
    pub face_indexes: Range<usize>,
}

impl Model {
    pub fn parse<R>(count: usize, r: &mut R) -> Result<Vec<Model>>
    where
        R: Read + Seek,
    {
        let mut models = Vec::with_capacity(count);

        for _ in 0..count {
            let bound_min = Vector3::from(r.read_vector3_float()?);
            let bound_max = Vector3::from(r.read_vector3_float()?);
            let origin = Vector3::from(r.read_vector3_float()?);
            let _node_index = [
                r.read_long()?,
                r.read_long()?,
                r.read_long()?,
                r.read_long()?,
            ];
            let _leafs = r.read_long()?;
            let face_start = r.read_long()?;
            let face_number = r.read_long()?;

            models.push(Model {
                bound: (bound_min, bound_max),
                origin,
                face_indexes: face_start as usize..(face_start as usize + face_number as usize),
            });
        }

        Ok(models)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Face {
    pub plane_index: usize,
    pub is_front: bool,
    pub edge_indexes: Range<usize>,
    pub texture_info_index: usize,
    pub type_light: u8,
    pub base_light: u8,
    pub light: [u8; 2],
    pub light_map: i32,
}

impl Face {
    pub fn parse<R>(count: usize, r: &mut R) -> Result<Vec<Face>>
    where
        R: Read + Seek,
    {
        let mut faces = Vec::with_capacity(count);

        for _ in 0..count {
            faces.push(Face {
                plane_index: r.read_ushort()? as usize,
                is_front: r.read_ushort()? == 0,
                edge_indexes: {
                    let start = r.read_long()? as usize;
                    start..(start + r.read_ushort()? as usize)
                },
                texture_info_index: r.read_ushort()? as usize,
                type_light: r.read_uchar()?,
                base_light: r.read_uchar()?,
                light: [r.read_uchar()?, r.read_uchar()?],
                light_map: r.read_long()?,
            });
        }

        Ok(faces)
    }
}

#[derive(Debug, PartialEq)]
pub struct Plane {
    pub normal: Vector3<f32>,
    pub distance: f32,
    pub kind: i32,
}

impl Plane {
    pub fn parse<R>(count: usize, r: &mut R) -> Result<Vec<Plane>>
    where
        R: Read + Seek,
    {
        let mut planes = Vec::with_capacity(count);

        for _ in 0..count {
            planes.push(Plane {
                normal: Vector3::from(r.read_vector3_float()?),
                distance: r.read_float()?,
                kind: r.read_long()?,
            });
        }

        Ok(planes)
    }
}

#[derive(Debug, PartialEq)]
pub struct Edge(pub Vector3<f32>, pub Vector3<f32>);

impl Edge {
    pub fn parse<R>(count: usize, vertices: Vec<Vector3<f32>>, r: &mut R) -> Result<Vec<Edge>>
    where
        R: Read + Seek,
    {
        let mut edges = Vec::with_capacity(count);
        for _ in 0..count {
            edges.push(Edge(
                vertices[r.read_ushort()? as usize],
                vertices[r.read_ushort()? as usize],
            ));
        }
        Ok(edges)
    }
}

#[derive(Debug, PartialEq)]
pub struct TextureInfo {
    pub vector_s: Vector3<f32>,
    pub dist_s: f32,
    pub vector_t: Vector3<f32>,
    pub dist_t: f32,
    pub texture_index: usize,
    pub is_animated: bool,
}

impl TextureInfo {
    pub fn parse<R>(count: usize, r: &mut R) -> Result<Vec<TextureInfo>>
    where
        R: Read + Seek,
    {
        let mut info = Vec::with_capacity(count);

        for _ in 0..count {
            info.push(TextureInfo {
                vector_s: Vector3::from(r.read_vector3_float()?),
                dist_s: r.read_float()?,
                vector_t: Vector3::from(r.read_vector3_float()?),
                dist_t: r.read_float()?,
                texture_index: r.read_ulong()? as usize,
                is_animated: r.read_ulong()? != 0,
            })
        }

        Ok(info)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Entry {
    offset: i32,
    size: i32,
}

impl Entry {
    fn read<R>(r: &mut R) -> Result<Entry>
    where
        R: Read,
    {
        Ok(Entry {
            offset: r.read_long()?,
            size: r.read_long()?,
        })
    }
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
            assert_eq!(bsp.edges.len(), 760);
            assert_eq!(bsp.faces.len(), 323);
            assert_eq!(bsp.light_maps.len(), 15850);
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
            assert_eq!(bsp.edges.len(), 8030);
            assert_eq!(bsp.faces.len(), 3236);
            assert_eq!(bsp.light_maps.len(), 134639);
            assert_eq!(bsp.models.len(), 7);
            assert_eq!(bsp.planes.len(), 835);
            assert_eq!(bsp.texture_info.len(), 272);
            assert_eq!(bsp.textures.len(), 59);
        }
        Ok(())
    }
}
