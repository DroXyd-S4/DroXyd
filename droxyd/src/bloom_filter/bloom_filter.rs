use super::hash_functions::*;

pub fn print_filter(filter: &Vec<u8>) -> ()
{
    print!("State of the filter:");
    let mut index = -1;
    let mut count = 10;
    for elt in filter
    {
        count += 1;
        if count > 9
        {
            count = 0;
            index += 1;
            println!();
            print!("[{:03} -> {:03}]: ", index*10, (index+1)*10);
        }

        print!("{} ", elt);
    }
    println!();
}

pub fn from_hex(word: String) -> i64
{
    let mut len = word.len();
    let mut res: i64 = 0;
    for c in word.chars()
    {
        len -= 1;
        if c >= '0' && c <= '9'
        {
            let mut temp = ((c as i64) - 48) as i64;
            for _i in 0..len
            {
                temp = ((temp as i128) * (16 as i128)) as i64;
            }
            res += temp;
        }
        if c >= 'a' && c <= 'f'
        {
            let mut temp = ((c as i64) - 97) as i64;
            for _i in 0..len
            {
                temp = ((temp as i128) * (16 as i128)) as i64;
            }
            res += temp;
        }
    }
    return res;
}

pub fn bloom_size_computing(num_key: u64, error_rate: f64) -> Vec<u64>
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

    let res: Vec<u64> = vec![
        n as u64,
        k as u64
    ];
    return res;
}

pub fn init_filter(len: u64, error_rate: f64) -> Vec<u8>
{
    let dimensions = bloom_size_computing(len, error_rate);
    let mut filter: Vec<u8> = vec![];
    for _i in 0..(dimensions[0])
    {
        filter.push(0);
    }
    return filter;
}

pub fn add_elt(filter: &mut Vec<u8>, word: &String) -> ()
{
    let r1 = ((from_hex(sha256(&word))) as u64) % (filter.len() as u64);
    let r2 = ((from_hex(md5(&word))) as u64) % (filter.len() as u64);
    let r3 = ((from_hex(sha256(&sha256(&word)))) as u64) % (filter.len() as u64);
    filter[r1 as usize] = 1;
    filter[r2 as usize] = 1;
    filter[r3 as usize] = 1;
}

pub fn bloom_filter(words: &Vec<String>, error_rate: f64) -> Vec<u8>
{
    let dimensions = bloom_size_computing((words.len()*5) as u64, error_rate);
    let mut filter: Vec<u8> = vec![];
    for _i in 0..(dimensions[0])
    {
        filter.push(0);
    }

    for i in 0..words.len()
    {
        let r1 = ((from_hex(sha256(&words[i]))) as u64) % dimensions[0];
        let r2 = ((from_hex(md5(&words[i]))) as u64) % dimensions[0];
        let r3 = ((from_hex(sha256(&sha256(&words[i])))) as u64) % dimensions[0];

        //println!("Dim : {}, {}, {}, {}", dimensions[0], r1, r2, r3);

        filter[r1 as usize] = 1;
        filter[r2 as usize] = 1;
        filter[r3 as usize] = 1;
    }

    return filter;
}
