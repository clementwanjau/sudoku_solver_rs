pub fn index_of(array: Vec<i32>, item: i32) -> Option<usize>{
    for (index, _item) in array.iter().enumerate() {
        if _item.cmp(&item.clone()).is_eq() {
            return Some(index)
        }
    }
    None
}