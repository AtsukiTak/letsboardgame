use cgmath::Array;
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepVec<A>
where
    A: Array,
{
    vec: Vec<A::Element>,
    _arr: PhantomData<A>,
}

impl<A> StepVec<A>
where
    A: Array,
{
    pub fn new() -> Self {
        StepVec {
            vec: Vec::new(),
            _arr: PhantomData,
        }
    }

    pub fn step() -> usize {
        A::len()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, array: A) {
        for i in 0..A::len() {
            self.vec.push(array[i]);
        }
    }
}

impl<A> AsRef<[A::Element]> for StepVec<A>
where
    A: Array,
{
    fn as_ref(&self) -> &[A::Element] {
        self.vec.as_ref()
    }
}
