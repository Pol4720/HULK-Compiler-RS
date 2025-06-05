use inkwell::values::BasicValueEnum;
use std::collections::HashMap;

#[derive(Default)]
pub struct ValueMap<'ctx> {
    symbols: HashMap<String, BasicValueEnum<'ctx>>,
}

impl<'ctx> ValueMap<'ctx> {
    pub fn insert(&mut self, name: String, val: BasicValueEnum<'ctx>) {
        self.symbols.insert(name, val);
    }

    pub fn get(&self, name: &str) -> Option<&BasicValueEnum<'ctx>> {
        self.symbols.get(name)
    }
}
