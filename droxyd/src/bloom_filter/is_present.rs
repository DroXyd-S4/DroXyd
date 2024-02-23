pub fn is_present(filter: &mut Vec<i32>, hash_funcs: Vec<fn(&str) -> i32>, word: &str) -> bool
{
    for func in hash_funcs
    {
        if filter[func(word)] == 0
        {
            return false;
        }
    }
    return true;
}
