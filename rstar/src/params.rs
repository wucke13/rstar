use typenum::{Unsigned, U3, U6, U1};
use rtree::InsertionStrategy;
use ::std::marker::PhantomData;
use rstar::RStarInsertionStrategy;

pub trait RTreeParams {
    type MinSize: Unsigned;
    type MaxSize: Unsigned;
    type ReinsertionCount: Unsigned;
    type DefaultInsertionStrategy: InsertionStrategy;

    fn debug_output() -> String {
        format!("MinSize: {}, MaxSize: {}, ReinsertionCount: {}", 
                Self::MinSize::to_usize(), 
                Self::MaxSize::to_usize(), 
                Self::ReinsertionCount::to_usize())
    }
}



enum Void {}

pub struct CustomParams<MinSize, MaxSize, ReinsertionCount, DefaultInsertionStrategy> where
    MinSize: Unsigned,
    MaxSize: Unsigned,
    ReinsertionCount: Unsigned,
    DefaultInsertionStrategy: InsertionStrategy {
    _min_size: PhantomData<MinSize>,
    _max_size: PhantomData<MaxSize>,
    _reinsertion_count: PhantomData<ReinsertionCount>,
    _default_insertion_strategy: PhantomData<DefaultInsertionStrategy>,
    _void: Void,
}

impl <MinSize, MaxSize, ReinsertionCount, DefaultInsertionStrategy> RTreeParams
    for CustomParams<MinSize, MaxSize, ReinsertionCount, DefaultInsertionStrategy> where
    MinSize: Unsigned,
    MaxSize: Unsigned,
    ReinsertionCount: Unsigned,
    DefaultInsertionStrategy: InsertionStrategy,
{
    type MinSize = MinSize;
    type MaxSize = MaxSize;
    type ReinsertionCount = ReinsertionCount;
    type DefaultInsertionStrategy = DefaultInsertionStrategy;
}

pub type DefaultParams = CustomParams<U3, U6, U1, RStarInsertionStrategy>;
