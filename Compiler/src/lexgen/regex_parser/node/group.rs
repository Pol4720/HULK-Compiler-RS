/// Representa una agrupación en una expresión regular.
///
/// Una agrupación puede contener cualquier subexpresión válida de regex.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegexGroup<T> {
    /// La subexpresión agrupada
    pub expr: T,
}

impl<T> RegexGroup<T> {
    /// Crea una nueva agrupación a partir de una subexpresión.
    pub fn new(expr: T) -> Self {
        RegexGroup { expr }
    }
    /// Devuelve una referencia a la subexpresión agrupada.
    pub fn inner(&self) -> &T {
        &self.expr
    }
    /// Devuelve la subexpresión agrupada, consumiendo la agrupación.
    pub fn into_inner(self) -> T {
        self.expr
    }
}
