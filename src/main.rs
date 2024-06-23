mod binding_and_mutability;
mod scope;

fn main() {
    binding_and_mutability::initialize();
    binding_and_mutability::mutable();
    scope::scope();
    scope::define();
    println!("Success!");
}
