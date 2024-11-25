use std::fmt;

#[derive(Debug)]
pub enum CustomErrors {
    LowStock,
    NameTooLong,
    NoCategory,
    OperationCanceled,
    ProductNotFound,
    SaleNotFound,
    TooManyArguments
}

impl fmt::Display for CustomErrors {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomErrors::LowStock => write!(format, "O estoque do produto não é suficiente para esta compra."),
            CustomErrors::NameTooLong => write!(format, "O nome do produto deve ter, no máximo, 40 caracteres."),
            CustomErrors::NoCategory => write!(format, "A categoria especificada não existe."),
            CustomErrors::OperationCanceled => write!(format, "Operação cancelada."),
            CustomErrors::ProductNotFound => write!(format, "O produto não foi encontrado."),
            CustomErrors::SaleNotFound => write!(format, "Nenhuma venda encontrada."),
            CustomErrors::TooManyArguments => write!(format, "Foram fornecidos mais argumentos que o máximo.")
        }
    }
}

impl std::error::Error for CustomErrors {}

#[cfg(test)]
mod tests {
    use super::CustomErrors;
    
    #[test]
    fn test_custom_errors_display() {
        assert_eq!(format!("{}", CustomErrors::LowStock), "O estoque do produto não é suficiente para esta compra.");
        assert_eq!(format!("{}", CustomErrors::NameTooLong), "O nome do produto deve ter, no máximo, 40 caracteres.");
        assert_eq!(format!("{}", CustomErrors::NoCategory), "A categoria especificada não existe.");
        assert_eq!(format!("{}", CustomErrors::OperationCanceled), "Operação cancelada.");
        assert_eq!(format!("{}", CustomErrors::ProductNotFound), "O produto não foi encontrado.");
        assert_eq!(format!("{}", CustomErrors::SaleNotFound), "Nenhuma venda encontrada.");
        assert_eq!(format!("{}", CustomErrors::TooManyArguments), "Foram fornecidos mais argumentos que o máximo.");
    }
}