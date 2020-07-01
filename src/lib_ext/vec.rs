pub trait GetWhere<T> { 
    fn next<F>(self: &Self, index: usize, predicate: F) -> Option<usize> where F: Fn (&T) -> bool;
    fn prev<F>(self: &Self, index: usize, predicate: F) -> Option<usize> where F: Fn (&T) -> bool;
}

impl <T> GetWhere<T> for std::vec::Vec<T> {
    fn prev<F>(self: &Self, i: usize, predicate: F)  -> Option<usize> where F: Fn (&T) -> bool {
        let mut result = None;
        if i > 0 { 
            for index in (0..i).rev() {
                if predicate(&self[index]) {
                    result = Some(index);
                    break;
                }
            }
        } 
        result
    }

    fn next<F>(self: &Self, i: usize, predicate: F)  -> Option<usize> where F: Fn (&T) -> bool {
        let len = self.len();
        let mut result = None;
        let i = i + 1;
        if i < len { 
            for index in i..len {
                if predicate(&self[index]) {
                    result = Some(index);
                    break;
                }
            }
        } 
        result
    }
}
