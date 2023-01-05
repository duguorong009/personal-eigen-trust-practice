use nalgebra::{ArrayStorage, Matrix, SMatrix, U10};

mod utils;
use crate::utils::csv::{read_from_csv_file, write_to_csv_file};
use crate::utils::random::gen_random_downloads_data;

const M: usize = 10; // count of peers
const SAT_DATA_FILE: &str = "sat_downloads.csv"; // file path of satisfactory downloads counts data
const UNSAT_DATA_FILE: &str = "unsat_downloads.csv"; // file path of unsatisfactory downloads counts data

fn main() {
    // generate_sim_data();

    // Read downloads data for eigentrust computation
    let sat_data = read_from_csv_file(SAT_DATA_FILE).unwrap();
    let unsat_data = read_from_csv_file(UNSAT_DATA_FILE).unwrap();

    // Convert the downloads data to matrix for operations
    let sat_mat: SMatrix<f64, M, M> =
        SMatrix::from_vec(sat_data.concat().into_iter().map(|v| v as f64).collect());
    let unsat_mat: SMatrix<f64, M, M> =
        SMatrix::from_vec(unsat_data.concat().into_iter().map(|v| v as f64).collect());

    // local trust values : s(i, j) = sat(i, j) - unsat(i, j)
    let s: SMatrix<f64, M, M> = sat_mat - unsat_mat;

    // normalized local trust values : c(i, j)
    let c: SMatrix<f64, M, M> = custom_normalize(s);

    basic_eigentrust_algo(c);
}

/// Generate random data for simulation & save to files for future use
fn generate_sim_data() {
    let sat_downloads = gen_random_downloads_data(M);
    write_to_csv_file(&SAT_DATA_FILE, M, &sat_downloads).unwrap();

    let unsat_downloads = gen_random_downloads_data(M);
    write_to_csv_file(&UNSAT_DATA_FILE, M, &unsat_downloads).unwrap();
}

/// Simple centralized trust value computing algorithm (basic EigenTrust algorithm)
fn basic_eigentrust_algo(c: SMatrix<f64, M, M>) {
    let mut t_i = c.clone(); // t_0 = c_0
    let mut t_i_next: Matrix<f64, U10, U10, ArrayStorage<f64, M, M>> = Matrix::default();

    let mut n = 0;
    let mut sig = f64::MAX;
    let err = 0.05;

    while sig > err {
        c.tr_mul_to(&t_i, &mut t_i_next);

        sig = (t_i_next - t_i).norm();

        t_i = t_i_next;

        n += 1;
    }

    println!("t_i: {t_i:?}");
    println!("after {n} iterations!");
}

/// Normalize the matrix of local trust values(`s_i_j`) by using the
/// formula from eigen-trust algorithm.
///
/// Ref: https://en.wikipedia.org/wiki/EigenTrust
fn custom_normalize(s: SMatrix<f64, M, M>) -> SMatrix<f64, M, M> {
    let mut res: Vec<Vec<f64>> = vec![vec![0.0; M]; M];

    for r in 0..M {
        let sum = s.row(r).iter().map(|v| v.max(0.0)).sum::<f64>();
        for c in 0..M {
            res[r][c] = if sum == 0.0 {
                1.0 / M as f64 // TODO: Should be the trust value of initial trusted peers - p_i
            } else {
                s[(r, c)].max(0.0) / sum
            }
        }
    }

    SMatrix::from_vec(res.concat())
}
