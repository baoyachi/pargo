use actix_web::error::ResponseError;
use actix_web::http::{header, StatusCode};
use actix_web::HttpResponse;
use std::fmt;

pub type ApiResult<T> = std::result::Result<T, ApiErr>;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ApiErr {
    errors: Vec<ErrDetail>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ErrDetail {
    detail: String,
}

impl fmt::Display for ApiErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", json!(self))
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
