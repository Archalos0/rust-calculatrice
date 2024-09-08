#[cfg(test)]
mod tests {
    //use super::*; // Importe les fonctions du module principal
    use crate::calcul::launch_calcul;

    #[test]
    fn test_basic_operations() {
        assert_eq!(launch_calcul("1+1".to_string()), 2);
        assert_eq!(launch_calcul("2*3".to_string()), 6);
        assert_eq!(launch_calcul("6/2".to_string()), 3);
        assert_eq!(launch_calcul("5-3".to_string()), 2);
    }
    
    #[test]
    fn test_parenthesis_operations() {
        assert_eq!(launch_calcul("2*(3+4)".to_string()), 14);
        assert_eq!(launch_calcul("(10-3)*2".to_string()), 14);
        assert_eq!(launch_calcul("4*(6/2)".to_string()), 12);
    }

    #[test]
    fn test_combinaison_operations() {
        assert_eq!(launch_calcul("2+3*4".to_string()), 14);
        assert_eq!(launch_calcul("(5+3)*2-4".to_string()), 12);
        assert_eq!(launch_calcul("10/2+7*3".to_string()), 26);
    }

    #[test]
    fn test_with_zero() {
        assert_eq!(launch_calcul("0+5".to_string()), 5);
        assert_eq!(launch_calcul("10-0".to_string()), 10);
        assert_eq!(launch_calcul("0*7".to_string()), 0);
        assert_eq!(launch_calcul("10/1".to_string()), 10);
    }

    #[test]
    fn test_association_parenhesis_parenthesis() {
        assert_eq!(launch_calcul("(2*3)(15+7)".to_string()), 132);
    }
}