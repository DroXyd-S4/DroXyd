use super::*;

pub fn bloom_size_computing(num_key: i64, error_rate: f64) -> Vec<u8>
{
    let mut x = 2.0_f64;
    let mut y = 2.0_f64;
    y = y.ln();
    x = x.powf(y);
    let mut fac = 1.0/x;
    fac = fac.ln();
    let mut n = ((num_key as f64) * error_rate.ln()) / fac;
    n = n.ceil();
    x = 2.0_f64;
    let mut k = (x.ln()*n) / (num_key as f64);
    k = k.ceil();
    println!("On a {} et {} !!", n, k);

    let mut res: Vec<u8> = vec![];
    return res;
}

pub fn bloom_filter(num_key: i64, error_rate: f64) -> ()
{
    // TODO
}
