fn xor<'a, A, B>(i1: A, i2: B) -> Vec<u8>
    where A: Iterator<Item = &'a u8>,
          B: Iterator<Item = &'a u8> {
    let mut out: Vec<u8> = Vec::new();
    for (b1, b2) in i1.zip(i2) {
        out.push(b1 ^ b2)
    }
    out
}

pub fn vec_xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    xor(v1.iter(), v2.iter())
}

pub fn char_xor(v1: &Vec<u8>, char: u8) -> Vec<u8> {
    use std::iter::repeat;
    xor(v1.iter(), repeat(&char).into_iter())
}

pub fn repeating_key_xor(v1: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    xor(v1.iter(), key.iter().cycle())
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test() {

    }
}