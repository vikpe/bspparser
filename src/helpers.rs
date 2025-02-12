use crate::{BspFile, Face, Vertex};

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
