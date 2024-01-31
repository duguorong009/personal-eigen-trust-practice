use nalgebra::{ArrayStorage, Matrix, SMatrix, U1, U10};

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

    let peer = 1;
    let a = 0.4; // constant less than 1
    let p: Matrix<f64, U10, U1, ArrayStorage<f64, M, 1>> =
        SMatrix::from_vec(vec![0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0]);

    simple_non_distributed_eigen_trust(c, peer);

    basic_eigen_trust(c, peer, a, p);
}

/// Simple centralized trust value computing algorithm (Algorithm 1. Simple non-distributed EigenTrust algorithm)
fn simple_non_distributed_eigen_trust(c: SMatrix<f64, M, M>, peer: usize) {
    println!("Simple non-distributed EigenTrust algorithm");

    let mut t_i = c.row(peer).transpose(); // t_0 = c_0
    let mut t_i_next: Matrix<f64, U10, U1, ArrayStorage<f64, M, 1>> = Matrix::default();

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

/// Basice EigenTrust algorithm (Algorithm 2.)
fn basic_eigen_trust(c: SMatrix<f64, M, M>, _peer: usize, a: f64, p: SMatrix<f64, M, 1>) {
    println!("Basice EigenTrust algorithm");

    let mut t_i = p; // t_0 = p
    let mut t_i_next: Matrix<f64, U10, U1, ArrayStorage<f64, M, 1>> = Matrix::default();

    let mut n = 0;
    let mut sig = f64::MAX;
    let err = 0.05;

    while sig > err {
        c.tr_mul_to(&t_i, &mut t_i_next);

        t_i_next = (1.0 - a) * t_i_next + a * p;

        sig = (t_i_next - t_i).norm();

        t_i = t_i_next;

        n += 1;
    }

    println!("t_i: {t_i:?}");
    println!("after {n} iterations!");
}

/// Generate random data for simulation & save to files for future use
#[allow(unused)]
fn generate_sim_data() {
    let sat_downloads = gen_random_downloads_data(M);
    write_to_csv_file(SAT_DATA_FILE, M, &sat_downloads).unwrap();

    let unsat_downloads = gen_random_downloads_data(M);
    write_to_csv_file(UNSAT_DATA_FILE, M, &unsat_downloads).unwrap();
}

/// Normalize the matrix of local trust values s(i, j) by using the
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
