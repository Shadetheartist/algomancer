
#[derive(Clone)]
pub enum Resource {
    Fire,
    Earth,
    Water,
    Metal,
    Wood,
}

#[derive(Clone)]
pub struct Cost {
    resource: Resource,
    amount: i32
}

pub type Costs = Vec<Cost>;
