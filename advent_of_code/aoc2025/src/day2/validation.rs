pub fn is_invalid_id(id_: u64) -> bool {
    let id = id_.to_string();
    let num_chars = id.len();
    if num_chars % 2 != 0 {
        return false;
    }

    let (half_1, half_2) = id.split_at(num_chars/2);
    half_1 == half_2
}