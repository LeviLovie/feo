use crate::api::register_apis;
use rhai::{Engine, EvalAltResult, Scope, AST};

pub struct ScriptEngine {
    engine: Engine,
    ast: Option<AST>,
    scope: Scope<'static>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        register_apis(&mut engine);

        Self {
            engine,
            ast: None,
            scope: Scope::new(),
        }
    }

    pub fn load_script(&mut self, code: &str) -> Result<(), Box<EvalAltResult>> {
        let ast = self.engine.compile(code)?;
        self.ast = Some(ast);
        Ok(())
    }

    pub fn call_init(&mut self) {
        if let Some(ast) = &self.ast {
            match self.engine.call_fn::<()>(&mut self.scope, ast, "init", ()) {
                Err(e) => {
                    println!("Error calling init: {}", e);
                }
                _ => {}
            }
        }
    }

    pub fn call_update(&mut self, dt: f32) {
        if let Some(ast) = &self.ast {
            match self
                .engine
                .call_fn::<()>(&mut self.scope, ast, "update", (dt,))
            {
                Err(e) => {
                    println!("Error calling update: {}", e);
                }
                _ => {}
            }
        }
    }

    pub fn call_draw(&mut self) {
        if let Some(ast) = &self.ast {
            match self.engine.call_fn::<()>(&mut self.scope, ast, "draw", ()) {
                Err(e) => {
                    println!("Error calling draw: {}", e);
                }
                _ => {}
            }
        }
    }
}
