use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CalculationError {
    ErrorNumberParenthesis,
    DivisionByZero,
    UnknownOperator(String),
}

// Implémentation de fmt::Display pour afficher un message pour chaque type d'erreur
impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculationError::ErrorNumberParenthesis => write!(f, "Erreur : Parenthèses mal équilibrées."),
            CalculationError::DivisionByZero => write!(f, "Erreur : Division par zéro."),
            CalculationError::UnknownOperator(operator) => write!(f, "Erreur : opérateur {} inconnu.", operator)
        }
    }
}