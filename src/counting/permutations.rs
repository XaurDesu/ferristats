use factorial

fn size(set:Vec<>, permutation_size:i32) -> i32 {
    let n = set.size;
    return factorial.factorial(n)/factorial.factorial(n-permutation_size)
}