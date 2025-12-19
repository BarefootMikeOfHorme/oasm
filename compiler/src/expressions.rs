#![allow(dead_code)]
#[derive(Debug, Clone)] pub enum Expr { Num(i64), Add(Box<Expr>,Box<Expr>), Sub(Box<Expr>,Box<Expr>), Mul(Box<Expr>,Box<Expr>), Div(Box<Expr>,Box<Expr>) }
impl Expr { pub fn eval(&self)->i64{match self{Expr::Num(n)=>*n,Expr::Add(a,b)=>a.eval()+b.eval(),Expr::Sub(a,b)=>a.eval()-b.eval(),Expr::Mul(a,b)=>a.eval()*b.eval(),Expr::Div(a,b)=>{let rhs=b.eval();if rhs==0{0}else{a.eval()/rhs}}}}}
pub fn parse_simple(input:&str)->Expr{input.trim().parse::<i64>().map(Expr::Num).unwrap_or(Expr::Num(0))}
