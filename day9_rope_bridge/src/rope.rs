// src/rope.rs
//
// NOTE: several chunks of this code were borrowed from `ericwburden` on GitHub, whose
// entire Advent of Code repo is phenomenal (https://github.com/ericwburden/advent_of_code_2022)

use crate::motion::Motion;
use itertools::Itertools;
use std::cmp::Ordering::*;
use std::collections::HashSet;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Rope<const N: usize> {
    knots: [Knot; N],
    pub path: HashSet<Knot>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Knot(pub i32, pub i32);

impl Knot {
    pub fn new() -> Knot {
        Knot(0, 0)
    }

    fn too_far(&self, other: &Knot) -> bool {
        let Knot(x1, y1) = self;
        let Knot(x2, y2) = other;

        x1.abs_diff(*x2) > 1 || y1.abs_diff(*y2) > 1
    }
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        let knots = [Knot::new(); N];
        let path = HashSet::from([Knot::new()]);

        Self { knots, path }
    }

    fn follow(&mut self, leader: usize, follower: usize) {
        let Knot(head_x, head_y) = self.knots[leader];
        let Knot(tail_x, tail_y) = self.knots[follower];

        self.knots[follower] = match (head_x.cmp(&tail_x), head_y.cmp(&tail_y)) {
            (Less, Less) => Knot(tail_x - 1, tail_y - 1),
            (Less, Equal) => Knot(tail_x - 1, tail_y),
            (Less, Greater) => Knot(tail_x - 1, tail_y + 1),
            (Equal, Less) => Knot(tail_x, tail_y - 1),
            (Equal, Equal) => unreachable!(),
            (Equal, Greater) => Knot(tail_x, tail_y + 1),
            (Greater, Less) => Knot(tail_x + 1, tail_y - 1),
            (Greater, Equal) => Knot(tail_x + 1, tail_y),
            (Greater, Greater) => Knot(tail_x + 1, tail_y + 1),
        };

        // If the `follower` is the last position in the rope, we record its position
        if follower == N - 1 {
            self.path.insert(self.knots[N - 1]);
        }
    }

    pub fn move_head_knot(&mut self, motion: &Motion) {
        let (n_steps, offset) = match motion {
            Motion::Up(n) => (n, (0, -1)),
            Motion::Down(n) => (n, (0, 1)),
            Motion::Left(n) => (n, (-1, 0)),
            Motion::Right(n) => (n, (1, 0)),
        };

        for _ in 0..*n_steps {
            self.knots[0] += offset;

            for (leader, follower) in (0..N).tuple_windows() {
                if self.knots[leader].too_far(&self.knots[follower]) {
                    self.follow(leader, follower);
                }
            }
        }
    }
}

impl AddAssign<(i32, i32)> for Knot {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        let Knot(x, y) = self;

        let (dx, dy) = rhs; // change in x and y
        *self = Knot(*x + dx, *y + dy);
    }
}
