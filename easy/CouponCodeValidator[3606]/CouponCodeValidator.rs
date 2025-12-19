// i didn't come up with this Solution 
//copied from the top soution
//
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coupon {
    business_line: BusinessLine,
    code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BusinessLine {
    Electronics,
    Grocery,
    Pharmacy,
    Restaurant,
}

impl BusinessLine {
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "electronics" => Self::Electronics,
            "grocery" => Self::Grocery,
            "pharmacy" => Self::Pharmacy,
            "restaurant" => Self::Restaurant,
            _ => return None,
        })
    }
}

impl Coupon {
    pub fn new(code: String, business_line: &str, is_active: bool) -> Option<Self> {
        if code.is_empty() || !is_active { return None };
        let business_line = BusinessLine::from_str(business_line)?;
        if !code.bytes().all(|b| {
            (b'a' <= b && b <= b'z')
            | (b'A' <= b && b <= b'Z')
            | (b'0' <= b && b <= b'9')
            | (b == b'_')
        }) { return None }
        Some(Self {
            business_line,
            code,
        })
    }
}

impl Solution {
    pub fn validate_coupons(code: Vec<String>, business_line: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        let mut valid_coupons: Vec<Coupon> =
            std::iter::zip(std::iter::zip(code, business_line), is_active)
            .filter_map(|((code, business_line), is_active)| {
                Coupon::new(code, &business_line, is_active)
            })
            .collect();
        valid_coupons.sort_unstable();
        valid_coupons.into_iter().map(|coupon| coupon.code).collect()
    }
}
