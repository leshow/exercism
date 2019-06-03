use std::ops::{AddAssign, Rem};
// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    f: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: AsRef<str>>(f: F, subs: S) -> Matcher<T> {
        Matcher {
            f: Box::new(f),
            subs: subs.as_ref().into(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T>
where
    T: ToString + Copy,
{
    pub fn new() -> Self {
        Fizzy(vec![])
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    /// map this fizzy onto every element of an interator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        // unimplemented!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        iter.map(move |x| {
            let mut ret = String::new();
            for matcher in &self.0 {
                if (matcher.f)(x) {
                    ret += &matcher.subs;
                }
            }
            if ret.is_empty() {
                ret = x.to_string();
            }
            ret
        })
    }
}

impl<T> From<Vec<Matcher<T>>> for Fizzy<T> {
    fn from(matchers: Vec<Matcher<T>>) -> Self {
        Fizzy(matchers)
    }
}
/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: ToString + Copy + Default + Rem<Output = T> + From<u8> + PartialEq,
{
    Fizzy(vec![
        Matcher::new(move |n| n % 3.into() == T::default(), "fizz"),
        Matcher::new(move |n| n % 5.into() == T::default(), "buzz"),
    ])
}
