use core::f64;
use std::iter::successors;
// TODO: make panes resizeable.
// TODO: ratios instead of floats
use druid::{Data, Widget, WidgetPod};

use druid::widget::{Flex, Axis};


pub struct SplitN<T>
  { children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>
  , split_axis: Axis
  , split_points_chosen: Vec<f64>
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
         , split_points_chosen: Self::split_evenly(s-1, 0.)
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

  pub fn split_points(mut self, mut split_points: Vec<f64>) -> Self {
    // either split evenly the rest / ignore the excess
    // or forbid it
    // assert_eq!(split_points.len(), self.children.len() - 1, "types could've :^) ");
    let len = self.children.len() - 1;
    if split_points.len() >= len {
      split_points.truncate(len);
      assert!(split_points.iter().sum::<f64>() <= 1.);
    } else {
      let sum : f64 = split_points.iter().sum();
      split_points.extend(Self::split_evenly(len - split_points.len(), sum));
    }

    self.split_points_chosen = split_points;
    self
  }

  fn split_evenly(len: usize, offset: f64) -> Vec<f64> {
    let inc = (1. - offset) / len as f64;
    std::iter::successors( Some((1, inc)) 
                         , |(n, x)| {
        if n > &len {
          return None;
        }
        let x1 = x + inc;
        if x1 < 1. { Some((n+1, x1)) } else { None }
      }).map(DoubleExt::snd).collect()
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