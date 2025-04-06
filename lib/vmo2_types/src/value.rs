#[derive(Debug, PartialEq, Eq, Clone, Ord, PartialOrd)]
pub enum Value {
    UInt(u32),
    Bool(bool),
    String(String),
}

impl std::ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::UInt(a), Value::UInt(b)) => Value::UInt(a + b),
            (Value::String(a), Value::String(b)) => Value::String(a + b.as_str()),
            _ => todo!(),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::UInt(a), Value::UInt(b)) => Value::UInt(a - b),
            _ => todo!(),
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::UInt(a), Value::UInt(b)) => Value::UInt(a * b),
            _ => todo!(),
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::UInt(a), Value::UInt(b)) => Value::UInt(a / b),
            _ => todo!(),
        }
    }
}

impl std::ops::Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(a) => Value::Bool(!a),
            _ => todo!(),
        }
    }
}

impl Value {
    pub fn and(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
            _ => todo!(),
        }
    }

    pub fn or(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
            _ => todo!(),
        }
    }

    pub fn xor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a ^ b),
            _ => todo!(),
        }
    }
}
