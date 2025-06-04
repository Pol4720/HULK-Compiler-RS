use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// Rc<RefCell<TypeNode>> para hijos, variables y métodos:

// Rc (Reference Counted): Permite que varios nodos compartan la propiedad de un hijo, variable o método.
// RefCell: Permite mutar el contenido incluso si tienes referencias compartidas.
// Así puedes modificar hijos, variables y métodos en cualquier parte del árbol.
// Weak<RefCell<TypeNode>> para el padre:

// Weak es una referencia débil, que no incrementa el contador de referencias de Rc.
// Esto evita ciclos de referencia (memory leaks) entre padres e hijos.
// Si el padre se elimina, el hijo puede detectar que su padre ya no existe.

#[derive(Debug, Clone)]
pub struct TypeNode {
    pub type_name: String,
    pub depth: i32,
    pub parent: Option<Weak<RefCell<TypeNode>>>,
    pub children: Vec<Rc<RefCell<TypeNode>>>,
    pub variables: HashMap<String, Rc<RefCell<TypeNode>>>,
    pub methods: HashMap<String, Rc<RefCell<TypeNode>>>,
}

// Manual PartialEq implementation that ignores the parent field
impl PartialEq for TypeNode {
    fn eq(&self, other: &Self) -> bool {
        self.type_name == other.type_name
            && self.depth == other.depth
            // parent is ignored or you can implement custom logic if needed
            && self.children == other.children
            && self.variables == other.variables
            && self.methods == other.methods
    }
}

impl TypeNode {
    pub fn new(type_name: String, depth: i32, parent: Option<Weak<RefCell<TypeNode>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TypeNode {
            type_name,
            depth,
            parent,
            children: Vec::new(),
            variables: HashMap::new(),
            methods: HashMap::new(),
        }))
    }

    pub fn add_child(parent: &Rc<RefCell<TypeNode>>, child: Rc<RefCell<TypeNode>>) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(child);
    }

    pub fn add_variable(&mut self, name: String, variable: Rc<RefCell<TypeNode>>) {
        self.variables.insert(name, variable);
    }

    pub fn add_method(&mut self, name: String, method: Rc<RefCell<TypeNode>>) {
        self.methods.insert(name, method);
    }
}