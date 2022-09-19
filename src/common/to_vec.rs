pub trait ToVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl <T, I: Iterator<Item=T>> ToVec<T> for I {
    fn to_vec(self) -> Vec<T> {
        self.collect()
    }
}
