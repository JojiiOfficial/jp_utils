use super::FuriSequence;
use crate::furigana::as_part::AsPart;
use std::ops::Deref;

/// Iterator over furigana sequences
pub struct SeqIter<'s, T> {
    seq: &'s FuriSequence<T>,
    pos: usize,
}

impl<'s, T> SeqIter<'s, T>
where
    T: AsPart,
{
    #[inline]
    pub fn new(seq: &'s FuriSequence<T>) -> Self {
        Self { seq, pos: 0 }
    }
}

impl<'s, T> Iterator for SeqIter<'s, T> {
    type Item = IterItem<'s, T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.seq.parts.get(self.pos)?;
        self.pos += 1;
        Some(IterItem(item))
    }
}

/// Borrowed item for borrowed iterator
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IterItem<'s, T>(&'s T);

impl<'s, T> Deref for IterItem<'s, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'s, T> AsRef<T> for IterItem<'s, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}
