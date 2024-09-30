#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

use core::{
    fmt::{self, Debug},
    iter::FusedIterator,
};

/// Like the [`Iterator::scan`], but use `B`, instead of `Option<B>`,
/// which can bring better [`size_hint`] and ergonomics.
///
/// At the same time,
/// it will also be able to implement [`ExactSizeIterator`] and [`FusedIterator`]
///
/// [`size_hint`]: Iterator::size_hint
pub trait IterScanB: Iterator + Sized {
    /// An iterator adapter which, like [`Iterator::scan`],
    /// but returns a value of `B` instead of `Option<B>`.
    /// which can bring better [`size_hint`] and ergonomics.
    ///
    /// At the same time,
    /// it will also be able to implement [`ExactSizeIterator`] and [`FusedIterator`]
    ///
    /// # Examples
    /// ```
    /// # use iter_scanb::IterScanB;
    /// let a = [1, 2, 3, 4];
    ///
    /// let mut iter = a.iter().scanb(1, |state, &x| {
    ///     *state *= x;
    ///     -*state
    /// });
    ///
    /// assert_eq!(iter.next(), Some(-1));
    /// assert_eq!(iter.next(), Some(-2));
    /// assert_eq!(iter.next(), Some(-6));
    /// assert_eq!(iter.next(), Some(-24));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    /// [`size_hint`]: Iterator::size_hint
    fn scanb<S, B, F>(self, init_state: S, f: F) -> ScanB<Self, S, F>
    where F: FnMut(&mut S, Self::Item) -> B,
    {
        ScanB {
            iter: self,
            f,
            state: init_state,
        }
    }
}
impl<I: Iterator> IterScanB for I {}

/// Create from [`IterScanB::scanb`]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ScanB<I, S, F> {
    iter: I,
    f: F,
    state: S,
}

impl<I: Debug, S: Debug, F> Debug for ScanB<I, S, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScanB")
            .field("iter", &self.iter)
            .field("state", &self.state)
            .finish()
    }
}

impl<I, S, F, B> Iterator for ScanB<I, S, F>
where I: Iterator,
      F: FnMut(&mut S, I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
            .map(|ele| (self.f)(&mut self.state, ele))
    }

    // NOTE:
    // Do not implement `nth`,
    // otherwise it may not have been executed on every element

    fn fold<B1, F1>(mut self, init: B1, mut f: F1) -> B1
    where Self: Sized,
          F1: FnMut(B1, Self::Item) -> B1,
    {
        self.iter.fold(init, |acc, ele| {
            let ele = (self.f)(&mut self.state, ele);
            f(acc, ele)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, S, F> FusedIterator for ScanB<I, S, F>
where I: FusedIterator,
      F: for<'a> FnMut(&'a mut S, I::Item),
{
}

impl<I, S, F> ExactSizeIterator for ScanB<I, S, F>
where I: ExactSizeIterator,
      F: for<'a> FnMut(&'a mut S, I::Item),
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}
