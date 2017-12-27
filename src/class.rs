use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use callable::{LoxCallable, LoxFunction};
use instance::LoxInstance;
use interpreter::Interpreter;
use primitive::{Value, ValueResult};

type MethodMap = HashMap<String, Rc<LoxFunction>>;

#[derive(Clone, Debug)]
pub struct LoxClass {
    name: String,
    superclass: Option<Rc<LoxClass>>,
    methods: Rc<MethodMap>,
}

impl LoxClass {
    pub fn new(name: String, superclass: Option<Rc<LoxClass>>, methods: MethodMap) -> Self {
        Self {
            name,
            superclass,
            methods: Rc::new(methods),
        }
    }

    pub fn get_method(&self, name: &str, instance: Rc<LoxInstance>) -> Option<Rc<LoxFunction>> {
        if let Some(method) = self.methods.get(name) {
            return Some(Rc::new(method.bind(instance)));
        }

        if let Some(ref superclass) = self.superclass {
            superclass.get_method(name, instance)
        } else {
            None
        }
    }

    fn initializer(&self) -> Option<Rc<LoxFunction>> {
        self.methods.get("init").cloned()
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> ValueResult {
        let instance = Rc::new(LoxInstance::new(self.clone()));

        if let Some(initializer) = self.initializer() {
            initializer
                .bind(Rc::clone(&instance))
                .call(interpreter, arguments)?;
        }

        Ok(Value::Instance(instance))
    }

    fn arity(&self) -> usize {
        self.initializer()
            .map(|initializer| initializer.arity())
            .unwrap_or(0)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
