use matfile;
use std::fs::File;
use std::io::BufReader;

pub fn load_mat(
    file_name: &str,
    var_name: &str,
) -> Result<(Vec<f64>, Vec<usize>), Box<dyn std::error::Error>> {
    let file = File::open(file_name)?;
    let reader = BufReader::with_capacity(100_000_000, file);
    let mat_file = matfile::MatFile::parse(reader)?;
    let array = mat_file
        .find_by_name(var_name)
        .ok_or::<String>(format!("Variable {} not found in {}", var_name, file_name))?;
    match array.data() {
        matfile::NumericData::Double { real, imag: _ } => Ok((real.clone(), array.size().clone())),
        _ => Err(From::from(format!("No real data found in {}", var_name))),
    }
}
