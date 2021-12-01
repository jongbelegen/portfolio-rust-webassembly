// pub struct RSplitNInclusive<'a, T: 'a, P>
// where
//     P: FnMut(&T) -> bool,
// {
//     v: &'a [T],
//     pred: P,
//     finished: bool,
// }
//
// impl<'a, T: 'a, P: FnMut(&T) -> bool> RSplitNInclusive<'a, T, P> {
//     pub(super) fn new(slice: &'a [T], pred: P) -> Self {
//         Self {
//             v: slice,
//             pred,
//             finished: false,
//         }
//     }
// }
//
// pub fn rsplitn_inclusive<T, F>(slice: &[T], pred: F) -> RSplitNInclusive<'_, T, F>
// where
//     F: FnMut(&T) -> bool,
// {
//     RSplitNInclusive::new(slice, pred)
// }
//
// impl<'a, T, P> Iterator for RSplitNInclusive<'a, T, P>
// where
//     P: FnMut(&T) -> bool,
// {
//     type Item = &'a [T];
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.finished {
//             return None;
//         }
//
//         let pos = self.v.iter().rposition(|x|(self.pred)(x)).unwrap_or(0);
//
//         if pos == 0 {
//             self.finished = true;
//         }
//
//         let result = Some(&self.v[pos..]);
//         self.v = &self.v[..pos];
//         result
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test() {
//         let a = &[1, 2, 3, 4];
//
//         let mut a = rsplitn_inclusive(a, |a| a == &2);
//     }
// }
