use std::collections::HashMap;
use crate::semantic::{MTree, Type};

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    Bool(bool),
    Void,
}

impl Value {
    pub fn as_int(&self) -> Result<i32, String> {
        match self {
            Value::Int(i) => Ok(*i),
            _ => Err(format!("Expected Int, found {:?}", self)),
        }
    }

    pub fn as_bool(&self) -> Result<bool, String> {
        match self {
            Value::Bool(b) => Ok(*b),
            _ => Err(format!("Expected Bool, found {:?}", self)),
        }
    }
}

pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn declare(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Variable '{}' not found", name))
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        // Search from innermost to outermost scope
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(format!("Variable '{}' not found", name))
    }
}

pub struct Interpreter {
    env: Environment,
    functions: HashMap<String, (Vec<(String, Type)>, Type, Box<MTree>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            functions: HashMap::new(),
        }
    }

    pub fn execute(&mut self, ast: MTree) -> Result<(), String> {
        // Register all functions
        if let MTree::START { funcs } = &ast {
            for func in funcs {
                if let MTree::FUNC_DECL { name, params, ret_type, body } = func {
                    self.functions.insert(
                        name.clone(),
                        (params.clone(), ret_type.clone(), body.clone()),
                    );
                }
            }
        }

        // Call main
        match self.call_function("main", vec![]) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Runtime error: {}", e)),
        }
    }

    fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        let (params, ret_type, body) = self.functions.get(name)
            .ok_or_else(|| format!("Function '{}' not found", name))?
            .clone();

        // Check argument count
        if params.len() != args.len() {
            return Err(format!(
                "Function '{}' expects {} arguments, got {}",
                name,
                params.len(),
                args.len()
            ));
        }

        // Create new scope 
        self.env.push_scope();

        // Bind params
        for ((param_name, _), arg_value) in params.iter().zip(args.iter()) {
            self.env.declare(param_name.clone(), arg_value.clone());
        }

        // Execute function body
        let result = match self.execute_block(&body)? {
            Some(val) => val,
            None => Value::Void,
        };

        
        self.env.pop_scope();

        Ok(result)
    }

    fn execute_block(&mut self, block: &MTree) -> Result<Option<Value>, String> {
        if let MTree::BLOCK { stmts } = block {
            for stmt in stmts {
                if let Some(ret_val) = self.execute_statement(stmt)? {
                    return Ok(Some(ret_val));
                }
            }
            Ok(None)
        } else {
            Err("Expected BLOCK node".to_string())
        }
    }

    fn execute_statement(&mut self, stmt: &MTree) -> Result<Option<Value>, String> {
        match stmt {
            MTree::LET_STMT { id, ty, expr } => {
                let value = if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    // Default initialization
                    match ty {
                        Type::Int => Value::Int(0),
                        Type::Bool => Value::Bool(false),
                        Type::Unknown => Value::Int(0),
                    }
                };
                self.env.declare(id.clone(), value);
                Ok(None)
            }

            MTree::ASSIGN { id, expr } => {
                let value = self.eval_expr(expr)?;
                self.env.set(id, value)?;
                Ok(None)
            }

            MTree::RTRN_STMT { expr } => {
                let value = self.eval_expr(expr)?;
                Ok(Some(value))
            }

            MTree::IF_STMT { cond, then_block, else_block } => {
                let cond_val = self.eval_expr(cond)?;
                if cond_val.as_bool()? {
                    self.execute_block(then_block)
                } else if let Some(else_b) = else_block {
                    self.execute_block(else_b)
                } else {
                    Ok(None)
                }
            }

            MTree::WHILE_STMT { cond, body } => {
                loop {
                    let cond_val = self.eval_expr(cond)?;
                    if !cond_val.as_bool()? {
                        break;
                    }
                    if let Some(ret_val) = self.execute_block(body)? {
                        return Ok(Some(ret_val));
                    }
                }
                Ok(None)
            }

            MTree::PRINT_STMT { expr } => {
                let value = self.eval_expr(expr)?;
                match value {
                    Value::Int(i) => println!("{}", i),
                    Value::Bool(b) => println!("{}", if b { "true" } else { "false" }),
                    Value::Void => println!("void"),
                }
                Ok(None)
            }

            MTree::BLOCK { .. } => {
                self.execute_block(stmt)
            }

            _ => {
                // Try to evaluate as expression statement
                self.eval_expr(stmt)?;
                Ok(None)
            }
        }
    }

    fn eval_expr(&mut self, expr: &MTree) -> Result<Value, String> {
        match expr {
            MTree::LIT_INT { value } => Ok(Value::Int(*value)),
            
            MTree::LIT_BOOL { value } => Ok(Value::Bool(*value)),

            MTree::ID { name } => self.env.get(name),

            MTree::CALL { name, args } => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg)?);
                }
                self.call_function(name, arg_values)
            }

            MTree::EXPR { left, op, right } => {
                // Handle unary operators
                if op == "!" {
                    let r = self.eval_expr(right)?;
                    return Ok(Value::Bool(!r.as_bool()?));
                }
                if op == "unary-" {
                    let r = self.eval_expr(right)?;
                    return Ok(Value::Int(-r.as_int()?));
                }

                // Binary operators
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;

                match op.as_str() {
                    "+" => Ok(Value::Int(left_val.as_int()? + right_val.as_int()?)),
                    "-" => Ok(Value::Int(left_val.as_int()? - right_val.as_int()?)),
                    "*" => Ok(Value::Int(left_val.as_int()? * right_val.as_int()?)),
                    "/" => {
                        let r = right_val.as_int()?;
                        if r == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::Int(left_val.as_int()? / r))
                    }
                    "==" => {
                        match (left_val, right_val) {
                            (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l == r)),
                            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l == r)),
                            _ => Err("Type mismatch in ==".to_string()),
                        }
                    }
                    "!=" => {
                        match (left_val, right_val) {
                            (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l != r)),
                            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l != r)),
                            _ => Err("Type mismatch in !=".to_string()),
                        }
                    }
                    "<" => Ok(Value::Bool(left_val.as_int()? < right_val.as_int()?)),
                    ">" => Ok(Value::Bool(left_val.as_int()? > right_val.as_int()?)),
                    "<=" => Ok(Value::Bool(left_val.as_int()? <= right_val.as_int()?)),
                    ">=" => Ok(Value::Bool(left_val.as_int()? >= right_val.as_int()?)),
                    "&&" => Ok(Value::Bool(left_val.as_bool()? && right_val.as_bool()?)),
                    "||" => Ok(Value::Bool(left_val.as_bool()? || right_val.as_bool()?)),
                    _ => Err(format!("Unknown operator: {}", op)),
                }
            }

            MTree::ASSIGN { id, expr } => {
                let value = self.eval_expr(expr)?;
                self.env.set(id, value.clone())?;
                Ok(value)
            }

            _ => Err(format!("Cannot evaluate expression: {:?}", expr)),
        }
    }
}