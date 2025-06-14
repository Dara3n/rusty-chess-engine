use rand::Rng;




pub fn get_random_element<T: Clone>(vector: &Vec<T>) -> Option<T> {
    if vector.is_empty() {
        return None;
    }
    
    let mut rng = rand::rng();
    let index = rng.random_range(0..vector.len());
    
    Some(vector[index].clone())
}