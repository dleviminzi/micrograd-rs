use std::{ops, fmt::{self, Debug}};

#[derive(Clone, Copy, Debug)]
enum Ops {
    None,
    Add,
    Subtract,
    Mul,
    Tanh,
    // Divide,
}

// TODO: implement custom debug to show parent values
#[derive(Clone, Debug)]
struct Value {
    data: f32,
    grad: f32,
    parents: Vec<*const Value>,
    operation: Ops,
}

impl Value {
    fn tanh(self) -> Value {
        println!("{:?}", self.data);
        let t:f32 = ((2.0 * self.data).exp() - 1.0)/((2.0 * self.data).exp() + 1.0);
        let parents:Vec<*const Value> = vec![&self];
        let operation: Ops = Ops::Tanh;

        return Value { data: (t), grad: (1.0), parents: (parents), operation: (operation) }
    }
}


impl ops::Add<&Value> for &Value {
    type Output = Value;

    fn add(self, rhs: &Value) -> Self::Output {
        let data: f32 = self.data + rhs.data;
        let grad: f32 = 0.0;
        let parents:Vec<*const Value> = vec![self, rhs];
        let operation: Ops = Ops::Add; 

        return Value{data: data, grad: grad, parents: parents, operation: operation,}
    }
}

impl ops::Mul<&Value> for &Value {
    type Output = Value;

    fn mul(self, rhs: &Value) -> Self::Output {
        let data: f32 = self.data * rhs.data;
        let grad: f32 = 0.0;
        let parents:Vec<*const Value> = vec![self, rhs];
        let operation: Ops = Ops::Mul; 

        return Value{data: data, grad: grad, parents: parents, operation: operation,}
    }
}

fn main() {
    let x1 = Value{data: 2.0, grad: 0.0, parents: vec![], operation: Ops::None}; 
    let x2 = Value{data: 0.0, grad: 0.0, parents: vec![], operation: Ops::None}; 
    let w1 = Value{data: -3.0, grad: 0.0, parents: vec![], operation: Ops::None}; 
    let w2 = Value{data: 1.0, grad: 0.0, parents: vec![], operation: Ops::None}; 
    let b = Value{data: 6.881373, grad: 0.0, parents: vec![], operation: Ops::None}; 

    let x1w1 = &x1 * &w1;
    let x2w2 = &x2 * &w2;

    let x1w1_plus_x2w2 = &x1w1 + &x2w2;

    let n = &x1w1_plus_x2w2 + &b;

    let o = n.tanh();
    println!("{:?}", o)
}
