/// Check if a string is a valid semver version
/// A valid semver version is in the format x.y.z
pub fn is_valid_semver<S: Into<String>>(semver: S) -> bool {
    let re = regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    re.is_match(semver.into().as_str())
}
