pub fn write_to_csv_file(
    path: &str,
    m: usize,
    data: &Vec<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    print!("Writing to file: {path}");

    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_path(path)?;

    // Write (empty) header record for convenience
    writer.write_record(&vec![""; m])?;

    // Write records one at a time.
    for i in 0..m {
        writer.write_record(
            &data[i]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        )?;
    }
    writer.flush()?;

    println!(" Completed!");

    Ok(())
}

/// Reads data(m * m matrix) from a file into a read and return all records
///
/// # Error
///
/// If an error occurs, the error is returned to caller.
pub fn read_from_csv_file(path: &str) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    print!("Reading from file: {path}");

    let mut data: Vec<Vec<u8>> = vec![];

    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // // Retrieve and parse records
    // let headers = reader.headers()?;
    // println!("{:?}", headers);

    // `.records` return an iterator of the internal
    for result in reader.records() {
        let record = result?;
        // println!("{:?}", record);

        let parsed = record
            .into_iter()
            .map(|c| u8::from_str_radix(c, 10).unwrap())
            .collect::<Vec<u8>>();
        data.push(parsed);
    }

    println!(" Completed!");

    Ok(data)
}
