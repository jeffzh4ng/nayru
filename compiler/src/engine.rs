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
        }
    }
}
