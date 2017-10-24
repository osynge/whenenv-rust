use std::collections::HashMap;

pub struct depneacyPair {
    key: string,
    value: string
}

pub struct depneacy {
    key: string,
    value: string
}

pub struct JobModule {
    name: string,
    depneacyPair: Vec<depneacyPair>,
    depneacy: Vec<depneacy>,
    filename: string,
}

trait JobModuleTrait {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}


// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> JobModule {
        depneacyPair = Vec<depneacyPair>;
        depneacy = Vec<depneacyPair>;
        JobModule { 
            name: name, 
            depneacyPair: depneacyPair,
            depneacy: depneacy,
             }
    }
