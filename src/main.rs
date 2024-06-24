mod binding_and_mutability;
mod scope;
mod shadowing;
mod unused_variables;

fn main() {
    binding_and_mutability::initialize();
    binding_and_mutability::mutable();
    scope::scope();
    scope::define();
    shadowing::shadow();
    shadowing::rebind();
    unused_variables::unused_variable();
    println!("Success!");
}
