use std::collections::HashMap;
use crate::hod::{HODModel, HODVertex, Vector3, Vector2, HODMesh, HODMeshPart};
use crate::iff::IffChunk;

#[derive(Hash, PartialEq, Eq, Clone)]
struct HashableVertex {
    px: u32, py: u32, pz: u32,
    nx: u32, ny: u32, nz: u32,
    c: u32,
    u: u32, v: u32,
    tx: u32, ty: u32, tz: u32,
    bx: u32, by: u32, bz: u32,
    skinning: Option<Vec<u8>>,
}

fn f32_to_u32(f: f32) -> u32 {
    if f.is_nan() { 0 } else { f.to_bits() }
}

impl HashableVertex {
    fn from_hod(v: &HODVertex) -> Self {
        let (nx, ny, nz) = v.normal.as_ref().map_or((0, 0, 0), |n| (f32_to_u32(n.x), f32_to_u32(n.y), f32_to_u32(n.z)));
        let (u, v_uv) = v.uv.as_ref().map_or((0, 0), |uv| (f32_to_u32(uv.u), f32_to_u32(uv.v)));
        let (tx, ty, tz) = v.tangent.as_ref().map_or((0, 0, 0), |t| (f32_to_u32(t.x), f32_to_u32(t.y), f32_to_u32(t.z)));
        let (bx, by, bz) = v.binormal.as_ref().map_or((0, 0, 0), |b| (f32_to_u32(b.x), f32_to_u32(b.y), f32_to_u32(b.z)));
        
        Self {
            px: f32_to_u32(v.position.x), py: f32_to_u32(v.position.y), pz: f32_to_u32(v.position.z),
            nx, ny, nz,
            c: v.color.unwrap_or(0),
            u, v: v_uv,
            tx, ty, tz,
            bx, by, bz,
            skinning: v.skinning_data.clone(),
        }
    }
}

pub struct MeshDeduplicator {
    vertices: Vec<HODVertex>,
    vertex_map: HashMap<HashableVertex, u16>,
}

impl MeshDeduplicator {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            vertex_map: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, v: &HODVertex) -> u16 {
        let hv = HashableVertex::from_hod(v);
        if let Some(&idx) = self.vertex_map.get(&hv) {
            return idx;
        }
        let idx = self.vertices.len() as u16;
        self.vertices.push(v.clone());
        self.vertex_map.insert(hv, idx);
        idx
    }

    pub fn get_vertices(&self) -> &[HODVertex] {
        &self.vertices
    }
}

use byteorder::{LittleEndian, BigEndian, WriteBytesExt};
use std::io::{Cursor, Write};

pub struct CompiledMesh {
    pub name: String,
    pub parent_name: String,
    pub deduplicated_vertices: Vec<HODVertex>,
    pub new_parts: Vec<HODMeshPart>,
}

fn write_len_string<W: Write>(writer: &mut W, s: &str) -> std::io::Result<()> {
    writer.write_u32::<LittleEndian>(s.len() as u32)?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}

fn get_vertex_stride(mask: u32, version: u32) -> u32 {
    let mut stride = 0;
    if (mask & 0x01) != 0 { stride += 16; }
    if (mask & 0x02) != 0 { stride += 16; }
    if (mask & 0x04) != 0 { stride += 4; }
    if (mask & 0x08) != 0 { stride += 8; }
    if version == 1401 {
        if (mask & 0x10) != 0 { stride += 8; }
        if (mask & 0x20) != 0 { stride += 8; }
    }
    if (mask & 0x2000) != 0 { stride += 12; }
    if (mask & 0x4000) != 0 { stride += 12; }
    stride.max(1)
}

fn serialize_vertices(vertices: &[HODVertex], mask: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    for v in vertices {
        if (mask & 0x01) != 0 {
            let _ = cursor.write_f32::<LittleEndian>(v.position.x);
            let _ = cursor.write_f32::<LittleEndian>(v.position.y);
            let _ = cursor.write_f32::<LittleEndian>(v.position.z);
            let _ = cursor.write_f32::<LittleEndian>(1.0); // w
        }
        if (mask & 0x02) != 0 {
            let n = v.normal.as_ref().unwrap_or(&Vector3 { x: 0.0, y: 0.0, z: 1.0 });
            let _ = cursor.write_f32::<LittleEndian>(n.x);
            let _ = cursor.write_f32::<LittleEndian>(n.y);
            let _ = cursor.write_f32::<LittleEndian>(n.z);
            let _ = cursor.write_f32::<LittleEndian>(0.0);
        }
        if (mask & 0x04) != 0 {
            let _ = cursor.write_u32::<LittleEndian>(v.color.unwrap_or(0xFFFFFFFF));
        }
        if (mask & 0x08) != 0 {
            let uv = v.uv.as_ref().unwrap_or(&Vector2 { u: 0.0, v: 0.0 });
            let _ = cursor.write_f32::<LittleEndian>(uv.u);
            let _ = cursor.write_f32::<LittleEndian>(uv.v);
        }
        if (mask & 0x2000) != 0 {
            let t = v.tangent.as_ref().unwrap_or(&Vector3 { x: 1.0, y: 0.0, z: 0.0 });
            let _ = cursor.write_f32::<LittleEndian>(t.x);
            let _ = cursor.write_f32::<LittleEndian>(t.y);
            let _ = cursor.write_f32::<LittleEndian>(t.z);
        }
        if (mask & 0x4000) != 0 {
            let b = v.binormal.as_ref().unwrap_or(&Vector3 { x: 0.0, y: 1.0, z: 0.0 });
            let _ = cursor.write_f32::<LittleEndian>(b.x);
            let _ = cursor.write_f32::<LittleEndian>(b.y);
            let _ = cursor.write_f32::<LittleEndian>(b.z);
        }
    }
    buf
}

