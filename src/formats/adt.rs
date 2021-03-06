use crate::formats::chunk::*;
use serde::{Deserialize, Serialize};
use crate::common::R;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdtFile {
    pub mver: ChunkMver,
    pub mhdr: ChunkMhdr,
    pub mcin: Vec<ChunkMcin>,
    pub mtex: ChunkMtex,
    pub mmdx: ChunkMmdx,
    pub mmid: ChunkMmid,
    pub mwmo: ChunkMwmo,
    pub mwid: ChunkMwid,
    pub mddf: ChunkMddf,
    pub mcnk: ChunkMcnk,
}

impl AdtFile {
    pub fn from_path<P: AsRef<Path>>(path: P) -> R<AdtFile> {
        let chunks = Chunk::from_path(path)?;
        AdtFile::new(chunks)
    }

    fn new(chunks: Vec<Chunk>) -> R<AdtFile> {
        let mver = chunks.get_mver_chunk()?;
        let mhdr = chunks.get_mhdr();
        let mcin = chunks.get_mcin();
        let mtex = chunks.get_mtex();
        let mmdx = chunks.get_mmdx();
        let mmid = chunks.get_mmid();
        let mwmo = chunks.get_mwmo();
        let mwid = chunks.get_mwid();
        let mddf = chunks.get_mddf();
        let mcnk = chunks.get_mcnk();
        Ok(AdtFile {
            mver,
            mhdr,
            mcin,
            mtex,
            mmdx,
            mmid,
            mwmo,
            mwid,
            mddf,
            mcnk
        })
    }
}