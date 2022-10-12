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
    pub vertex_buff: Vec<Vertex>,
}
#[derive(BinRead, Copy, Clone)]
#[br(import(normals: bool))]
pub struct Vertex {
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

impl MDLFile {
    pub fn open<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        BufReader::new(std::fs::File::open(path)?).read_le()
    }
}
