const PLING: &str = "Pling";
const PLANG: &str = "Plang";
const PLONG: &str = "Plong";

pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    if n % 3 == 0 {
        res.push_str(PLING);
    }

    if n % 5 == 0 {
        res.push_str(PLANG);
    }

    if n % 7 == 0 {
        res.push_str(PLONG);
    }

    if res.is_empty() {
        res = n.to_string();
    }

    res
}
