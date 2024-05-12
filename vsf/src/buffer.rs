use bitvec::{array::BitArray, order::Msb0};

#[derive(Clone)]
struct Buffer(Vec<BitArray<u8, Msb0>>);


impl Buffer {
    pub fn to_byte_vec(&self) -> Vec<u8>{
        let _vec =  self.0.clone();
        _vec.iter().map(|bitarr|{
            bitarr.data
        }).collect()
    }
}