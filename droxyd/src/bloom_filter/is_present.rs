use super::bloom_filter::*;

pub fn is_present(filter: &Vec<u8>, hash_funcs: &Vec<fn(&str) -> String>, word: &str) -> bool
{
    for func in hash_funcs
    {
        let temp = ((from_hex(func(word)) as u64) % (filter.len() as u64)) as usize;
        if filter[temp] == 0
        {
            return false;
        }
    }
    return true;
}
