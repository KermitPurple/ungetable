pub struct Ungetable<I: Iterator> {
    iter: I,
    ungotten: Vec<I::Item>,
}

impl<I: Iterator> Ungetable<I> {
    fn new(into: impl IntoIterator<IntoIter = I>) -> Self {
        Self {
            iter: into.into_iter(),
            ungotten: vec![],
        }
    }

    fn unget(&mut self, item: I::Item) {
        self.ungotten.push(item);
    }
}

impl<I: Iterator> Iterator for Ungetable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.ungotten.pop().or_else(|| self.iter.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ungetable_test() {
        let mut it = Ungetable::new([1, 2, 3, 4]);
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        it.unget(3);
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(4));
        assert_eq!(it.next(), None);
    }
}
