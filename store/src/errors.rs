use std::fmt;

#[derive(Debug)]
pub enum CustomErrors {
    EmptyStock,
    IDNotFound,
    NameTooLong,
    NoCategory
}

impl fmt::Display for CustomErrors {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomErrors::EmptyStock => write!(format, "O estoque do produto está vazio"),
            CustomErrors::IDNotFound => write!(format, "O ID especificado não foi encontrado"),
            CustomErrors::NameTooLong => write!(format, "O nome do produto deve ter, no máximo, 40 caracteres"),
            CustomErrors::NoCategory => write!(format, "A categoria especificada não existe")
        }
    }
}

impl std::error::Error for CustomErrors {}