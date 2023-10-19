
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Resource {
    Fire,
    Earth,
    Water,
    Metal,
    Wood,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Cost {
    resource: Resource,
    amount: i32
}

pub type Costs = Vec<Cost>;
