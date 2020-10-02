mod ast;
mod compiler;
mod envelope;
mod parser;
mod vector;

// TODO:
// - Stack based vectors
// - Marker lists
// - Arithmetics
// - Randoms

fn main() {
    let script = std::fs::read_to_string("example.bs").expect("Could not read example.bs");
    println!("SCRIPT:\n\n{}\n\n", script);

    let ast = match parser::parse(&script) {
        Ok(expr) => expr,
        Err(error) => panic!(format!("Could not parse: {:?}", error)),
    };
    println!("AST:\n\n{:?}\n\n", ast);

    let output = match compiler::build(ast) {
        Ok(exports) => exports,
        Err(err) => panic!(format!("Could not compile: {:?}", err)),
    };
    for i in 0..50 {
        let t = i as f64 / 10.0;
        println!("At {}:", t);
        for (key, envelope) in output.iter() {
            println!("  {}: {:?}", key, envelope.get_value(t));
        }
    }
}
