#![allow(dead_code)]
pub fn is_pow2(x: usize) -> bool { x != 0 && (x & (x - 1)) == 0 }
pub fn align_up(value: usize, alignment: usize) -> usize {
    assert!(is_pow2(alignment));
    (value + (alignment - 1)) & !(alignment - 1)
}
pub fn align_down(value: usize, alignment: usize) -> usize {
    assert!(is_pow2(alignment));
    value & !(alignment - 1)
}
