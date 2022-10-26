use crate::runtime::ConditionallySafe;

#[derive(PartialEq, Eq, Clone)]
pub enum Signal<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  Ok(Item),
  Err(Err),
  Complete,
}

impl<Item, Err> std::fmt::Debug for Signal<Item, Err>
where
  Item: ConditionallySafe + std::fmt::Debug,
  Err: ConditionallySafe + std::fmt::Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Ok(arg0) => f.debug_tuple("Ok").field(arg0).finish(),
      Self::Err(arg0) => f.debug_tuple("Err").field(arg0).finish(),
      Self::Complete => write!(f, "Complete"),
    }
  }
}
