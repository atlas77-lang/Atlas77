use internment::Intern;
use u64 as HlirDataType;

use crate::errors::HlirError;

#[derive(Debug, Clone)]
pub struct ContextScope {
    id: usize,
    value: usize,
}
#[derive(Debug, Clone)]
pub struct ContextVariable {
    name: Intern<String>,
    id: usize,
    data_type: HlirDataType,
    scope: ContextScope,
}
impl ContextVariable {
    pub fn new(name: Intern<String>, id: usize, data_type: HlirDataType, scope: ContextScope) -> ContextVariable {
        ContextVariable {
            name,
            id,
            data_type,
            scope,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ContextFunction {
    name: Intern<String>,
    variables: Vec<ContextVariable>,
    id: usize,
    current_scope: ContextScope,
    next_scope: usize,
    scopes_id: Vec<usize>,
    return_type: HlirDataType,
    next_variable: usize,
}
impl ContextFunction {
    pub fn new(name: Intern<String>, return_type: HlirDataType, id: usize) -> ContextFunction {
        ContextFunction {
            name,
            variables: vec![],
            id,
            current_scope: ContextScope {
                id: 1,
                value: 1,
            },
            next_scope: 2,
            scopes_id: vec![1],
            return_type,
            next_variable: 0,
        }
    }
}

#[derive(Default, Debug)]
pub struct HlirContext {
    functions: Vec<ContextFunction>,
    current_function: usize,
}
impl HlirContext {
    pub fn new() -> HlirContext {
        HlirContext {
            functions: vec![],
            current_function: 0,
        }
    }
    pub fn create_variable(&mut self, name: String, data_type: HlirDataType) -> Result<(), HlirError> {
        for variable in self.functions[self.current_function].variables.iter() {
            if variable.name == name && variable.scope.value >= self.functions[self.current_function].current_scope.value {
                return Err(HlirError::VariableAlreadyExists(name, self.functions[self.current_function].variables.len()));
            }
        }
        let id = self.functions[self.current_function].next_variable;
        let scope = self.functions[self.current_function].current_scope.clone();
        self.functions[self.current_function].variables.push(ContextVariable::new(name, id, data_type, scope));
        self.functions[self.current_function].next_variable += 1;
        Ok(())
    }
    pub fn get_variable_id(&self, identifier: String) -> Result<usize, HlirError> {
        let mut scopes: Vec<usize> = self.functions[self.current_function].scopes_id.clone();
        scopes.push(self.functions[self.current_function].current_scope.id);
        for variable in self.functions[self.current_function].variables.iter() {
            for scope in scopes.iter() {
                if variable.name == identifier && variable.scope.id == *scope {
                    return Ok(variable.id);
                }
            }
        }
        Err(HlirError::VariableNotFound(identifier, 0))
    }
    pub fn variable_exist(&self, identifier: String) -> bool {
        let mut scopes: Vec<usize> = self.functions[self.current_function].scopes_id.clone();
        scopes.push(self.functions[self.current_function].current_scope.id);
        for variable in self.functions[self.current_function].variables.iter() {
            for scope in scopes.iter() {
                if variable.name == identifier && variable.scope.id == *scope {
                    return true;
                }
            }
        }
        false
    }
    pub fn create_scope(&mut self) {
        self.functions[self.current_function].current_scope.id = self.functions[self.current_function].next_scope;
        self.functions[self.current_function].current_scope.value += 1;
        self.functions[self.current_function].next_scope += 1;
    }
    pub fn leave_scope(&mut self) {
        if let Some(id) = self.functions[self.current_function].scopes_id.pop() {
            self.functions[self.current_function].current_scope.id = id;
            self.functions[self.current_function].current_scope.value -= 1;
        }
    }
    pub fn sort_function(&mut self) -> Result<(), HlirError>{
        let mut sorted_functions: Vec<ContextFunction> = Vec::new();
        let mut main_function: Option<ContextFunction> = None;
        let mut index: usize = 0;
        let mut current_id: usize = 0;
    
        while self.functions.len() > 0 {
            let mut function: ContextFunction = self.functions.remove(0);
            if function.name == "main" {
                function.id = 0;
                index = 0;
                current_id = 0;
                main_function = Some(function.clone());
            } else {
                function.id = current_id;
                index += 1;
                current_id += 1;
            }
            sorted_functions.push(function);
        }
        self.functions = sorted_functions;
        if let Some(main_function) = main_function {
            self.functions.insert(0, main_function);
        } else {
            return Err(HlirError::NoMainFunction);
        }
        Ok(())
    }
}