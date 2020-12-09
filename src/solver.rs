use crate::sat::{SatResult, Cnf};

pub fn full_search(cnf: &Cnf) -> Vec<Vec<bool>> {
    let mut ret = Vec::new();
    let bitrange = 1 << (cnf.num_vars);
    for i in 0..bitrange {
        let v = bit_to_vec(i, cnf.num_vars);
        if let SatResult::Sat(answers) = cnf.solve(v) {
            ret.push(answers);
        }
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
