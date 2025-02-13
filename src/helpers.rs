use crate::ioextra::read_vec;
use crate::{BspFile, Entry, Face, Texture, TextureInfo, Vertex};
use anyhow::{anyhow as e, Result};
use std::io::{Read, Seek};

pub fn get_face_texture(bsp: &BspFile, face: &Face) -> Result<Texture> {
    let info = get_face_texture_info(bsp, face)?;

    match bsp.textures.get(info.texture_index as usize) {
        None => Err(e!("Texture index out of bounds")),
        Some(texture) => Ok(texture.clone()),
    }
}

pub fn get_face_texture_info(bsp: &BspFile, face: &Face) -> Result<TextureInfo> {
    match bsp.texture_info.get(face.texture_info_index as usize) {
        None => Err(e!("Texture info index out of bounds")),
        Some(info) => Ok(*info),
    }
}

pub fn get_face_vertices(bsp: &BspFile, face: &Face) -> Vec<Vertex> {
    get_face_vertice_indexes(bsp, face)
        .iter()
        .map(|vertex_id| bsp.vertices[*vertex_id as usize])
        .collect::<Vec<Vertex>>()
}

pub fn get_face_vertice_indexes(bsp: &BspFile, face: &Face) -> Vec<u32> {
    bsp.edge_list
        .iter()
        .skip(face.edge_list_index as usize)
        .take(face.edge_count as usize)
        .cloned()
        .map(|edge_index| match edge_index >= 0 {
            true => bsp.edges[edge_index as usize].v0,
            false => bsp.edges[-edge_index as usize].v1,
        })
        .collect::<Vec<u32>>()
}

pub enum TextureScale {
    Full,    // 1
    Half,    // 1/2
    Quarter, // 1/4
    Eighth,  // 1/8
}

pub struct TextureImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn read_texture_image<R>(
    reader: &mut R,
    texture: &Texture,
    scale: TextureScale,
) -> Result<TextureImage>
where
    R: Read + Seek,
{
    let offset = match &scale {
        TextureScale::Full => texture.offset1,
        TextureScale::Half => texture.offset2,
        TextureScale::Quarter => texture.offset4,
        TextureScale::Eighth => texture.offset8,
    };
    let scale_factor = match &scale {
        TextureScale::Full => 1.,
        TextureScale::Half => 0.5,
        TextureScale::Quarter => 0.25,
        TextureScale::Eighth => 0.125,
    };
    let width = ((texture.width as f32) * scale_factor) as u32;
    let height = ((texture.height as f32) * scale_factor) as u32;
    let size = width * height;

    let color_indexes = read_vec::<u8>(reader, &Entry { offset, size })?;
    let mut pixel_colors: Vec<u8> = vec![0; color_indexes.len() * 3];

    for (i, index) in color_indexes.iter().enumerate() {
        let offset = *index as usize * 3;
        pixel_colors[i * 3] = PALETTE[offset];
        pixel_colors[i * 3 + 1] = PALETTE[offset + 1];
        pixel_colors[i * 3 + 2] = PALETTE[offset + 2];
    }

    Ok(TextureImage {
        width,
        height,
        data: pixel_colors,
    })
}

