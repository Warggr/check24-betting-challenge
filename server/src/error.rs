// Adapted from https://github.com/edo1z/rust-rocket-sqlx-sample
// Therefore under the MIT License:
/*
MIT License

Copyright (c) 2023 net3i

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;

#[derive(Debug)]
pub enum AppError {
    #[error("Database Error")]
    DbError(#[from] DbRepoError),
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("{message}")]
    CustomError { status_code: u16, message: String },
}

impl AppError {
    pub fn new(status_code: u16, message: &str) -> Self {
        AppError::CustomError {
            status_code,
            message: message.to_string(),
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            AppError::DbError(_) => 500,
            AppError::BadRequest => 400,
            AppError::Unauthorized => 401,
            AppError::Forbidden => 403,
            AppError::NotFound => 404,
            AppError::InternalServerError => 500,
            AppError::CustomError { status_code, .. } => *status_code,
        }
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status_code = self.status_code();
        let status = Status::from_code(status_code).unwrap_or(Status::InternalServerError);
        let error_message = self.to_string();
        Response::build_from(error_message.respond_to(req)?)
            .status(status)
            .ok()
    }
}
