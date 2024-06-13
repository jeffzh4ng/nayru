use ordered_float::NotNan;
use std::collections::BTreeSet;
use std::ops::{Add, Mul};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Op {
    Add,
    Mul,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Expr {
    pub evaluated: NotNan<f32>,
    pub expr: Option<(Op, BTreeSet<Box<Expr>>)>,
    pub grad: NotNan<f32>,
    pub backward: (),
}

impl Expr {
    pub fn new(evaluated: NotNan<f32>, expr: Option<(Op, BTreeSet<Box<Expr>>)>) -> Self {
        Expr {
            evaluated,
            expr,
            grad: NotNan::new(0.0).unwrap(),
        }
    }
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, rhs: Self) -> Self::Output {
        let evaluated = self.evaluated + rhs.evaluated;
        let mut operands = BTreeSet::new();
        operands.insert(Box::new(self));
        operands.insert(Box::new(rhs));

        Expr {
            evaluated,
            expr: Some((Op::Add, operands)),
            grad: todo!(),
        }
    }
}

impl Mul for Expr {
    type Output = Expr;

    fn mul(self, rhs: Self) -> Self::Output {
        let evaluated = self.evaluated * rhs.evaluated;
        let mut operands = BTreeSet::new();
        operands.insert(Box::new(self));
        operands.insert(Box::new(rhs));

        Expr {
            evaluated,
            expr: Some((Op::Mul, operands)),
            grad: todo!(),
        }
    }
}
