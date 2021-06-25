pub fn min_by_fkey<T, F>(vec: &Vec<T>, selector: F) -> Option<&T>
where F: Fn(&T) -> f32 {

    let mut result: Option<&T> = None;
    let mut result_value = f32::INFINITY;
    for item in vec.iter() {
        let value = selector(&item);
        if value < result_value {
            result = Some(&item);
            result_value = value;
        }
    }
    result
}