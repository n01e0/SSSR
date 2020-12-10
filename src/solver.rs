use crate::sat::{Cnf, SatResult};

pub fn full_search(cnf: &Cnf) -> Vec<SatResult> {
    let mut ret = Vec::new();
    let bitrange = 1 << (cnf.num_vars);
    for i in 0..bitrange {
        let v = bit_to_vec(i, cnf.num_vars);
        ret.push(cnf.solve(v));
    }

    return ret;
}

fn bit_to_vec(bit: u64, range: u64) -> Vec<bool> {
    let bit = bit;
    let mut ret = Vec::new();
    for _ in 0..range {
        ret.push(false);
    }
    for i in 0..range {
        ret[i as usize] = bit >> i & 1 == 1;
    }
    return ret;
}
