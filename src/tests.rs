#[cfg(test)]
mod tests {
    //use super::*; // Importe les fonctions du module principal
    use crate::calcul::launch_calcul;

    fn test(operation_string: String, expected_result: i32) {
        let result = launch_calcul(operation_string);
    
        match result {
            Ok(value) => assert_eq!(value, expected_result),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_basic_operations() {
        test("1+1".to_string(), 2);
        test("2*3".to_string(), 6);
        test("6/2".to_string(), 3);
        test("5-3".to_string(), 2);
    }
    
    #[test]
    fn test_parenthesis_operations() {
        test("2*(3+4)".to_string(), 14);
        test("(10-3)*2".to_string(), 14);
        test("4*(6/2)".to_string(), 12);
    }

    #[test]
    fn test_combinaison_operations() {
        test("2+3*4".to_string(), 14);
        test("(5+3)*2-4".to_string(), 12);
        test("10/2+7*3".to_string(), 26);
    }

    #[test]
    fn test_with_zero() {
        test("0+5".to_string(), 5);
        test("10-0".to_string(), 10);
        test("0*7".to_string(), 0);
        test("10/1".to_string(), 10);
    }

    #[test]
    fn test_consecutives_parenthesis() {
        test("(2*3)(15+7)".to_string(), 132);
        test("(5+6)(2*2+7)(9-3)".to_string(), 726);
        test("((5-8)(7*8))(15+7)".to_string(), -3696);
    }

    #[test]
    fn test_implicit_multiplication_combined() {
        test("2(3)(4)".to_string(), 24); 
        test("5(2+3)4".to_string(), 100);
        test("2(3+4)".to_string(), 14); 
        test("5(2)".to_string(), 10);
    }
}