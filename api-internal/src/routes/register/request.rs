use crate::generated::components::{request_bodies, responses};
use crate::generated::paths::register_request;
use actix_swagger::Answer;
use actix_web::web;

pub async fn route(
    body: web::Json<request_bodies::Register>,
    app: web::Data<accesso_app::App>,
) -> Answer<'static, register_request::Response> {
    use accesso_core::app::registrator::{
        CreateRegisterRequest,
        RegisterRequestError::{EmailAlreadyRegistered, InvalidForm, Unexpected},
        Registrator,
    };
    use register_request::Response;

    match app.registrator_create_request(CreateRegisterRequest::from_email(&body.email)) {
        Err(EmailAlreadyRegistered) => Response::BadRequest(responses::RegisterFailed {
            error: responses::RegisterFailedError::EmailAlreadyRegistered,
        }),
        Err(InvalidForm) => Response::BadRequest(responses::RegisterFailed {
            error: responses::RegisterFailedError::InvalidForm,
        }),
        Err(Unexpected) => Response::Unexpected,
        Ok(request) => Response::Created(responses::RegistrationRequestCreated {
            expires_at: request.expires_at.timestamp_millis(),
        }),
    }
    .answer()
}
