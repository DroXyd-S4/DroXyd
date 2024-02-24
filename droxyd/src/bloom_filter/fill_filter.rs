pub fn fill_filter(filter: &mut Vec<u8>, hash_funcs: Vec<fn(&str) -> i64>, word: &str) -> ()
{
    for func in hash_funcs
    {
        filter[func(word) as usize] = 1;
    }
}
