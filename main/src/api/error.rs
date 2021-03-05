use actix_web::error::{ResponseError, PayloadError};
use actix_web::http::{header, StatusCode};
use actix_web::HttpResponse;
use std::fmt;
use anyhow::Error;

pub type ApiResult<T> = std::result::Result<T, ApiErr>;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ApiErr {
    errors: Vec<ErrDetail>,
}

impl ApiErr {
    pub fn new<E: Into<String>>(err: E) -> Self {
        ApiErr { errors: vec![ErrDetail::new(err)] }
    }

    pub fn push<E: Into<String>>(&mut self, err: E) {
        self.errors.push(ErrDetail::new(err))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ErrDetail {
    detail: String,
}

impl ErrDetail {
    fn new<E: Into<String>>(err: E) -> Self {
        ErrDetail { detail: err.into() }
    }
}

impl fmt::Display for ApiErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", json!(self))
    }
}

impl From<anyhow::Error> for ApiErr {
    fn from(err: anyhow::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<PayloadError> for ApiErr {
    fn from(err: PayloadError) -> Self {
        Self::new(err.to_string())
    }
}

impl From<std::io::Error> for ApiErr {
    fn from(err: std::io::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl From<serde_json::Error> for ApiErr {
    fn from(err: serde_json::Error) -> Self {
        Self::new(err.to_string())
    }
}

impl ResponseError for ApiErr {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error() -> anyhow::Result<()> {
        let err_input = r#"
{
    "errors":[
        {
            "detail":"error message text"
        }
    ]
}"#;
        let api_err: ApiErr = serde_json::from_str(err_input)?;
        let expect = ApiErr {
            errors: vec![ErrDetail {
                detail: "error message text".to_string(),
            }],
        };
        assert_eq!(api_err, expect);
        Ok(())
    }
}
