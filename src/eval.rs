use std::collections::HashMap;
use std::fmt;

enum Value {
	Number(I64),
	Callable(Callable),
	Nil
}

impl Value{
	fn is_truthy(&self) -> bool{
		use Value::*;
		match *self {
			Number(n) => n !=0,
			_ => true, 
		} 
	}
	fn into_num(self) -> i64{
		match self {
			Value::Number(n) => n,
			other => panic!("{:?} Not a number", other),
		}
	} 
}