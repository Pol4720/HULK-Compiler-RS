use super::hulk_visitor::Visitor;

pub trait Accept {
    fn accept<V: Visitor<T>,T>(&self, visitor: &mut V) -> T;
}