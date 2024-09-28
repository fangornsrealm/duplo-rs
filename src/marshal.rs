use std::io::{Read, Write};

pub fn store_bool(val: bool, to: &mut Vec<u8>) {
    let u = val as u8;
    let bytes = u.to_be_bytes();
    match to.write(&bytes[..]) {
        Err(why) => panic!("couldn't write bool: {}", why),
        Ok(retval) => retval,
    };
}

pub fn restore_bool(from: &mut std::io::Cursor<Vec<u8>>) -> bool {
    let mut bytes = [0; 1];
    match from.read_exact(&mut bytes) {
        Err(why) => panic!("couldn't read bool: {}", why),
        Ok(retval) => retval,
    };
    let u = u8::from_be_bytes(bytes);
    let val: bool;
    if u == 0 {
        val = false;
    } else {
        val = true;
    }
    val
}

pub fn store_usize(val: usize, to: &mut Vec<u8>) {
    let bytes = &val.to_be_bytes();
    match to.write(&bytes[..]) {
        Err(why) => panic!("couldn't write usize: {}", why),
        Ok(retval) => retval,
    };
}

pub fn restore_usize(from: &mut std::io::Cursor<Vec<u8>>) -> usize {
    let mut bytes = [0; 8];
    match from.read_exact(&mut bytes) {
        Err(why) => panic!("couldn't read usize: {}", why),
        Ok(retval) => retval,
    };
    let val = usize::from_be_bytes(bytes);
    val
}

pub fn store_u32(val: u32, to: &mut Vec<u8>) {
    let bytes = &val.to_be_bytes();
    match to.write(&bytes[..]) {
        Err(why) => panic!("couldn't write u32: {}", why),
        Ok(retval) => retval,
    };
}

pub fn restore_u32(from: &mut std::io::Cursor<Vec<u8>>) -> u32 {
    let mut bytes = [0; 4];
    match from.read_exact(&mut bytes) {
        Err(why) => panic!("couldn't read u32: {}", why),
        Ok(retval) => retval,
    };
    let val = u32::from_be_bytes(bytes);
    val
}

pub fn store_i32(val: i32, to: &mut Vec<u8>) {
    let bytes = &val.to_be_bytes();
    match to.write(&bytes[..]) {
        Err(why) => panic!("couldn't write i32: {}", why),
        Ok(retval) => retval,
    };
}

pub fn restore_i32(from: &mut std::io::Cursor<Vec<u8>>) -> i32 {
    let mut bytes = [0; 4];
    match from.read_exact(&mut bytes) {
        Err(why) => panic!("couldn't read u32: {}", why),
        Ok(retval) => retval,
    };
    let val = i32::from_be_bytes(bytes);
    val
}

pub fn store_u64(val: u64, to: &mut Vec<u8>) {
    let bytes = &val.to_be_bytes();
    match to.write(&bytes[..]) {
        Err(why) => panic!("couldn't write u64: {}", why),
        Ok(retval) => retval,
    };
}

pub fn restore_u64(from: &mut std::io::Cursor<Vec<u8>>) -> u64 {
    let mut bytes = [0; 8];
    match from.read_exact(&mut bytes) {
        Err(why) => panic!("couldn't read u64: {}", why),
        Ok(retval) => retval,
    };
    let val = u64::from_be_bytes(bytes);
    val
}

pub fn store_f32(val: f32, to: &mut Vec<u8>) {
    let bytes = &val.to_be_bytes();
    match to.write(&bytes[..]) {
        Err(why) => panic!("couldn't write f32: {}", why),
        Ok(retval) => retval,
    };
}

pub fn restore_f32(from: &mut std::io::Cursor<Vec<u8>>) -> f32 {
    let mut bytes = [0; 4];
    match from.read_exact(&mut bytes) {
        Err(why) => panic!("couldn't read f32: {}", why),
        Ok(retval) => retval,
    };
    let val = f32::from_be_bytes(bytes);
    val
}

pub fn store_f64(val: f64, to: &mut Vec<u8>) {
    let bytes = &val.to_be_bytes();
    match to.write(&bytes[..]) {
        Err(why) => panic!("couldn't write f64: {}", why),
        Ok(retval) => retval,
    };
}

