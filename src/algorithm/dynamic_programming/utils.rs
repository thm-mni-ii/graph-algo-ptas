use bitvec::vec::BitVec;
use fxhash::FxHashSet;
use itertools::Itertools;

pub fn bit_vec_powerset(set: &FxHashSet<usize>, subset_size: usize) -> Vec<BitVec> {
    set.iter()
        .powerset()
        .map(|subset| to_bit_vec(subset.iter().copied(), subset_size))
        .collect()
}

pub fn init_bit_vec(size: usize) -> BitVec {
    let n = (size as f64 / 64.0).ceil() as usize;

    BitVec::from_vec(vec![0; n])
}

pub fn to_bit_vec<'a>(it: impl Iterator<Item = &'a usize>, size: usize) -> BitVec {
    let mut bit_vec: BitVec = init_bit_vec(size);

    for x in it {
        bit_vec.set(*x, true);
    }

    bit_vec
}

pub fn immutable_bit_vec_update(subset: &BitVec, v: usize) -> BitVec {
    let mut subset = subset.clone();
    subset.set(v, true);
    subset
}
