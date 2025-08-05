/// What should the type of _function be?
pub fn map<T, U: FnMut(T) -> V, V>(input: Vec<T>, mut _function: U) -> Vec<V> {
    let mut result = Vec::<V>::new();
    if input.is_empty() {
        return result;
    }

    for input_element in input {
        result.push(_function(input_element));
    }

    result
}
