Extend `Iterator::scan`, but use `B`, instead of `Option<B>`,
which can bring better `size_hint` and ergonomics.

At the same time,
it will also be able to implement `ExactSizeIterator` and `FusedIterator`

# Examples
```
use iter_scanb::IterScanB;
let a = [1, 2, 3, 4];

let mut iter = a.iter().scanb(1, |state, &x| {
    *state *= x;
    -*state
});

assert_eq!(iter.next(), Some(-1));
assert_eq!(iter.next(), Some(-2));
assert_eq!(iter.next(), Some(-6));
assert_eq!(iter.next(), Some(-24));
assert_eq!(iter.next(), None);
```
