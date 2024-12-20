#[derive(Debug)]

pub enum VegiKinds {
  Tomato(Vegi),
  Lemon(Vegi),
  Avocado(Vegi),
}

#[derive(Debug)]
pub struct Vegi {
  pub color: String,
  pub weight: f64
}

