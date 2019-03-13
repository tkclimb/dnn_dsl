use std::convert;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Var {
  X,
  Y,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
  pub name: String,
}

impl Source {
  pub fn new(name: &str) -> Source {
    Source {
      name: name.to_string(),
    }
  }

  pub fn at<U, V>(&self, x: U, y: V) -> Def
  where
    U: Into<VarExpr>,
    V: Into<VarExpr>,
  {
    Def::Access(Access::new(&self.name, x.into(), y.into()))
  }
}
impl convert::Into<VarExpr> for Var {
  fn into(self) -> VarExpr {
    VarExpr::Var(self)
  }
}

/// An expression defining the coordinate to access an input at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VarExpr {
  Var(Var),
  Const(i32),
  Add(Box<VarExpr>, Box<VarExpr>),
  Sub(Box<VarExpr>, Box<VarExpr>),
  Mul(Box<VarExpr>, Box<VarExpr>),
}

/// An expression defining the value to set an image pixel to
#[derive(Debug, Clone)]
pub enum Def {
  Access(Access),
  Const(i32),
  Add(Box<Def>, Box<Def>),
  Sub(Box<Def>, Box<Def>),
  Mul(Box<Def>, Box<Def>),
  Div(Box<Def>, Box<Def>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Access {
  pub(crate) source: String,
  pub(crate) x: VarExpr,
  pub(crate) y: VarExpr,
}

impl Access {
  pub fn new(source: &str, x: VarExpr, y: VarExpr) -> Access {
    let source = source.to_string();
    Access { source, x, y }
  }
}

/*
#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) name: String,
    pub(crate) definition: Def
}

/// An expression defining the value to set an image pixel to
#[derive(Debug, Clone)]
pub enum Def {
    Access(Access),
    // All intermediate calculations happen at type i32 for now
    Const(i32),
    Param(String),
    Cond(Condition),
    // TODO: share code for printing and lowering arithmetic expressions
    // TODO: between VarExpr and Definition
    Add(Box<Definition>, Box<Definition>),
    Mul(Box<Definition>, Box<Definition>),
    Sub(Box<Definition>, Box<Definition>),
    Div(Box<Definition>, Box<Definition>)
}




*/

macro_rules! impl_var_expr_arithmetics {
  ($name:ident, $op:ident) => {
    impl ops::$name<Self> for VarExpr {
      type Output = VarExpr;
      fn $op(self, rhs: Self) -> VarExpr {
        VarExpr::$name(Box::new(self), Box::new(rhs))
      }
    }
    impl ops::$name<i32> for VarExpr {
      type Output = VarExpr;
      fn $op(self, rhs: i32) -> VarExpr {
        VarExpr::$name(Box::new(self), Box::new(VarExpr::Const(rhs)))
      }
    }
    impl ops::$name<VarExpr> for i32 {
      type Output = VarExpr;
      fn $op(self, rhs: VarExpr) -> VarExpr {
        VarExpr::$name(Box::new(VarExpr::Const(self)), Box::new(rhs))
      }
    }
    impl ops::$name<i32> for Var {
      type Output = VarExpr;
      fn $op(self, rhs: i32) -> VarExpr {
        VarExpr::$name(Box::new(VarExpr::Var(self)), Box::new(VarExpr::Const(rhs)))
      }
    }
    impl ops::$name<Var> for i32 {
      type Output = VarExpr;
      fn $op(self, rhs: Var) -> VarExpr {
        VarExpr::$name(Box::new(VarExpr::Const(self)), Box::new(VarExpr::Var(rhs)))
      }
    }
  };
}
impl_var_expr_arithmetics!(Add, add);
impl_var_expr_arithmetics!(Sub, sub);
impl_var_expr_arithmetics!(Mul, mul);

macro_rules! impl_def_arithmetics {
  ($name:ident, $op:ident) => {
    impl ops::$name<Self> for Def {
      type Output = Def;
      fn $op(self, rhs: Self) -> Def {
        Def::$name(Box::new(self), Box::new(rhs))
      }
    }
    impl ops::$name<i32> for Def {
      type Output = Def;
      fn $op(self, rhs: i32) -> Def {
        Def::$name(Box::new(self), Box::new(Def::Const(rhs)))
      }
    }
    impl ops::$name<Def> for i32 {
      type Output = Def;
      fn $op(self, rhs: Def) -> Def {
        Def::$name(Box::new(Def::Const(self)), Box::new(rhs))
      }
    }
  };
}
impl_def_arithmetics!(Add, add);
impl_def_arithmetics!(Sub, sub);
impl_def_arithmetics!(Mul, mul);
impl_def_arithmetics!(Div, div);
