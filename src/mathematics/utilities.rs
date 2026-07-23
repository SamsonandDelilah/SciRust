//
//   --- mylib\src\mathematics\utilities.rs als Library siehe lib.rs ---
//

// Erforderliche Einbindung 
use std::any::type_name;        // für type_of...
//use std::fmt::Display;          // für print

//
//   --- Definition fn type_of ---
//
    pub fn type_of<T>(_: &T) {
        println!("Type is {}", type_name::<T>());
    }




//
//   --- Unit-Test ---
//
//   Nur bei direktem Ausführen: main-Funktion mit cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use super::*;
    //use std::io::{self, Write};
    //use std::sync::{Arc, Mutex};
    //use std::str;

    //
    //   --- Test fn type_of ---
    //

    #[test] // Integer i32
    fn test_type_of_i32() {
        let val:i32 = 42;
        type_of(&val);
        // Erwartete Ausgabe: "i32"
    }

    #[test] // Float f64
    fn test_type_of_f64() {
        let valf:f64 = 42.0;
        type_of(&valf);
        // Erwartete Ausgabe: "if64"
    }

    #[test] // String
    fn test_type_of_str() {
        let str = "hello";
        type_of(&str);
        // Erwartete Ausgabe: "&str"
    }

    #[test] // Vector
    fn test_type_of_vec() {
        let val = vec![1, 2, 3];
        type_of(&val);
        // Erwartete Ausgabe: "alloc::vec::Vec<i32>"
    }

    //
    //   --- End Test fn type_of ---
    //
}