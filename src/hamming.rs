const M1: u64  = 0x5555555555555555; //binary: 0101...
const M2: u64  = 0x3333333333333333; //binary: 00110011..
const M4: u64  = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
const M8: u64  = 0x00ff00ff00ff00ff; //binary:  8 zeros,  8 ones ...
const M16: u64 = 0x0000ffff0000ffff; //binary: 16 zeros, 16 ones ...
const M32: u64 = 0x00000000ffffffff; //binary: 32 zeros, 32 ones
const HFF: u64  = 0xffffffffffffffff; //binary: all ones
const H01: u64 = 0x0101010101010101; //the sum of 256 to the power of 0,1,2,3...
const H32: u32 = 0x01010101; //the sum of 256 to the power of 0,1,2,3...

use std::convert::TryFrom;

fn convert(v: u64) -> i64 {
    let mut val = 0;
    let ret = i64::try_from(v);
    if ret.is_ok() {
        val = ret.unwrap();
    }
    
    v as i64
}

pub fn hamming_distance(left: u64, right: u64) -> i64 {
    use std::num::Wrapping;
	let mut x = left ^ right;
	x -= (x >> 1) & M1;             //put count of each 2 bits into those 2 bits
	x = (x & M2) + ((x >> 2) & M2); //put count of each 4 bits into those 4 bits
	x = (x + (x >> 4)) & M4;        //put count of each 8 bits into those 8 bits
    let vx = Vec::from(x.to_le_bytes());
    let (vlow, vhigh) = vx.split_at(4);
    let xlow = u32::from_le_bytes(vlow.try_into().unwrap());
    let xhigh = u32::from_le_bytes(vhigh.try_into().unwrap());
    let xhlow = xlow as u64 * H32 as u64;
    let xhhigh = xhigh as u64 * H32 as u64;
    let xhwlow = Wrapping(xhlow);
    let xhwhigh = Wrapping(xhhigh);
    let shifted_xhwlow = xhwlow >> 56;
    let shifted_xhwhigh = xhwhigh >> 24;
    let unsigned = shifted_xhwlow + shifted_xhwhigh;
    unsigned.0 as i64
    /*
    //let val32: i32 = (x8 * h8) as i32;c
    let xw = Wrapping(x);
    let hw = Wrapping(H01);
    let xws = xw >> 56 ;
    let hws = hw >> 56;
    let xwhw56 = (xw * hw) >> 56;
    let xwhw48 = (xw * hw) >> 48;
    let xwhw32 = (xw * hw) >> 32;
    let xwhw16 = (xw * hw) >> 16;
	//return convert(xwhw.0 >> 56);    //returns left 8 bits of x + (x<<8) + (x<<16) + (x<<24) + ...
    (xwhw56.0 * 10) as i64
    */
}

pub fn hamming_distance_ori(left: u64, right: u64) -> i64 {
    let val64 = hamming_rs::distance(&left.to_le_bytes(), &right.to_le_bytes());
    val64 as i64
}