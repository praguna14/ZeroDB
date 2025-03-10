use heapless::String as HeapLessString;
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub id: i32,
    pub username: HeapLessString<32>,
    pub email: HeapLessString<256>,
}
impl Row {
    pub fn max_size() -> usize {
        size_of::<i32>() + size_of::<HeapLessString<32>>() + size_of::<HeapLessString<256>>()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_row_max_size() {
        let total = size_of::<i32>() + size_of::<HeapLessString<32>>() + size_of::<HeapLessString<256>>();
        assert_eq!(Row::max_size(), total);
    }
}