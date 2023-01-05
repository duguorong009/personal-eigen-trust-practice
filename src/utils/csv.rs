pub fn write_to_csv_file(
    path: &str,
    m: usize,
    data: &Vec<Vec<u8>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Creates new `Writer` for `stdout`
    let mut writer = csv::Writer::from_path(path)?;

    // Write records one at a time including header record.
    for i in 0..m {
        writer.write_record(
            &data[i]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        )?;
    }
    writer.flush()?;
    Ok(())
}
