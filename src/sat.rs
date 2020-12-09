#[derive(Debug)]
pub enum SatResult {
    Sat(Vec<bool>),
    UnSat,
}

impl std::fmt::Display for SatResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SatResult::Sat(vec) => {
                let len = vec.len();
                let mut buf = String::new();
                for (i, ans) in vec.iter().enumerate() {
                    if *ans {
                        buf.push_str(&format!("{}", i + 1));
                    } else {
                        buf.push_str(&format!("-{}", i + 1));
                    }
                    if i < len {
                        buf.push_str(", ");
                    }
                }
                write!(f, "{}", buf)
            }
            SatResult::UnSat => write!(f, "UnSat"),
        }
    }
}

#[derive(Debug)]
pub struct Cnf {
    pub num_vars: u64,
    pub clauses: Vec<dimacs::Clause>,
}

#[derive(Debug)]
pub enum CnfError {
    NotCnf,
}

impl std::fmt::Display for CnfError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CnfError::NotCnf => write!(f, "The Instance is not Cnf"),
        }
    }
}

impl Cnf {
    pub fn from(instance: dimacs::Instance) -> Result<Self, CnfError> {
        match instance {
            dimacs::Instance::Cnf {
                num_vars: n,
                clauses: c,
            } => Ok(Cnf {
                num_vars: n,
                clauses: Vec::from(c),
            }),
            dimacs::Instance::Sat {
                num_vars: _,
                extensions: _,
                formula: _,
            } => Err(CnfError::NotCnf),
        }
    }

    pub fn solve(&self, answers: Vec<bool>) -> SatResult {
        for clause in &self.clauses {
            let mut flag = false;
            for lit in clause.lits() {
                let var = lit.var().to_u64() as usize - 1;
                match lit.sign() {
                    dimacs::Sign::Pos => {
                        if answers[var] {
                            flag = true;
                        }
                    },
                    dimacs::Sign::Neg => {
                        if !answers[var] {
                            flag = true;
                        }
                    }
                }
            }
            if !flag {
                return SatResult::UnSat;
            }
        }
        SatResult::Sat(answers)
    }
}
