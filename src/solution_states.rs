use core::ops::{Add, Mul};
use ndarray::{Array, Dimension};
use num::rational::Rational64;
use num::ToPrimitive;




//============================================================================
#[derive(Clone)]
pub struct SolutionStateArray<T, D: Dimension>
{
    pub time: f64,
    pub iteration: Rational64,
    pub conserved: Array<T, D>,
}

impl<T, D: Dimension> Add for SolutionStateArray<T, D> where Array<T, D>: Add<Array<T, D>, Output=Array<T, D>>
{
    type Output = SolutionStateArray<T, D>;

    fn add(self, b: Self::Output) -> Self::Output
    {
        Self::Output{
            time: self.time + b.time,
            iteration: self.iteration + b.iteration,
            conserved: self.conserved + b.conserved,
        }
    }
}

impl<T, D: Dimension> Mul<Rational64> for SolutionStateArray<T, D> where Array<T, D>: Mul<f64, Output=Array<T, D>>
{
    type Output = SolutionStateArray<T, D>;

    fn mul(self, b: Rational64) -> Self::Output
    {
        Self::Output{
            time: self.time * b.to_f64().unwrap(),
            iteration: self.iteration * b,
            conserved: self.conserved * b.to_f64().unwrap(),
        }
    }
}
