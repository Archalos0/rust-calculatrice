#[cfg(test)]
mod tests {
    //use super::*; // Importe les fonctions du module principal
    use crate::{calcul::launch_calcul, errors::CalculationError};

    fn test_valid_calcul(operation_string: String, expected_result: i32) {
        let result = launch_calcul(operation_string);
    
        match result {
            Ok(value) => assert_eq!(value, expected_result),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    fn test_invalid_calcul(operation_string: String, expected_error: CalculationError) {
        let result = launch_calcul(operation_string);

        // Vérifie que le résultat est une erreur et compare l'erreur
        assert!(result.is_err(), "Expected an error but got Ok");
        match result {
            Err(e) => assert_eq!(e, expected_error),
            _ => {} // Cette branche ne devrait jamais être atteinte à cause de l'assertion précédente
        }
    }

    #[test]
    fn test_basic_operations() {
        test_valid_calcul("1+1".to_string(), 2);
        test_valid_calcul("2*3".to_string(), 6);
        test_valid_calcul("6/2".to_string(), 3);
        test_valid_calcul("5-3".to_string(), 2);
    }
    
    #[test]
    fn test_parenthesis_operations() {
        test_valid_calcul("2*(3+4)".to_string(), 14);
        test_valid_calcul("(10-3)*2".to_string(), 14);
        test_valid_calcul("4*(6/2)".to_string(), 12);
    }

    #[test]
    fn test_combinaison_operations() {
        test_valid_calcul("2+3*4".to_string(), 14);
        test_valid_calcul("(5+3)*2-4".to_string(), 12);
        test_valid_calcul("10/2+7*3".to_string(), 26);
    }

    #[test]
    fn test_with_zero() {
        test_valid_calcul("0+5".to_string(), 5);
        test_valid_calcul("10-0".to_string(), 10);
        test_valid_calcul("0*7".to_string(), 0);
        test_valid_calcul("10/1".to_string(), 10);
    }

    #[test]
    fn test_consecutives_parenthesis() {
        test_valid_calcul("(2*3)(15+7)".to_string(), 132);
        test_valid_calcul("(5+6)(2*2+7)(9-3)".to_string(), 726);
        test_valid_calcul("((5-8)(7*8))(15+7)".to_string(), -3696);
    }

    #[test]
    fn test_implicit_multiplication_combined() {
        test_valid_calcul("2(3)(4)".to_string(), 24); 
        test_valid_calcul("5(2+3)4".to_string(), 100);
        test_valid_calcul("2(3+4)".to_string(), 14); 
        test_valid_calcul("5(2)".to_string(), 10);
    }

    #[test]
    fn test_division_by_zero() {
        test_invalid_calcul("10/0".to_string(), CalculationError::DivisionByZero); 
        test_invalid_calcul("100 / (5 * 2 - 10)".to_string(), CalculationError::DivisionByZero);
        test_invalid_calcul("(2 * 3 + 4) / (5 - 5 * 1)".to_string(), CalculationError::DivisionByZero); 
        test_invalid_calcul("3 * 4 / (12 - 12)".to_string(), CalculationError::DivisionByZero);
        test_invalid_calcul("(5 + 5) / (10 - 10)".to_string(), CalculationError::DivisionByZero);
    }

    #[test]
    fn test_invalid_operator() {
        test_invalid_calcul("10$0".to_string(), CalculationError::UnknownOperator("$".to_string())); 
        test_invalid_calcul("100 / (5 * 2 ° 10)".to_string(), CalculationError::UnknownOperator("°".to_string()));
        test_invalid_calcul("(2 * 3 + 4) / (5 - 5 ¤ 1)".to_string(), CalculationError::UnknownOperator("$".to_string()));
    }

}