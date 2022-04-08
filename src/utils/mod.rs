
use bitvec::prelude::*;
use bitvec::store::BitStore;

pub fn read_ubitvar<T: BitStore>(bv: &mut BitVec<T, Lsb0>) -> u32 {
    // read 6 bits; 4 first bits of value, followed by 2 encoding remainder length
    let h = &bv[..6].load::<u32>();

    // determine total number of bits based on length encoding
    let nbits: usize = match h & 0b110000 {
        16 => 10,
        32 => 14,
        48 => 34,
        _  => 6,
    };

    // drain total bits from buffer, add remaining bits as high bits to return
    let all_bits: BitVec = bv.drain(..nbits).collect();
    let remainder = all_bits.load::<u32>();

    // use leading 4 bits + trailing bits
    h & 0b001111 | ((remainder >> 6) << 4)
}

pub fn read_varuint32<T: BitStore>(bv: &mut BitVec<T, Lsb0>) -> u32 {
    let mut result = 0;
    let mut bits: u32 = 0;

    loop {
        let next_bits: BitVec = bv.drain(..8).collect();
        let next_byte = next_bits.load::<u32>();
        result |= (next_byte & 0b01111111) << bits;
        bits += 7;
        if ((next_byte & 0b10000000) == 0) || (bits == 35) { break }
    }

    result
}
