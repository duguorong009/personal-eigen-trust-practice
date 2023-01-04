use rand::Rng;

fn main() {
    println!("Hello, world!");

    let m = 10; // count of peers
    let sat_data = rng_tr_data(m);
    let unsat_data = rng_tr_data(m);

    // local trust values
    let s = mat_sub(sat_data, unsat_data);

    // normalized local trust values
    let c = normalize(s);

    // inversed
    let c_t = mat_inverse(c.clone());

    // Get the converged c_t value
    let converged_c_t = converge(c_t);

    // Final t vector
    let t = mat_mul(converged_c_t, c);

    println!("Global trust values:: {t:?}");
}

fn rng_tr_data(m: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut sat: Vec<Vec<u8>> = vec![];

    for _ in 0..m {
        let mut sat_i: Vec<u8> = vec![];
        for _ in 0..m {
            sat_i.push(rng.gen::<u8>());
        }
        sat.push(sat_i);
    }

    sat
}

fn mat_sub(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<Vec<i16>> {
    let m = a.len();

    let mut res: Vec<Vec<i16>> = vec![vec![0; m]; m];

    for i in 0..m {
        for j in 0..m {
            res[i][j] = a[i][j] as i16 - b[i][j] as i16;
        }
    }

    res
}

fn normalize(s: Vec<Vec<i16>>) -> Vec<Vec<f64>> {
    let m = s.len();
    let mut res: Vec<Vec<f64>> = vec![vec![0.0; m]; m];

    for i in 0..m {
        for j in 0..m {
            let sum = s[i].iter().map(|s_i_j| s_i_j.max(&0)).sum::<i16>();
            res[i][j] = if sum == 0 {
                1.0 / m as f64 // TODO: should be the trust value of initial trusted peers - p_i
            } else {
                s[i][j].max(0) as f64 / sum as f64
            };
        }
    }

    res
}

fn mat_inverse(mat: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m = mat.len();
    let mut inversed = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            inversed[i][j] = mat[j][i];
        }
    }
    inversed
}

fn mat_mul(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let m = a.len();

    let mut c = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            c[i][j] += a[i][j] * b[j][i];
        }
    }

    c
}

fn converge(c_t: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut res = c_t.clone();
    for _ in 0..10 {
        res = mat_mul(res, c_t.clone());
    }

    res
}
