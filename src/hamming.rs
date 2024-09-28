const M1: u64  = 0x5555555555555555; //binary: 0101...
const M2: u64  = 0x3333333333333333; //binary: 00110011..
const M4: u64  = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
const M8: u64  = 0x00ff00ff00ff00ff; //binary:  8 zeros,  8 ones ...
const M16: u64 = 0x0000ffff0000ffff; //binary: 16 zeros, 16 ones ...
const M32: u64 = 0x00000000ffffffff; //binary: 32 zeros, 32 ones
const HFF: u64  = 0xffffffffffffffff; //binary: all oes
const H01: u64 = 0x0101010101010101; //the sum of 256 to the power of 0,1,2,3...

use std::convert::TryFrom;

fn convert(v: u64) -> i32 {
    let mut val = 0;
    let ret = i32::try_from(v);
    if ret.is_ok() {
        val = ret.unwrap();
    }
    
    val
}
pub fn hamming_distance(left: u64, right: u64) -> i32 {
	let mut x = left ^ right;
	x -= (x >> 1) & M1;             //put count of each 2 bits into those 2 bits
	x = (x & M2) + ((x >> 2) & M2); //put count of each 4 bits into those 4 bits
	x = (x + (x >> 4)) & M4;        //put count of each 8 bits into those 8 bits
	return convert((x * H01) >> 56);    //returns left 8 bits of x + (x<<8) + (x<<16) + (x<<24) + ...
}