pub const PALETTE: [u8; 768] = [
    0, 0, 0, 15, 15, 15, 31, 31, 31, 47, 47, 47, 63, 63, 63, 75, 75, 75, 91, 91, 91, 107, 107, 107,
    123, 123, 123, 139, 139, 139, 155, 155, 155, 171, 171, 171, 187, 187, 187, 203, 203, 203, 219,
    219, 219, 235, 235, 235, 15, 11, 7, 23, 15, 11, 31, 23, 11, 39, 27, 15, 47, 35, 19, 55, 43, 23,
    63, 47, 23, 75, 55, 27, 83, 59, 27, 91, 67, 31, 99, 75, 31, 107, 83, 31, 115, 87, 31, 123, 95,
    35, 131, 103, 35, 143, 111, 35, 11, 11, 15, 19, 19, 27, 27, 27, 39, 39, 39, 51, 47, 47, 63, 55,
    55, 75, 63, 63, 87, 71, 71, 103, 79, 79, 115, 91, 91, 127, 99, 99, 139, 107, 107, 151, 115,
    115, 163, 123, 123, 175, 131, 131, 187, 139, 139, 203, 0, 0, 0, 7, 7, 0, 11, 11, 0, 19, 19, 0,
    27, 27, 0, 35, 35, 0, 43, 43, 7, 47, 47, 7, 55, 55, 7, 63, 63, 7, 71, 71, 7, 75, 75, 11, 83,
    83, 11, 91, 91, 11, 99, 99, 11, 107, 107, 15, 7, 0, 0, 15, 0, 0, 23, 0, 0, 31, 0, 0, 39, 0, 0,
    47, 0, 0, 55, 0, 0, 63, 0, 0, 71, 0, 0, 79, 0, 0, 87, 0, 0, 95, 0, 0, 103, 0, 0, 111, 0, 0,
    119, 0, 0, 127, 0, 0, 19, 19, 0, 27, 27, 0, 35, 35, 0, 47, 43, 0, 55, 47, 0, 67, 55, 0, 75, 59,
    7, 87, 67, 7, 95, 71, 7, 107, 75, 11, 119, 83, 15, 131, 87, 19, 139, 91, 19, 151, 95, 27, 163,
    99, 31, 175, 103, 35, 35, 19, 7, 47, 23, 11, 59, 31, 15, 75, 35, 19, 87, 43, 23, 99, 47, 31,
    115, 55, 35, 127, 59, 43, 143, 67, 51, 159, 79, 51, 175, 99, 47, 191, 119, 47, 207, 143, 43,
    223, 171, 39, 239, 203, 31, 255, 243, 27, 11, 7, 0, 27, 19, 0, 43, 35, 15, 55, 43, 19, 71, 51,
    27, 83, 55, 35, 99, 63, 43, 111, 71, 51, 127, 83, 63, 139, 95, 71, 155, 107, 83, 167, 123, 95,
    183, 135, 107, 195, 147, 123, 211, 163, 139, 227, 179, 151, 171, 139, 163, 159, 127, 151, 147,
    115, 135, 139, 103, 123, 127, 91, 111, 119, 83, 99, 107, 75, 87, 95, 63, 75, 87, 55, 67, 75,
    47, 55, 67, 39, 47, 55, 31, 35, 43, 23, 27, 35, 19, 19, 23, 11, 11, 15, 7, 7, 187, 115, 159,
    175, 107, 143, 163, 95, 131, 151, 87, 119, 139, 79, 107, 127, 75, 95, 115, 67, 83, 107, 59, 75,
    95, 51, 63, 83, 43, 55, 71, 35, 43, 59, 31, 35, 47, 23, 27, 35, 19, 19, 23, 11, 11, 15, 7, 7,
    219, 195, 187, 203, 179, 167, 191, 163, 155, 175, 151, 139, 163, 135, 123, 151, 123, 111, 135,
    111, 95, 123, 99, 83, 107, 87, 71, 95, 75, 59, 83, 63, 51, 67, 51, 39, 55, 43, 31, 39, 31, 23,
    27, 19, 15, 15, 11, 7, 111, 131, 123, 103, 123, 111, 95, 115, 103, 87, 107, 95, 79, 99, 87, 71,
    91, 79, 63, 83, 71, 55, 75, 63, 47, 67, 55, 43, 59, 47, 35, 51, 39, 31, 43, 31, 23, 35, 23, 15,
    27, 19, 11, 19, 11, 7, 11, 7, 255, 243, 27, 239, 223, 23, 219, 203, 19, 203, 183, 15, 187, 167,
    15, 171, 151, 11, 155, 131, 7, 139, 115, 7, 123, 99, 7, 107, 83, 0, 91, 71, 0, 75, 55, 0, 59,
    43, 0, 43, 31, 0, 27, 15, 0, 11, 7, 0, 0, 0, 255, 11, 11, 239, 19, 19, 223, 27, 27, 207, 35,
    35, 191, 43, 43, 175, 47, 47, 159, 47, 47, 143, 47, 47, 127, 47, 47, 111, 47, 47, 95, 43, 43,
    79, 35, 35, 63, 27, 27, 47, 19, 19, 31, 11, 11, 15, 43, 0, 0, 59, 0, 0, 75, 7, 0, 95, 7, 0,
    111, 15, 0, 127, 23, 7, 147, 31, 7, 163, 39, 11, 183, 51, 15, 195, 75, 27, 207, 99, 43, 219,
    127, 59, 227, 151, 79, 231, 171, 95, 239, 191, 119, 247, 211, 139, 167, 123, 59, 183, 155, 55,
    199, 195, 55, 231, 227, 87, 127, 191, 255, 171, 231, 255, 215, 255, 255, 103, 0, 0, 139, 0, 0,
    179, 0, 0, 215, 0, 0, 255, 0, 0, 255, 243, 147, 255, 247, 199, 255, 255, 255, 159, 91, 83,
];
