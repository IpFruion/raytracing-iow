use std::ops::{Bound, Range, RangeBounds, RangeTo};

#[macro_export]
macro_rules! assign_math {
    ($name:ty, $target:ty, $fn_name:ident, $op:tt) => {
        impl $target for $name {
            fn $fn_name(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
    }
}

#[macro_export]
macro_rules! clone_math {
    ($name:ty, $target:ty, $fn_name:ident, $op:tt) => {
        impl $target for $name {
            type Output = Self;

            fn $fn_name(self, rhs: Self) -> Self::Output {
                // let mut new = self.clone();
                // new $op rhs;
                // new
                Self {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }
    };
}

pub trait Interval<T> {
    fn surrounds<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: PartialOrd<T>;

    fn clamp(&self, t: T) -> T
    where
        T: PartialOrd<T> + Copy;
}

impl<T, B: RangeBounds<T>> Interval<T> for B {
    fn surrounds<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: PartialOrd<T>,
    {
        (match self.start_bound() {
            Bound::Included(start) | Bound::Excluded(start) => start < item,
            Bound::Unbounded => true,
        }) && (match self.end_bound() {
            Bound::Included(end) | Bound::Excluded(end) => item < end,
            Bound::Unbounded => true,
        })
    }

    fn clamp(&self, t: T) -> T
    where
        T: PartialOrd<T> + Copy,
    {
        let t = match self.start_bound() {
            Bound::Included(start) if start > &t => *start,
            Bound::Excluded(start) if start >= &t => *start, // this isn't quite right since it
            // would just bring it back to the start
            _ => t,
        };

        let t = match self.end_bound() {
            Bound::Included(end) if end < &t => *end,
            Bound::Excluded(end) if end <= &t => *end,
            _ => t,
        };
        t
    }
}
