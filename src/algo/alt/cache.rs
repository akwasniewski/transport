use std::{collections::HashMap, fs::File, io::{BufReader, BufWriter, Read, Write}};
use ordered_float::OrderedFloat;
use crate::graph::{Graph, LandmarkData};
use crate::utility::{EdgeDir, IndexVec};
use rayon::join;

// ---------------------------------------------------------------------------
// Primitive encode / decode helpers
// ---------------------------------------------------------------------------

fn write_u32(w: &mut impl Write, v: u32) -> std::io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn write_u64(w: &mut impl Write, v: u64) -> std::io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
fn write_f32(w: &mut impl Write, v: f32) -> std::io::Result<()> {
    w.write_all(&v.to_le_bytes())
}

fn read_u32(r: &mut impl Read) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}
fn read_u64(r: &mut impl Read) -> std::io::Result<u64> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}
fn read_f32(r: &mut impl Read) -> std::io::Result<f32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(f32::from_le_bytes(buf))
}

// ---------------------------------------------------------------------------
// IndexVec<OrderedFloat<f32>> encode / decode
// ---------------------------------------------------------------------------

fn write_distance_vec(w: &mut impl Write, v: &IndexVec<OrderedFloat<f32>>) -> std::io::Result<()> {
    let slice: &[OrderedFloat<f32>] = v.as_ref();
    write_u64(w, slice.len() as u64)?;
    for &of in slice {
        write_f32(w, *of)?;
    }
    Ok(())
}

fn read_distance_vec(r: &mut impl Read) -> std::io::Result<IndexVec<OrderedFloat<f32>>> {
    let len = read_u64(r)? as usize;
    let mut vec: IndexVec<OrderedFloat<f32>> = IndexVec::with_capacity(len);
    for _ in 0..len {
        vec.push(OrderedFloat(read_f32(r)?));
    }
    Ok(vec)
}

// ---------------------------------------------------------------------------
// File format
//
//  [ magic: 8 bytes "LANDMARK" ]
//  [ version: u32 (1)          ]
//  [ landmark_count: u32       ]
//  for each landmark:
//    [ vertex_id: u32           ]
//    [ from vec (len + floats)  ]
//    [ to   vec (len + floats)  ]
// ---------------------------------------------------------------------------

const MAGIC: &[u8; 8] = b"LANDMARK";
const VERSION: u32 = 1;

impl Graph {
    /// Serialize landmarks to a `.bin` file. Panics on any I/O error.
    pub fn save_landmarks(&self, path: &str) {
        let file = File::create(path)
            .unwrap_or_else(|e| panic!("failed to create landmark file '{path}': {e}"));
        let mut w = BufWriter::new(file);

        w.write_all(MAGIC).unwrap();
        write_u32(&mut w, VERSION).unwrap();
        write_u32(&mut w, self.landmarks.len() as u32).unwrap();

        for (&vertex_id, data) in &self.landmarks {
            write_u32(&mut w, vertex_id).unwrap();
            write_distance_vec(&mut w, &data.from).unwrap();
            write_distance_vec(&mut w, &data.to).unwrap();
        }

        w.flush().unwrap();
    }

    /// Deserialize landmarks from a `.bin` file. Panics on any I/O or format error.
    /// Replaces any landmarks currently stored in the graph.
    pub fn load_landmarks(&mut self, path: &str) {
        let file = File::open(path)
            .unwrap_or_else(|e| panic!("failed to open landmark file '{path}': {e}"));
        let mut r = BufReader::new(file);

        let mut magic = [0u8; 8];
        r.read_exact(&mut magic).unwrap();
        assert_eq!(&magic, MAGIC, "invalid landmark file '{path}': bad magic bytes");

        let version = read_u32(&mut r).unwrap();
        assert_eq!(version, VERSION, "unsupported landmark file version {version} in '{path}'");

        let count = read_u32(&mut r).unwrap() as usize;
        let mut landmarks = HashMap::with_capacity(count);

        for _ in 0..count {
            let vertex_id = read_u32(&mut r).unwrap();
            let from = read_distance_vec(&mut r).unwrap();
            let to = read_distance_vec(&mut r).unwrap();
            landmarks.insert(vertex_id, LandmarkData { from, to });
        }

        self.landmarks = landmarks;
    }
}
