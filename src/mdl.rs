use binrw::{
    BinRead, // trait for reading
    BinReaderExt,
    BinResult,
};
use modular_bitfield::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(BinRead, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
#[derive(BinRead, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(BinRead, Copy, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[bitfield]
#[derive(BinRead, Debug, PartialEq)]
#[br(map = Self::from_bytes)]
pub struct Flags {
    pub norm: bool,
    pub txtr: bool,
    pub clrs: bool,
    pub unk0: bool,
    pub unk1: bool,
    pub unk2: bool,
    pub unk3: bool,
    pub unk4: bool,
}
#[derive(BinRead, Clone)]
#[br(import(vertex_count: u16,normals: bool))]
pub struct Frame {
    #[br(args{count: vertex_count as usize, inner:(normals,)})]
    pub vertex_buff: Vec<VertexInternal>,
}
#[derive(BinRead, Copy, Clone)]
#[br(import(normals: bool))]
pub struct VertexInternal {
    pub position: Vec3,
    #[br(if(normals))]
    pub normals: Option<Vec3>,
}

#[derive(BinRead)]
#[br(little, magic = b"MDL\x00")]
pub struct MDLFile {
    pub flags: Flags,
    pub idx_per_face: u8,

    pub vertex_count: u16,
    pub frame_count: u16,

    #[br(if(flags.txtr()),count = vertex_count)]
    pub texture_cords: Option<Vec<Vec2>>,

    #[br(if(flags.clrs()),count = vertex_count)]
    pub colors: Option<Vec<RGBA>>,

    pub index_count: u16,
    #[br(count = index_count)]
    pub indexs: Vec<u16>,

    #[br(count = frame_count,args{inner:(vertex_count,flags.norm(),)})]
    pub frames: Vec<Frame>,
}

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FrameVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}
impl Vertex for FrameVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<FrameVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    pub color: [u8; 4],
}

impl Vertex for ColorVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ColorVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Unorm8x4,
                    },
                ],
        }
    }
}
pub struct Mesh {
    pub name: String,
    pub fixed_buffer: wgpu::Buffer,
    pub frame_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub num_frames: u32,
    pub frame_stride: u32,
}


impl MDLFile {
    pub fn open<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        BufReader::new(std::fs::File::open(path)?).read_le()
    }
    pub fn read<P: AsRef<Path>>(path: P) -> Mesh<Self>{
        let mdl_file = Self::open(path);
        match mdl_file {
            Err(error) => {
                println!("Failed to Load")
            },
            Ok(model)=>{
                println!("Successful to Load {}",&out_file.display())
            },
        }

    }
}
