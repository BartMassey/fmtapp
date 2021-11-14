pub fn app_fmt(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

pub fn app_clone(s1: &str, s2: &str) -> String {
    s1.to_owned() + s2
}
