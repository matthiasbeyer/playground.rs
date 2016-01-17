pub fn generate_variants<A, B, C, F>(a: A, modders: Vec<B>, f: F)
    -> Vec<C>
    where
        F: Fn(&A, B) -> C
{
    modders.into_iter().map(|b| f(&a, b)).collect()
}

