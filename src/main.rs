#[macro_use]
extern crate clap;
mod sat;
mod solver;

fn main() {
    let app = clap_app!(sssr =>
        (version:   crate_version!())
        (author:    crate_authors!())
        (about:     crate_description!())
        (@arg file: +required "cnf file")
    )
    .get_matches();

    let cnf = std::fs::read_to_string(app.value_of("file").unwrap()).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    match dimacs::parse_dimacs(&cnf) {
        Ok(instance) => match sat::Cnf::from(instance) {
            Ok(cnf) => {
                let answers = solver::full_search(&cnf);
                if answers.iter().all(|r| !r.is_satisfiable()) {
                    println!("This is unsatisfiable");
                }
                for answer in answers {
                    if answer.is_satisfiable() {
                        println!("This is satisfiable when {}", answer);
                    }
                }
            }
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{:#?}", e),
    }
}
