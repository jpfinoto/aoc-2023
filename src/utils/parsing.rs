pub fn get_numbers(s: &str) -> Vec<u32> {
    s.split(" ").flat_map(|s| u32::from_str_radix(s, 10)).collect()
}

pub fn get_big_numbers(s: &str) -> Vec<u64> {
    s.split(" ").flat_map(|s| u64::from_str_radix(s, 10)).collect()
}

pub fn get_big_signed_numbers(s: &str) -> Vec<i64> {
    s.split(" ").flat_map(|s| i64::from_str_radix(s, 10)).collect()
}
