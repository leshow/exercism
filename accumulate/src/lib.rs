pub fn map<T, F, V>(input: Vec<T>, f: F) -> Vec<V>
where
    F: FnMut(T) -> V,
{
   input.into_iter().map(f).collect()
}
