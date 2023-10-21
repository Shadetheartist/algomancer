use crate::game::state::effect::Effect;

struct StackEffect {
    source: i32,
    effect: Effect
}

struct Stack {
    entries: Vec<StackEffect>
}


impl Stack {
    fn add(entry: StackEffect) {}
    fn next() {}
    fn resolve_next() {}
}
