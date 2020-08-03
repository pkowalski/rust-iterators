pub fn flatten<O>(iter: O) -> Flatten<O::IntoIter>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_inner: Option<<O::Item as IntoIterator>::IntoIter>,
    back_inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_inner: None,
            back_inner: None
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_inner_iter) = self.front_inner {
                if let Some(i) = front_inner_iter.next() {
                    return Some(i);
                }
                self.front_inner = None;
            }

            if let Some(next_front_inner_iter) = self.outer.next() {
                self.front_inner = Some(next_front_inner_iter.into_iter());
            } else {
                return self.back_inner.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O> 
where
    O: Iterator + DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator
{
    fn next_back(&mut self) ->  Option<Self::Item>{
        loop {
            if let Some(ref mut back_inner_iter) = self.back_inner {
                if let Some(i) = back_inner_iter.next_back() {
                    return Some(i);
                }
                self.back_inner = None;
            }

            if let Some(next_back_inner_iter) = self.outer.next_back() {
                self.back_inner = Some(next_back_inner_iter.into_iter());
            } else {
                return self.front_inner.as_mut()?.next();
            }
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn once() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
    }

    #[test]
    fn two_inner() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2)
    }

    #[test]
    fn two_outer() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2)
    }

    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), Vec::<()>::new(), Vec::<()>::new()])
                .count(),
            0
        );
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]]).rev().collect::<Vec<_>>(), vec!["b", "a"]
        )
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a", "b"]]).rev().collect::<Vec<_>>(), vec!["b", "a"]
        )
    }

    #[test]
    fn front_back() {
        
        let mut flat = flatten(vec![vec!["a1", "a2"], vec!["b1", "b2"], vec!["c3", "c4"]]);
        assert_eq!(flat.next(), Some("a1"));
        assert_eq!(flat.next_back(), Some("c4"));
        assert_eq!(flat.next(), Some("a2"));
        assert_eq!(flat.next_back(), Some("c3"))
        
    }
}
