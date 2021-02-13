use std::ops::{Add, Mul};
use std::convert::TryFrom;
use std::future::Future;
use num::rational::Rational64;




// ============================================================================
#[derive(Debug)]
pub struct InvalidRungeKuttaOrder {}

impl std::fmt::Display for InvalidRungeKuttaOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Runge-Kutta order must be 1, 2, or 3")
    }
}

impl std::error::Error for InvalidRungeKuttaOrder {}




// ============================================================================
pub trait WeightedAverage {
    fn weighted_average(self, br: Rational64, s0: &Self) -> Self;
}

impl<S> WeightedAverage for S where S: Clone + Add<Output=S> + Mul<Rational64, Output=S> {
    fn weighted_average(self, b: Rational64, s0: &Self) -> Self {
        self * (-b + 1) + s0.clone() * b
    }
}




// ============================================================================
pub fn advance_rk1<S: WeightedAverage, Update: Fn(S) -> S>(s0: S, update: Update) -> S {
    update(s0)
}

pub fn advance_rk2<S: Clone + WeightedAverage, Update: Fn(S) -> S>(s0: S, update: Update) -> S {
    let b1 = Rational64::new(1, 2);
    let s1 = s0.clone();
    let s1 = update(s1);
    let s1 = update(s1).weighted_average(b1, &s0);
    s1
}

pub fn advance_rk3<S: Clone + WeightedAverage, Update: Fn(S) -> S>(s0: S, update: Update) -> S {
    let b1 = Rational64::new(3, 4);
    let b2 = Rational64::new(1, 3);
    let s1 = s0.clone();
    let s1 = update(s1);
    let s1 = update(s1).weighted_average(b1, &s0);
    let s1 = update(s1).weighted_average(b2, &s0);
    s1
}




// ============================================================================
#[async_trait::async_trait]
pub trait WeightedAverageAsync: Clone {
    type Runtime;
    async fn weighted_average(self, b: Rational64, s0: &Self, runtime: &Self::Runtime) -> Self;
}




// ============================================================================
pub async fn try_advance_async_rk1<S, R, U, F, E>(state: S, update: U, _runtime: &R) -> Result<S, E>
where
    S: WeightedAverageAsync<Runtime=R>,
    U: Fn(S) -> F,
    F: Future<Output=Result<S, E>>
{
    update(state).await
}

pub async fn advance_async_rk1<S, R, U, F>(state: S, update: U, _runtime: &R) -> S
    where
    S: WeightedAverageAsync<Runtime=R>,
    U: Fn(S) -> F,
    F: Future<Output=S>
{
    update(state).await
}

pub async fn try_advance_async_rk2<S, R, U, F, E>(state: S, update: U, runtime: &R) -> Result<S, E>
where
    S: WeightedAverageAsync<Runtime=R>,
    U: Fn(S) -> F,
    F: Future<Output=Result<S, E>>
{
    let b1 = Rational64::new(1, 2);

    let s1 = state.clone();
    let s1 = update(s1).await?;
    let s1 = update(s1).await?.weighted_average(b1, &state, runtime).await;
    Ok(s1)
}

pub async fn advance_async_rk2<S, R, U, F>(state: S, update: U, runtime: &R) -> S
where
    S: WeightedAverageAsync<Runtime=R>,
    U: Fn(S) -> F,
    F: Future<Output=S>
{
    let b1 = Rational64::new(1, 2);

    let s1 = state.clone();
    let s1 = update(s1).await;
    let s1 = update(s1).await.weighted_average(b1, &state, runtime).await;
    s1
}

pub async fn try_advance_async_rk3<S, R, U, F, E>(state: S, update: U, runtime: &R) -> Result<S, E>
where
    S: WeightedAverageAsync<Runtime=R>,
    U: Fn(S) -> F,
    F: Future<Output=Result<S, E>>
{
    let b1 = Rational64::new(3, 4);
    let b2 = Rational64::new(1, 3);

    let s1 = state.clone();
    let s1 = update(s1).await?;
    let s1 = update(s1).await?.weighted_average(b1, &state, runtime).await;
    let s1 = update(s1).await?.weighted_average(b2, &state, runtime).await;
    Ok(s1)
}

pub async fn advance_async_rk3<S, R, U, F>(state: S, update: U, runtime: &R) -> S
where
    S: WeightedAverageAsync<Runtime=R>,
    U: Fn(S) -> F,
    F: Future<Output=S>
{
    let b1 = Rational64::new(3, 4);
    let b2 = Rational64::new(1, 3);

    let s1 = state.clone();
    let s1 = update(s1).await;
    let s1 = update(s1).await.weighted_average(b1, &state, runtime).await;
    let s1 = update(s1).await.weighted_average(b2, &state, runtime).await;
    s1
}




//============================================================================
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RungeKuttaOrder {
    RK1,
    RK2,
    RK3,
}




//============================================================================
impl TryFrom<i64> for RungeKuttaOrder {

    type Error = InvalidRungeKuttaOrder;

    fn try_from(order: i64) -> Result<RungeKuttaOrder, Self::Error> {
        match order {
            1 => Ok(RungeKuttaOrder::RK1),
            2 => Ok(RungeKuttaOrder::RK2),
            3 => Ok(RungeKuttaOrder::RK3),
            _ => Err(InvalidRungeKuttaOrder{}),
        }
    }
}




//============================================================================
impl RungeKuttaOrder {
    pub fn advance<State, Update>(self, state: State, update: Update) -> State
    where
        State: Clone + WeightedAverage,
        Update: Fn(State) -> State
    {
        match self {
            RungeKuttaOrder::RK1 => advance_rk1(state, update),
            RungeKuttaOrder::RK2 => advance_rk2(state, update),
            RungeKuttaOrder::RK3 => advance_rk3(state, update),
        }
    }

    pub async fn advance_async<S, R, U, F>(self, state: S, update: U, runtime: &R) -> S
    where
        S: WeightedAverageAsync<Runtime=R>,
        U: Fn(S) -> F,
        F: Future<Output=S>
    {
        match self {
            RungeKuttaOrder::RK1 => advance_async_rk1(state, update, runtime).await,
            RungeKuttaOrder::RK2 => advance_async_rk2(state, update, runtime).await,
            RungeKuttaOrder::RK3 => advance_async_rk3(state, update, runtime).await,
        }
    }

    pub async fn try_advance_async<S, R, U, F, E>(self, state: S, update: U, runtime: &R) -> Result<S, E>
    where
        S: WeightedAverageAsync<Runtime=R>,
        U: Fn(S) -> F,
        F: Future<Output=Result<S, E>>
    {
        match self {
            RungeKuttaOrder::RK1 => try_advance_async_rk1(state, update, runtime).await,
            RungeKuttaOrder::RK2 => try_advance_async_rk2(state, update, runtime).await,
            RungeKuttaOrder::RK3 => try_advance_async_rk3(state, update, runtime).await,
        }
    }
}
