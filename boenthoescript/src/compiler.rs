use crate::{ast::*, envelope, envelope::Envelope};
use std::collections::HashMap;

type Number = f64;
pub type EnvelopeFn = Box<dyn Envelope>;

impl std::fmt::Debug for EnvelopeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Envelope").finish()
    }
}

#[derive(Debug)]
pub enum Build {
    Symbol(String),
    NumberList(Vec<Number>),
    EnvelopeFn(EnvelopeFn),
    Partial(Expr),
    Nil,
}

#[derive(Debug)]
pub enum BuildError {
    InvalidType { expected: String, actual: Build },
    FunctionNotFound(String),
    VariableNotFound(String),
    MissingArgument,
    NotPartial(Expr),
}

type BuildResult = Result<Build, BuildError>;

#[derive(Debug, Clone)]
struct Variable {
    expr: Expr,
    is_export: bool,
}
type Env = HashMap<String, Variable>;

pub fn build(expr: Expr) -> Result<HashMap<String, EnvelopeFn>, BuildError> {
    let mut env = Env::new();
    compile(expr, &mut env)?;

    let mut exports = HashMap::new();
    for (name, export) in env.iter().filter(|(_, var)| var.is_export) {
        exports.insert(
            name.clone(),
            match compile(export.expr.clone(), &mut env.clone())? {
                Build::EnvelopeFn(f) => f,
                x => {
                    return Err(BuildError::InvalidType {
                        expected: "Envelope function".into(),
                        actual: x,
                    })
                }
            },
        );
    }
    Ok(exports)
}

fn compile(expr: Expr, env: &mut Env) -> BuildResult {
    match expr {
        Expr::Symbol(s) => match env.get(&s) {
            Some(var) => compile(var.expr.clone(), env),
            None => Ok(Build::Symbol(s.clone())),
        },
        Expr::NumberList(n) => Ok(Build::NumberList(n.clone())),
        Expr::List(name, cons) => list(&name, cons, env),
        Expr::Comment(_) => Ok(Build::Nil),
    }
}

fn list(name: &str, cons: Vec<Expr>, env: &mut Env) -> BuildResult {
    match name {
        KW_ROOT => block(cons, env),
        KW_BLOCK => block(cons, &mut env.clone()),
        KW_DEFINE => define(cons, env, false),
        KW_EXPORT => define(cons, env, true),
        KW_ARRAY => number_list(cons, env),

        STD_HOLD => hold(cons, env),
        STD_LINEAR => linear(cons, env),
        STD_CONCAT => concat(cons, env),
        STD_REPEAT => repeat(cons, env),
        STD_LOOP => inf_loop(cons, env),

        _ => match env.get(name) {
            Some(var) => match var.expr.clone() {
                Expr::List(sub_name, sub_cons) => {
                    let mut merged_cons = sub_cons.clone();
                    merged_cons.append(&mut cons.clone());
                    compile(Expr::list(&sub_name, merged_cons), env)
                }
                x => Err(BuildError::NotPartial(x)),
            },
            None => Err(BuildError::FunctionNotFound(name.into())),
        },
    }
}

fn block(cons: Vec<Expr>, env: &mut Env) -> BuildResult {
    let (init, last) = cons.split_at(cons.len() - 1);
    for expr in init.iter() {
        compile(expr.clone(), env)?;
    }
    match last.first() {
        Some(expr) => compile(expr.clone(), env),
        None => Ok(Build::Nil),
    }
}

fn define(cons: Vec<Expr>, env: &mut Env, is_export: bool) -> BuildResult {
    let name = arg_symbol(cons.get(0), env)?;
    let expr = arg_expr(cons.get(1))?;
    env.insert(
        name.clone(),
        Variable {
            expr: expr.clone(),
            is_export,
        },
    );
    Ok(Build::Partial(expr))
}

fn number_list(cons: Vec<Expr>, env: &mut Env) -> BuildResult {
    let mut arr = Vec::new();
    for expr in cons.iter() {
        match compile(expr.clone(), env)? {
            Build::NumberList(list) => arr.append(list.clone().as_mut()),
            x => {
                return Err(BuildError::InvalidType {
                    expected: "NumberList".into(),
                    actual: x,
                })
            }
        }
    }
    Ok(Build::NumberList(arr))
}

fn arg_symbol(expr: Option<&Expr>, env: &mut Env) -> Result<String, BuildError> {
    match expr {
        Some(e) => match compile(e.clone(), env)? {
            Build::Symbol(str) => Ok(str),
            x => {
                return Err(BuildError::InvalidType {
                    expected: "Symbol".into(),
                    actual: x,
                })
            }
        },
        None => Err(BuildError::MissingArgument),
    }
}

