type Value = f64;
const VECTOR_LENGTH: usize = 4;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector(pub [Value; VECTOR_LENGTH]);

impl Vector {
    pub fn to_f(&self) -> Value {
        *self.0.get(0).unwrap_or(&0.0)
    }

    pub fn to_f2(&self) -> (Value, Value) {
        (
            *self.0.get(0).unwrap_or(&0.0),
            *self.0.get(1).unwrap_or(&0.0),
        )
    }

    pub fn to_f3(&self) -> (Value, Value, Value) {
        (
            *self.0.get(0).unwrap_or(&0.0),
            *self.0.get(1).unwrap_or(&0.0),
            *self.0.get(2).unwrap_or(&0.0),
        )
    }

    pub fn to_f4(&self) -> (Value, Value, Value, Value) {
        (
            *self.0.get(0).unwrap_or(&0.0),
            *self.0.get(1).unwrap_or(&0.0),
            *self.0.get(2).unwrap_or(&0.0),
            *self.0.get(3).unwrap_or(&0.0),
        )
    }

    pub fn combine_with(&self, rhs: &Self, func: Box<dyn Fn(Value, Value) -> Value>) -> Self {
        let mut result = Self([0.0; VECTOR_LENGTH]);
        for i in 0..(self.0.len().min(rhs.0.len())) {
            result.0[i] = func(
                *self.0.get(i).unwrap_or(&0.0),
                *rhs.0.get(i).unwrap_or(&0.0),
            )
        }
        result
    }

    pub fn scalar(&self, rhs: Value) -> Self {
        let mut vector = self.clone();
        for i in 0..VECTOR_LENGTH {
            vector.0[i] *= rhs;
        }
        vector
    }
}

impl std::ops::Add for &Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        self.combine_with(rhs, Box::new(|a, b| a + b))
    }
}

impl std::ops::Sub for &Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Self::Output {
        self.combine_with(rhs, Box::new(|a, b| a - b))
    }
}

impl std::ops::Mul for &Vector {
    type Output = Vector;
    fn mul(self, rhs: &Vector) -> Self::Output {
        self.combine_with(rhs, Box::new(|a, b| a * b))
    }
}

impl std::ops::Div for &Vector {
    type Output = Vector;
    fn div(self, rhs: &Vector) -> Self::Output {
        self.combine_with(rhs, Box::new(|a, b| a / b))
    }
}

impl From<Value> for Vector {
    fn from(a: Value) -> Self {
        let mut arr = [0.0; VECTOR_LENGTH];
        arr[0] = a;
        Vector(arr)
    }
}

impl From<Vec<Value>> for Vector {
    fn from(a: Vec<Value>) -> Self {
        let mut arr = [0.0; VECTOR_LENGTH];
        for i in 0..(VECTOR_LENGTH.min(a.len())) {
            arr[i] = a[i];
        }
        Vector(arr)
    }
}

#[test]
fn add() {
    let a = Vector::from(vec![0.0, 1.0, 2.0]);
    let b = Vector::from(vec![3.0, 4.0, 5.0]);
    assert_eq!(&a + &b, Vector::from(vec![3.0, 5.0, 7.0]));
    assert_eq!((&a * &b).to_f2(), (0.0, 4.0));
}