pub fn restore_f64(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    let mut bytes = [0; 8];
    match from.read_exact(&mut bytes) {
        Err(why) => panic!("couldn't read f64: {}", why),
        Ok(retval) => retval,
    };
    let val = f64::from_be_bytes(bytes);
    val
}


pub fn store_char(val: char, to: &mut Vec<u8>) {
    let i = val as u32;
    let bytes = &i.to_be_bytes();
    match to.write(&bytes[..]) {
        Err(why) => panic!("couldn't write char: {}", why),
        Ok(retval) => retval,
    };
}

pub fn restore_char(from: &mut std::io::Cursor<Vec<u8>>) -> char {
    let mut bytes = [0; 4];
    match from.read_exact(&mut bytes) {
        Err(why) => panic!("couldn't read char: {}", why),
        Ok(retval) => retval,
    };
    let i = u32::from_be_bytes(bytes);

    let val = char::from_u32(i);
    if val.is_none() {
        return ' ';
    }
    val.unwrap()
}

pub fn store_string(val: &String, to: &mut Vec<u8>) {
    let s = val.len();
    store_usize(s, to);
    for c in val.chars() {
        store_char(c, to);
    }
}

pub fn restore_string(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let mut val = String::new();
    let s = restore_usize(from);
    for _i in 0..s {
        let c = restore_char(from);
        val.push(c);
    }
    val
}

pub fn store_vec_u32(val: &Vec<u32>, to: &mut Vec<u8>) {
    let s = val.len();
    store_usize(s, to);
    for elem in val {
        store_u32(*elem, to);
    }
}

pub fn restore_vec_u32(from: &mut std::io::Cursor<Vec<u8>>) -> Vec<u32> {
    let mut val = Vec::new();
    let s = restore_usize(from);
    for _i in 0..s {
        let elem = restore_u32(from);
        val.push(elem);
    }
    val
}

pub fn store_vec_u64(val: &Vec<u64>, to: &mut Vec<u8>) {
    let s = val.len();
    store_usize(s, to);
    for elem in val {
        store_u64(*elem, to);
    }
}

pub fn restore_vec_u64(from: &mut std::io::Cursor<Vec<u8>>) -> Vec<u64> {
    let mut val = Vec::new();
    let s = restore_usize(from);
    for _i in 0..s {
        let elem = restore_u64(from);
        val.push(elem);
    }
    val
}

pub fn store_vec_f32(val: &Vec<f32>, to: &mut Vec<u8>) {
    let s = val.len();
    store_usize(s, to);
    for elem in val {
        store_f32(*elem, to);
    }
}

pub fn restore_vec_f32(from: &mut std::io::Cursor<Vec<u8>>) -> Vec<f32> {
    let mut val = Vec::new();
    let s = restore_usize(from);
    for _i in 0..s {
        let elem = restore_f32(from);
        val.push(elem);
    }
    val
}

pub fn store_vec_f64(val: &Vec<f64>, to: &mut Vec<u8>) {
    let s = val.len();
    store_usize(s, to);
    for elem in val {
        store_f64(*elem, to);
    }
}

pub fn restore_vec_f64(from: &mut std::io::Cursor<Vec<u8>>) -> Vec<f64> {
    let mut val = Vec::new();
    let s = restore_usize(from);
    for _i in 0..s {
        let elem = restore_f64(from);
        val.push(elem);
    }
    val
}

pub fn store_hash_string_usize(val: &std::collections::BTreeMap<String, usize>, to: &mut Vec<u8>) {
    let t = val.len();
    store_usize(t, to);
    for (key, value) in val {
        store_string(&key, to);
        store_usize(*value, to);
    }
}

pub fn restore_hash_string_usize(from: &mut std::io::Cursor<Vec<u8>>) -> std::collections::BTreeMap<String, usize> {
    let mut val = std::collections::BTreeMap::new();
    let t = restore_usize(from);
    for _i in 0..t {
        let key = restore_string(from);
        let value = restore_usize(from);
        val.insert(key, value);
    }
    val
}
