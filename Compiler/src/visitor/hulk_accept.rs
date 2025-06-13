use super::hulk_visitor::Visitor;

pub trait Accept {
    fn accept<V: Visitor<T>,T>(&mut self, visitor: &mut V) -> T;
}