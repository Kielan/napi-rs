use napi::bindgen_prelude::*;

use crate::r#enum::Kind;

/// `constructor` option for `struct` requires all fields to be public,
/// otherwise tag impl fn as constructor
/// #[napi(constructor)]
#[napi]
pub struct Animal {
  #[napi(readonly)]
  pub kind: Kind,
  name: String,
}

#[napi]
impl Animal {
  #[napi(constructor)]
  pub fn new(kind: Kind, name: String) -> Self {
    Animal { kind, name }
  }

  #[napi(factory)]
  pub fn with_kind(kind: Kind) -> Self {
    Animal {
      kind,
      name: "Default".to_owned(),
    }
  }

  #[napi(getter)]
  pub fn get_name(&self) -> &str {
    self.name.as_str()
  }

  #[napi(setter)]
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  #[napi]
  pub fn whoami(&self) -> String {
    match self.kind {
      Kind::Dog => {
        format!("Dog: {}", self.name)
      }
      Kind::Cat => format!("Cat: {}", self.name),
      Kind::Duck => format!("Duck: {}", self.name),
    }
  }

  #[napi]
  pub fn get_dog_kind() -> Kind {
    Kind::Dog
  }
}

/// Smoking test for type generation
#[napi]
#[repr(transparent)]
pub struct Blake2bHasher(u32);

#[napi]
impl Blake2bHasher {
  #[napi(factory)]
  pub fn with_key(key: &Blake2bKey) -> Self {
    Blake2bHasher(key.get_inner())
  }
}

#[napi]
pub struct Blake2bKey(u32);

impl Blake2bKey {
  fn get_inner(&self) -> u32 {
    self.0
  }
}