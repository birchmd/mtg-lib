#[derive(Debug, Clone)]
pub enum CounterKind {
    // TODO: +1/+1, -1/-1, ability counters, lore counters, etc.
}

#[derive(Debug, Clone)]
pub struct Counters {
    pub kind: CounterKind,
    pub amount: u32,
}
