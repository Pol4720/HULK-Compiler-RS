// Pruebas para el generador de analizadores léxicos
#[cfg(test)]
mod tests {
    use super::generator::*;

    #[test]
    fn test_read_token_spec() {
        // TODO: Agregar pruebas para la lectura de la especificación
        assert!(read_token_spec("test_spec.txt").is_empty());
    }
}
