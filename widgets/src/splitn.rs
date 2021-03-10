use core::f64;
use std::convert::TryInto;

// TODO: make panes resizeable.
use druid::{Widget, WidgetPod};
use druid::widget::Axis;

use crate::Rational;

pub struct SplitN<T>
  { children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>
  , split_axis: Axis
  , split_points_chosen: Vec<Rational>
  , min_sizes: Vec<f64>
  , bar_size: f64
  , solid: bool
  , draggable: bool
  }

impl<T> SplitN<T> {
  pub fn new( split_axis: Axis
        , children : Vec<impl Widget<T> + 'static>
        ) -> Self {
    let s = children.len();
    Self { split_axis
         , children: children.into_iter().map(|w| WidgetPod::new(w).boxed()).collect()
         , split_points_chosen: Self::split_evenly(s, Rational::ZERO)
         , min_sizes: vec![0.; s] 
         , bar_size: 5.
         , solid: true
         , draggable: false
         }
  }

  pub fn columns(children: Vec<impl Widget<T> + 'static>) -> Self {
    Self::new(Axis::Horizontal, children)
  }

  pub fn rows(children: Vec<impl Widget<T> + 'static>) -> Self {
    Self::new(Axis::Vertical, children)
  }

  pub fn split_points(mut self, mut split_points: Vec<Rational>) -> Self {
    // either split evenly the rest / ignore the excess
    // or forbid it
    // assert_eq!(split_points.len(), self.children.len() - 1, "types could've :^) ");
    let len = self.children.len();
    if split_points.len() >= len - 1 {
      split_points.truncate(len);
      assert!(split_points.iter().sum::<Rational>() <= Rational::ONE);
    } else {
      let sum : Rational = split_points.iter().sum();
      split_points.extend(Self::split_evenly(len - split_points.len(), sum));
    }

    self.split_points_chosen = split_points;
    self
  }

  fn split_evenly(len: usize, offset: Rational) -> Vec<Rational> {
    use std::convert::TryFrom;

    if len == 1 {
      return vec![];
    }

    let inc : Rational = (Rational::ONE - offset) / Rational::try_from(len).unwrap();
    std::iter::successors( Some((1, offset + inc)) 
                         , |(n, x)| {
        if *n < (len - 1) {
          Some((n+1, x + inc))
        } else {
          None
        }
      }).map(DoubleExt::snd).collect()
  }
}

#[cfg(test)]
mod test_splitn {
  use crate::SplitN;
  use crate::Rational;
  #[test]
  fn test_split_evenly() {
    assert_eq!( SplitN::<()>::split_evenly(1, Rational::ZERO)
              , vec![]
              );
    assert_eq!( SplitN::<()>::split_evenly(2, Rational::ZERO)
              , vec![ Rational::new(1, 2)]
              );
    assert_eq!( SplitN::<()>::split_evenly(3, Rational::ZERO)
              , vec![ Rational::new(1, 3)
                    , Rational::new(2, 3)
                    ]
              );
    assert_eq!( SplitN::<()>::split_evenly(4, Rational::new(1, 2))
              , vec![ Rational::new(5, 8)
                    , Rational::new(6, 8)
                    , Rational::new(7, 8)
                    ]
              );
    assert_eq!( SplitN::<()>::split_evenly(3, Rational::new(2, 3))
              , vec![ Rational::new(7, 9)
                    , Rational::new(8, 9)
                    ]
              );
  }
}

trait DoubleExt<T, U> {
  fn fst(self) -> T; 
  fn snd(self) -> U;
}
trait TripleExt<T> {
  fn trd(self) -> T;
}

impl<T, U> DoubleExt<T, U> for (T, U) {
  fn fst(self) -> T {
    self.0
  }
  fn snd(self) -> U {
    self.1
  }
}
impl<T, U, V> DoubleExt<T, U> for (T, U, V) {
  fn fst(self) -> T {
    self.0
  }
  fn snd(self) -> U {
    self.1
  }
}
impl<T, U, V> TripleExt<V> for (T, U, V) {
  fn trd(self) -> V {
    self.2
  }
}