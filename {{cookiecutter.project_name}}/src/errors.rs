{% if cookiecutter.use_thiserror == 'y' %}#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("invalid {value:?}")]
    InvalidValue {
        value: String,
    }
}{% endif %}