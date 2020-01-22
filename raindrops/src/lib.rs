const PLING: &'static str = "Pling";
const PLANG: &'static str = "Plang";
const PLONG: &'static str = "Plong";

pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    if n % 3 == 0 {
        res += PLING;
    }

    if n % 5 == 0 {
        res += PLANG;
    }

    if n % 7 == 0 {
        res += PLONG;
    }

    if res.is_empty() {
        res = n.to_string();
    }

    res
}
