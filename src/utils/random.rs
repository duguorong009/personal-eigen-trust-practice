use rand::Rng;

pub fn gen_random_downloads_data(m: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut sat: Vec<Vec<u8>> = vec![];

    for i in 0..m {
        let mut sat_i: Vec<u8> = vec![];
        for j in 0..m {
            if i != j {
                sat_i.push(rng.gen::<u8>());
            } else {
                sat_i.push(0);
            }
        }
        sat.push(sat_i);
    }

    sat
}