pub fn compile_model_meshes(model: &HODModel) -> Vec<CompiledMesh> {
    let mut compiled_meshes = Vec::new();

    for mesh in &model.meshes {
        let mut dedup = MeshDeduplicator::new();
        let mut new_parts = Vec::new();

        for part in &mesh.parts {
            let mut new_indices = Vec::new();
            
            for &idx in &part.indices {
                if (idx as usize) < part.vertices.len() {
                    let v = &part.vertices[idx as usize];
                    let new_idx = dedup.add_vertex(v);
                    new_indices.push(new_idx);
                }
            }

            new_parts.push(HODMeshPart {
                material_index: part.material_index,
                vertex_mask: part.vertex_mask,
                vertices: Vec::new(),
                indices: new_indices,
            });
        }

        compiled_meshes.push(CompiledMesh {
            name: mesh.name.clone(),
            parent_name: mesh.parent_name.clone(),
            deduplicated_vertices: dedup.get_vertices().to_vec(),
            new_parts,
        });
    }

    compiled_meshes
}

pub fn generate_pool_data(compiled_meshes: &[CompiledMesh], original_textures: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decomp_mesh = Vec::new();
    let mut decomp_face = Vec::new();

    for mesh in compiled_meshes {
        for part in &mesh.new_parts {
            let v_data = serialize_vertices(&mesh.deduplicated_vertices, part.vertex_mask);
            decomp_mesh.extend_from_slice(&v_data);
            
            // Align face pool to 2 bytes
            if decomp_face.len() % 2 != 0 {
                decomp_face.push(0);
            }
            let mut face_cursor = Cursor::new(&mut decomp_face);
            face_cursor.set_position(face_cursor.get_ref().len() as u64);
            for &idx in &part.indices {
                face_cursor.write_u16::<LittleEndian>(idx)?;
            }
        }
    }

    let comp_tex = if original_textures.is_empty() { Vec::new() } else { crate::xpress::compress(original_textures) };
    let comp_mesh = if decomp_mesh.is_empty() { Vec::new() } else { crate::xpress::compress(&decomp_mesh) };
    let comp_face = if decomp_face.is_empty() { Vec::new() } else { crate::xpress::compress(&decomp_face) };

    let mut pool_buf = Vec::new();
    let mut cursor = Cursor::new(&mut pool_buf);
    
    cursor.write_u32::<LittleEndian>(0)?; // pool_type
    
    cursor.write_u32::<LittleEndian>(comp_tex.len() as u32)?;
    cursor.write_u32::<LittleEndian>(original_textures.len() as u32)?;
    cursor.write_all(&comp_tex)?;

    cursor.write_u32::<LittleEndian>(comp_mesh.len() as u32)?;
    cursor.write_u32::<LittleEndian>(decomp_mesh.len() as u32)?;
    cursor.write_all(&comp_mesh)?;

    cursor.write_u32::<LittleEndian>(comp_face.len() as u32)?;
    cursor.write_u32::<LittleEndian>(decomp_face.len() as u32)?;
    cursor.write_all(&comp_face)?;

    Ok(pool_buf)
}

pub fn generate_mult_chunks(compiled_meshes: &[CompiledMesh]) -> std::io::Result<Vec<IffChunk>> {
    let mut mult_chunks = Vec::new();
    
    for mesh in compiled_meshes {
        let mut mult_buf = Vec::new();
        let mut m_cursor = Cursor::new(&mut mult_buf);
        
        write_len_string(&mut m_cursor, &mesh.name)?;
        let parent_name_bytes = mesh.parent_name.as_bytes();
        let parent_len = parent_name_bytes.len() as u32;
        m_cursor.write_u32::<LittleEndian>(parent_len)?;
        m_cursor.write_all(parent_name_bytes)?;
        
        m_cursor.write_u32::<LittleEndian>(1)?; // lod_count
        
        // Write FORM TAGS "DoScars" (16 bytes payload)
        m_cursor.write_all(b"FORM")?;
        m_cursor.write_u32::<BigEndian>(16)?; // size
        m_cursor.write_all(b"TAGS")?;
        m_cursor.write_u32::<LittleEndian>(7)?; // string length
        m_cursor.write_all(b"DoScars")?;
        m_cursor.write_u8(0)?; // 1 byte padding to align to 2 bytes (15 is odd)

        for part in &mesh.new_parts {
            // Wrap BMSH in NRML chunk!
            m_cursor.write_all(b"NRML")?;
            m_cursor.write_u32::<BigEndian>(34)?; // size: 4 (BMSH) + 4 (version) + 26 (payload) = 34
            
            // Write BMSH header chunk manually!
            m_cursor.write_all(b"BMSH")?;
            m_cursor.write_u32::<LittleEndian>(1400)?; // version
            
            m_cursor.write_i32::<LittleEndian>(0)?; // lod
            m_cursor.write_i32::<LittleEndian>(1)?; // part count
            
            m_cursor.write_i32::<LittleEndian>(part.material_index as i32)?;
            m_cursor.write_u32::<LittleEndian>(part.vertex_mask)?;
            m_cursor.write_i32::<LittleEndian>(mesh.deduplicated_vertices.len() as i32)?;
            m_cursor.write_i16::<LittleEndian>(-1)?; // prim_group_count
            m_cursor.write_i32::<LittleEndian>(part.indices.len() as i32)?;
        }
        
        mult_chunks.push(IffChunk {
            id: "MULT".to_string(),
            chunk_type: crate::iff::ChunkType::Normal,
            version: 1400,
            data: mult_buf,
            children: Vec::new(),
        });
    }
    
    Ok(mult_chunks)
}
