#[allow(dead_code)]
pub fn to_vec(arr: &[&str]) -> Vec<String> {
    arr.iter().map(|x| x.to_string()).collect()
}
