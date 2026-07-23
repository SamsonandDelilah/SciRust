//
//   --- src\main.rs ---
//

// Erforderliche Module importieren 
use Library::utilities::*;
use Library::prefix::SIPrefix;
use play::SIPrefix::SIPrefix::*;          // Beispiel mit Zugriff auf exteren lib

fn main() {
    println!("\n{}", "Hallo, code is running!\n");

    type_of(&34);
    type_of(&vec![1.0, 2.0 , 3.0]);

    
    println!("{}", Nano);
    type_of(&Nano);

    let val = Kilo;
    println!("Multiplier für {}: {}", val, val.multiplier());
 
    // Alle auflisten
    for i in SIPrefix::all() {
        println!("{}", i.prefix_to_string());
    }
    
    // Speziellen Prefix suchen
    if let Some(val) = SIPrefix::prefix_find_by_name("nano") {
        println!("Gefunden: {}", val.prefix_to_string());
    } else {
        println!("Nicht gefunden");
    }

    if let Some(val) = SIPrefix::prefix_find_by_name("yotta") {
        let typ = val.get_prefix_type();

        println!("PrefixTyp von {} ist {:?}", val.name(), typ);
        type_of(&typ);
    }

    // end 
    println!("\n{}", "End of code!\n");
}
