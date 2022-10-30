pub trait Len {
    fn len(&self) -> f64;
}

pub trait LenSq {
    fn len_sq(&self) -> f64;
}

pub trait Dot<Rhs> {
    fn dot(&self,rhs: Rhs) -> f64;
}

pub trait Cross<Rhs> {
    type Output;
    fn cross(&self,rhs: Rhs) -> Self::Output;
}

pub trait Normalize {
    type Output;
    fn normalize(&self) -> Result<Self::Output,String>;
}
