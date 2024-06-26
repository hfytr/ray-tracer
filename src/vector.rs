use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

pub trait VecElem:
    Copy + Add<Output = Self> + Mul<Output = Self> + Default + MulAssign<Self> + AddAssign<Self>
{
}

pub trait SignedVecElem: VecElem + Neg<Output = Self> + Sub<Output = Self> {}

impl<T> VecElem for T where
    T: Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Default
        + MulAssign<Self>
        + AddAssign<Self>
{
}

impl<T> SignedVecElem for T where
    T: Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Default
        + Neg<Output = T>
        + MulAssign<Self>
        + AddAssign<Self>
{
}

#[derive(Debug, Default, Clone)]
pub struct Vector3<T> {
    v: [T; 3],
}

impl<T: VecElem> Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl<T: VecElem> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl<T: VecElem> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { v: [x, y, z] }
    }

    pub fn mul_element_wise(&mut self, rhs: &Vector3<T>) {
        self.v[0] *= rhs.v[0];
        self.v[1] *= rhs.v[1];
        self.v[2] *= rhs.v[2];
    }

    pub fn apply<U: VecElem>(&self, f: fn(T) -> U) -> Vector3<U> {
        Vector3::new(f(self.v[0]), f(self.v[1]), f(self.v[2]))
    }

    pub fn as_vec(&self) -> Vec<T> {
        self.v.to_vec()
    }

    pub fn dot(&self, rhs: &Vector3<T>) -> T {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn x(&self) -> T {
        self[0]
    }
    pub fn y(&self) -> T {
        self[1]
    }
    pub fn z(&self) -> T {
        self[2]
    }
}

impl<T: SignedVecElem> Vector3<T> {
    pub fn cross(&self, rhs: &Vector3<T>) -> Vector3<T> {
        Vector3::new(
            -(self[1] * rhs[2] - self[2] * rhs[1]),
            -(self[2] * rhs[0] - self[0] * rhs[2]),
            -(self[0] * rhs[1] - self[1] * rhs[0]),
        )
    }
}

impl Vector3<f64> {
    pub fn len(&self) -> f64 {
        (self.v[0].powi(2) + self.v[1].powi(2) + self.v[2].powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        for x in self.v.iter_mut() {
            *x /= len;
        }
    }
}

macro_rules! vec_add {
    ($type: ty) => {
        type Output = Vector3<T>;
        fn add(self, rhs: $type) -> Self::Output {
            Vector3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
        }
    };
}
macro_rules! vec_sub {
    ($type: ty) => {
        type Output = Vector3<T>;
        fn sub(self, rhs: $type) -> Self::Output {
            Vector3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
        }
    };
}
macro_rules! vec_mul {
    () => {
        type Output = Vector3<T>;
        fn mul(self, rhs: T) -> Self::Output {
            Vector3::<T>::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
        }
    };
}
macro_rules! vec_add_assign {
    ($type: ty) => {
        fn add_assign(&mut self, rhs: $type) {
            self[0] += rhs[0];
            self[1] += rhs[1];
            self[2] += rhs[2];
        }
    };
}
macro_rules! vec_mul_assign {
    () => {
        fn mul_assign(&mut self, rhs: T) {
            for x in self.v.iter_mut() {
                *x *= rhs;
            }
        }
    };
}

impl<T: VecElem> Add<Vector3<T>> for Vector3<T> {
    vec_add!(Vector3<T>);
}

impl<T: VecElem> Add<Vector3<T>> for &Vector3<T> {
    vec_add!(Vector3<T>);
}

impl<T: VecElem> Add<&Vector3<T>> for Vector3<T> {
    vec_add!(&Vector3<T>);
}

impl<T: VecElem> Add<&Vector3<T>> for &Vector3<T> {
    vec_add!(&Vector3<T>);
}

impl<T: SignedVecElem> Sub<Vector3<T>> for Vector3<T> {
    vec_sub!(Vector3<T>);
}

impl<T: SignedVecElem> Sub<Vector3<T>> for &Vector3<T> {
    vec_sub!(Vector3<T>);
}

impl<T: SignedVecElem> Sub<&Vector3<T>> for Vector3<T> {
    vec_sub!(&Vector3<T>);
}

impl<T: SignedVecElem> Sub<&Vector3<T>> for &Vector3<T> {
    vec_sub!(&Vector3<T>);
}

impl<T: VecElem> Mul<T> for Vector3<T> {
    vec_mul!();
}

impl<T: VecElem> Mul<T> for &Vector3<T> {
    vec_mul!();
}

impl<T: VecElem> MulAssign<T> for Vector3<T> {
    vec_mul_assign!();
}

impl<T: VecElem> MulAssign<T> for &mut Vector3<T> {
    vec_mul_assign!();
}

impl<T: VecElem> AddAssign<Vector3<T>> for Vector3<T> {
    vec_add_assign!(Vector3<T>);
}

impl<T: VecElem> AddAssign<Vector3<T>> for &mut Vector3<T> {
    vec_add_assign!(Vector3<T>);
}

impl<T: VecElem> AddAssign<&Vector3<T>> for Vector3<T> {
    vec_add_assign!(&Vector3<T>);
}

impl<T: VecElem> AddAssign<&Vector3<T>> for &mut Vector3<T> {
    vec_add_assign!(&Vector3<T>);
}
