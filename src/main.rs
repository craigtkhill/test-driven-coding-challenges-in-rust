mod binding_and_mutability;
mod scope;
mod shadowing;

fn main() {
    binding_and_mutability::initialize();
    binding_and_mutability::mutable();
    scope::scope();
    scope::define();
    shadowing::shadow();
    println!("Success!");
}
