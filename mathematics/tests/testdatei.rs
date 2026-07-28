use mathematics::math_core::are_nearly_equal;
use mathematics::math_constants::MathConstants;

    #[test]
    fn test_are_nearly_equal() {
        // Entweder mit eigenen Werten aufrufen:
        let test1 = are_nearly_equal(1.0, 1.000000000000001, Some(MathConstants::MIN_ABSTOL_f64), Some(MathConstants::MIN_RELTOL_f64));
        println!("test1: {}", test1);

        // Oder die Defaults bequem via `None` nutzen:
        let test2 = are_nearly_equal(1.0, 1.0000000000000001, None, None);
        println!("test2: {}", test2);
        
        // Optional: Echte Assertions hinzufügen
        assert!(test2);
    }
