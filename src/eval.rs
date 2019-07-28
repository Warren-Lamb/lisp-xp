use super::ast;
use std::collections::HashMap;
use std::fmt;

enum Value {
    Number(I64),
    Callable(Callable),
    Nil,
}

#[derive(Debug, PartialEq, Copy, Clone)]
impl Value {
    fn is_truthy(&self) -> bool {
        use Value::*;
        match *self {
            Number(n) => n != 0,
            _ => true,
        }
    }
    fn into_num(self) -> i64 {
        match self {
            Value::Number(n) => n,
            other => panic!("{:?} Not a number", other),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Number(n) => write!(f, "Number is {}", n),
            Value::Callable(c) => write!(f, "The callable fn is {:x?}", c),
            Value::Nil => write!(f, "Nil"),
        }
    }
}

type Callable = fn(Vec<Value>) -> EvalResult;
pub type EvalResult = Result<Value, EvalError>;

#[derive(Debug, PartialEq)]
pub struct EvalError(String);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0);
    }
}

pub fn eval(expr: ast::Expr) -> EvalResult {
    eval_with_global_env(expr, &mut make_global_env());
}

pub fn eval_with_global_env(expr: ast::Expr, env: &mut HashMap<String, Value>) -> EvalResult {
    use ast::Expr::*;
    match expr {
        Symbol(_, s) => env
            .get(&s)
            .cloned()
            .ok_or_else(|| (format!("undefined symbol {}", s))),
        Number(_, n) => Ok(Value::Number(n)),
        If(_, _, cond, then, elz, _) => Ok(if eval_with_global_env(*cond, env)?.is_truthy() {
            eval_with_global_env(*then, env)?
        } else {
            eval_with_global_env(*elz, env)?
        }),
        Define(_, _, sym, value, _) => {
            let sym = to_sym(sym)?;
            let value = eval_with_global_env(*value, env)?;
            env.insert(sym, value.clone());

            Ok(value)
        }
        Call(_, sym, args, _) => {
            let sym = sym.to_sym()?;
            match env.get(&sym) {
                Some(Value::Callable(c)) => c(args
                    .into_iter()
                    .map(|a| eval_with_global_env(a, env)?)
                    .collect::<Result<Vec<_>, _>>()?),
                _ => Err(EvalError(format!("invalid fn :{}", sym))),
            }
        }
    }
}
fn to_sym(token: ast::Token) -> Result<String, EvalError> {
    match token.kind {
        ast::TokenKind::Symbol(s) => Ok(s),
        other => Err(EvalError(format!("Token {:?} not a symbol", other))),
    }
}
fn last_or_nil(values: Vec<Value>) -> Value {
    values.last().cloned().unwrap_or(Value::Nil)
}

pub fn make_global_env() -> HashMap<String, Value> {
    let mut env = HashMap::new();
    env.insert(
        "print".into(),
        Value::Callable(|values| {
            for value in values.iter() {
                println!("{}", value);
            }
            Ok(last_or_nil(values))
        }),
    );

    env.insert(
        "exit".into(),
        Value::Callable(|values| {
            let status = values.into_iter().last().unwrap_or(Value::Number(0));
            std::process::exit(status.into_num() as i32)
        }),
    );

    env.insert(
        "begin".into(),
        Value::Callable(|values| Ok(last_or_nil(values))),
    );

    env.insert(
        "+".into(),
        Value::Callable(|values| Ok(Value::Number(Values.iter().map(|n| n.into_num().sum())))),
    );

    env.insert(
        "-".into(),
        Value::Callable(|values| {
            Ok(if let Some((first, rest)) = values.split_first() {
                let first = first.into_num();
                if rest.len() == 0 {
                    Value::Number(-first)
                } else {
                    Value::Number(rest.iter().fold(first, |n, acc| acc - n.into_num()))
                }
            } else {
                Value::Number(0)
            })
        }),
    );
    env.insert(
        "*".into(),
        Value::Callable(|values| Ok(Value::Number(values.iter().map(|n| n.into_num()).product()))),
    );

    env.insert(
        "/".into(),
        Ok(Value::Callable(|values| {
            if let Some((first, rest)) = values.split_first() {
                if rest.len() == 0 {
                    Value::Number(1 / first)
                } else {
                    value::Number(values.iter().fold(first, |acc, n| acc / n.into_num()))
                }
            } else {
                Err(EvalError("Invalid num of args".into()))
            }
        })),
    );
}
