use crate::generated::components::{request_bodies, responses};
use crate::generated::paths::register;
use crate::models::{RegistrationRequest, User};
use crate::DbPool;
use actix_swagger::Answer;
use actix_web::web;
use diesel::result::{DatabaseErrorKind, Error};

enum RegisterRequestError {
    EmailAlreadyRegistered,
    UnexpectedError,
}

fn handle_register_request(
    body: request_bodies::Register,
    pool: web::Data<DbPool>,
) -> Result<RegistrationRequest, RegisterRequestError> {
    let conn = &pool.get().unwrap();
    let is_busy = User::has_with_email(&conn, &body.email);

    if is_busy {
        Err(RegisterRequestError::EmailAlreadyRegistered)
    } else {
        let request = RegistrationRequest::new(&body.email);

        match request.create(&conn) {
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                Err(RegisterRequestError::EmailAlreadyRegistered)
            }
            Err(_) => Err(RegisterRequestError::UnexpectedError),
            Ok(value) => Ok(value),
        }
    }
}

pub async fn register_request(
    body: web::Json<request_bodies::Register>,
    pool: web::Data<DbPool>,
) -> Answer<'static, register::Response> {
    match handle_register_request(body.0, pool) {
        Ok(request) => register::Response::Created(responses::RegistrationRequestCreated {
            expires_at: request.expires_at.timestamp_millis(),
        })
        .answer(),
        Err(RegisterRequestError::EmailAlreadyRegistered) => {
            register::Response::BadRequest(responses::RegistrationFailed {
                error: "email_already_registered".to_string(),
            })
            .answer()
        }
        Err(RegisterRequestError::UnexpectedError) => register::Response::Unexpected.answer(),
    }
}