fn arg_number(expr: Option<&Expr>, env: &mut Env) -> Result<Number, BuildError> {
    match expr {
        Some(e) => match compile(e.clone(), env)? {
            Build::NumberList(n) if n.len() == 1 => Ok(n[0]),
            Build::Symbol(k) => Err(BuildError::VariableNotFound(k)),
            x => {
                return Err(BuildError::InvalidType {
                    expected: "Number".into(),
                    actual: x,
                })
            }
        },
        None => Err(BuildError::MissingArgument),
    }
}

fn arg_number_list(expr: Option<&Expr>, env: &mut Env) -> Result<Vec<Number>, BuildError> {
    match expr {
        Some(e) => match compile(e.clone(), env)? {
            Build::NumberList(a) => Ok(a),
            Build::Symbol(k) => Err(BuildError::VariableNotFound(k)),
            x => {
                return Err(BuildError::InvalidType {
                    expected: "NumberList".into(),
                    actual: x,
                })
            }
        },
        None => Err(BuildError::MissingArgument),
    }
}

fn arg_expr(expr: Option<&Expr>) -> Result<Expr, BuildError> {
    match expr {
        Some(e) => Ok(e.clone()),
        None => Err(BuildError::MissingArgument),
    }
}

fn arg_envelope_fn(expr: Option<&Expr>, env: &mut Env) -> Result<EnvelopeFn, BuildError> {
    match expr {
        Some(e) => match compile(e.clone(), env)? {
            Build::EnvelopeFn(f) => Ok(f),
            Build::Symbol(k) => Err(BuildError::VariableNotFound(k)),
            x => {
                return Err(BuildError::InvalidType {
                    expected: "EnvelopeFunction".into(),
                    actual: x,
                })
            }
        },
        None => Err(BuildError::MissingArgument),
    }
}

// Standard library

const STD_HOLD: &str = "hold";
fn hold(cons: Vec<Expr>, env: &mut Env) -> BuildResult {
    let value = arg_number_list(cons.get(0), env)?;
    let duration = arg_number(cons.get(1), env)?;

    Ok(Build::EnvelopeFn(Box::new(envelope::Hold::new(
        duration,
        value.into(),
    ))))
}

const STD_LINEAR: &str = "linear";
fn linear(cons: Vec<Expr>, env: &mut Env) -> BuildResult {
    let from = arg_number_list(cons.get(0), env)?;
    let to = arg_number_list(cons.get(1), env)?;
    let duration = arg_number(cons.get(2), env)?;

    Ok(Build::EnvelopeFn(Box::new(envelope::Linear::new(
        duration,
        from.into(),
        to.into(),
    ))))
}

const STD_CONCAT: &str = "concat";
fn concat(cons: Vec<Expr>, env: &mut Env) -> BuildResult {
    let mut fns = Vec::new();
    for expr in cons.iter() {
        match compile(expr.clone(), env)? {
            Build::EnvelopeFn(f) => fns.push(f),
            actual => {
                return Err(BuildError::InvalidType {
                    expected: "EnvelopeFunction".into(),
                    actual,
                })
            }
        }
    }

    Ok(Build::EnvelopeFn(Box::new(envelope::Concat::new(fns))))
}

const STD_REPEAT: &str = "repeat";
fn repeat(cons: Vec<Expr>, env: &mut Env) -> BuildResult {
    let repeats = arg_number(cons.get(0), env)? as u32;
    let envelope_fn = arg_envelope_fn(cons.get(1), env)?;
    Ok(Build::EnvelopeFn(Box::new(envelope::Repeat::new(
        repeats,
        envelope_fn,
    ))))
}

const STD_LOOP: &str = "loop";
fn inf_loop(cons: Vec<Expr>, env: &mut Env) -> BuildResult {
    let envelope_fn = arg_envelope_fn(cons.get(0), env)?;
    Ok(Build::EnvelopeFn(Box::new(envelope::Loop::new(
        envelope_fn,
    ))))
}

#[test]
fn build1() {
    use crate::ast::Expr;

    let ast = Expr::list(
        KW_BLOCK,
        vec![
            Expr::list(KW_DEFINE, vec!["length".into(), 4.0.into()]),
            Expr::list(KW_DEFINE, vec!["value".into(), 1.0.into()]),
            Expr::list("hold", vec!["length".into(), "value".into()]),
        ],
    );

    let mut env = HashMap::new();
    let result = compile(ast, &mut env);

    match result {
        Ok(c) => match c {
            Build::EnvelopeFn(f) => {
                assert_eq!(f.get_value(0.0).to_f(), 1.0);
                assert_eq!(f.get_duration(), 4.0);
            }
            _ => panic!("Not a compiled function"),
        },
        _ => panic!("Failure of life"),
    }
}
