pub fn is_present(filter: &mut Vec<u8>, hash_funcs: Vec<fn(&str) -> i64>, word: &str) -> bool
{
    for func in hash_funcs
    {
        if filter[func(word) as usize] == 0
        {
            return false;
        }
    }
    return true;
}
