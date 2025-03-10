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