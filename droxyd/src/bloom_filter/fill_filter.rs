pub fn fill_filter(filter: &mut Vec<i32>, hash_funcs: Vec<fn(&str) -> i32>, word: &str) -> ()
{
    for func in hash_funcs
    {
        filter[func(word)] = 1;
    }
}
