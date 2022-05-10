use crate::VecCell;

pub struct Iter<'a, T> {
    vc: &'a VecCell<T>,
    idx: usize,
}

impl<'a, T: Clone> Iter<'a, T> {
    pub(crate) fn new(vc: &'a VecCell<T>) -> Self {
        Self { vc, idx: 0 }
    }
}

impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vc.get(self.idx);
        self.idx += 1;
        item
    }
}
