use cw20::Denom;

pub fn denom_to_string(denom: &Denom) -> String {
    match denom {
        Denom::Native(s) => s.clone(),
        Denom::Cw20(addr) => addr.to_string(),
    }
}

pub fn denom_is_native(denom: Denom) -> bool {
    match denom {
        Denom::Native(_) => true,
        Denom::Cw20(_) => false,
    }
}
