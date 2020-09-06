use std::ops::{Add, Mul};
use std::convert::TryFrom;
use num::rational::Rational64;




// ============================================================================
#[derive(Debug)]
pub struct InvalidRungeKuttaOrder {}

impl std::fmt::Display for InvalidRungeKuttaOrder
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Runge-Kutta order must be 1, 2, or 3")
    }
}

impl std::error::Error for InvalidRungeKuttaOrder {}




//============================================================================
fn advance_rk1<State, Update>(s0: State, update: Update) -> State where State: Clone + Add<Output=State> + Mul<Rational64, Output=State>, Update: Fn(State) -> State
{
    update(s0)
}

fn advance_rk2<State, Update>(s0: State, update: Update) -> State where State: Clone + Add<Output=State> + Mul<Rational64, Output=State>, Update: Fn(State) -> State
{
    let b1 = Rational64::new(1, 2);
    let mut s1 = s0.clone();
    s1 =           update(s1);
    s1 = s0 * b1 + update(s1) * (-b1 + 1);
    s1
}

fn advance_rk3<State, Update>(s0: State, update: Update) -> State where State: Clone + Add<Output=State> + Mul<Rational64, Output=State>, Update: Fn(State) -> State
{
    let b1 = Rational64::new(3, 4);
    let b2 = Rational64::new(1, 3);
    let mut s1 = s0.clone();
    s1 =                   update(s1);
    s1 = s0.clone() * b1 + update(s1) * (-b1 + 1);
    s1 = s0.clone() * b2 + update(s1) * (-b2 + 1);
    s1
}




//============================================================================
#[derive(Clone, Copy, Debug)]
pub enum RungeKuttaOrder
{
    RK1,
    RK2,
    RK3,
}




//============================================================================
impl TryFrom<i64> for RungeKuttaOrder
{
    type Error = InvalidRungeKuttaOrder;
    fn try_from(order: i64) -> Result<RungeKuttaOrder, Self::Error>
    {
        match order {
            1 => Ok(RungeKuttaOrder::RK1),
            2 => Ok(RungeKuttaOrder::RK2),
            3 => Ok(RungeKuttaOrder::RK3),
            _ => Err(InvalidRungeKuttaOrder{}),
        }
    }
}




//============================================================================
impl RungeKuttaOrder
{
    pub fn advance<State, Update>(self, state: State, update: Update) -> State where State: Clone + Add<Output=State> + Mul<Rational64, Output=State>, Update: Fn(State) -> State
    {
        match self {
            RungeKuttaOrder::RK1 => advance_rk1(state, update),        
            RungeKuttaOrder::RK2 => advance_rk2(state, update),        
            RungeKuttaOrder::RK3 => advance_rk3(state, update),        
        }
    }
}
