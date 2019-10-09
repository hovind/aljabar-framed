#[cfg(test)]
mod tests {
    use crate::{Frame, frame, Vec};
    #[test]
    fn it_works() {
        frame!(World);
        frame!(Cat);
        frame!(Dog);
        let cat = Vec::<f64, World, World, Cat>::new(3.0f64);
        let cat2dog = Vec::<f64, World, Cat, Dog>::new(2.0f64);
        let _dog = cat + cat2dog;
        /*
         * let _err = cat2dog + cat;
         */
        assert_eq!(2 + 2, 4);
    }

}

use std::ops::{Add, Sub, Neg};

pub trait Frame {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec<T, In, Fr, To>
where
    In: Frame,
    Fr: Frame,
    To: Frame,
{
    a: T,
    _in: std::marker::PhantomData<In>,
    _from: std::marker::PhantomData<Fr>,
    _to: std::marker::PhantomData<To>,
}

impl <T, In, Fr, To> Vec<T, In, Fr, To>
where
    In: Frame,
    Fr: Frame,
    To: Frame,
{
    pub fn new(x: T) -> Vec<T, In, Fr, To> {
        Vec {
            a: x,
            _in: std::marker::PhantomData,
            _from: std::marker::PhantomData,
            _to: std::marker::PhantomData,
        }
    }
}

impl<T, In, Fr, Be, To> Add<Vec<T, In, Be, To>> for Vec<T, In, Fr, Be>
where
    T: Add<T, Output = T>,
    In: Frame,
    Fr: Frame,
    Be: Frame,
    To: Frame,
{
    type Output = Vec<T, In, Fr, To>;

    fn add(self, other: Vec<T, In, Be, To>) -> Self::Output {
        Vec {
            a: self.a + other.a,
            _in: self._in,
            _from: self._from,
            _to: other._to,
        }
    }
}

impl<T, In, Fr, Be, To> Sub<Vec<T, In, To, Be>> for Vec<T, In, Fr, Be>
where
    T: Sub<T, Output = T>,
    In: Frame,
    Fr: Frame,
    Be: Frame,
    To: Frame,
{
    type Output = Vec<T, In, Fr, To>;

    fn sub(self, other: Vec<T, In, To, Be>) -> Self::Output {
        Vec {
            a: self.a - other.a,
            _in: self._in,
            _from: self._from,
            _to: other._from,
        }
    }
}

impl<T, In, Fr, To> Neg for Vec<T, In, Fr, To>
where
    T: Neg<Output = T>,
    In: Frame,
    Fr: Frame,
    To: Frame,
{
    type Output = Vec<T, In, To, Fr>;

    fn neg(self) -> Self::Output {
        Vec {
            a: -self.a,
            _in: self._in,
            _from: self._to,
            _to: self._from,
        }
    }
}

#[macro_export]
macro_rules! frame {
    ($x:ident) => {
        enum $x {};
        impl Frame for $x {};
    };
}

