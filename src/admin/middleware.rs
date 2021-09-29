use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::Method;
use actix_web::web::Data;
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};

use crate::admin::users::verify_token;
use crate::s_env::{Env, RockWrapper};

pub struct CheckLogin;

impl<S, B> Transform<S> for CheckLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

// TODO CHECK IF THE USER ACTUALLY EXISTS & WRITE THAT TO APP STATE

impl<S, B> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // this is the authorization path
        if req.path() == "/api/admin/users" && req.method() == Method::POST {
            return Either::Left(self.service.call(req));
        }

        let auth = req.headers().get(actix_web::http::header::AUTHORIZATION);

        if auth.is_some() {
            let auth = auth.unwrap().to_str().unwrap();
            let auth = auth.replace("Bearer ", "");

            let env = req.app_data::<Data<Env>>().unwrap();
            let rock = req.app_data::<Data<RockWrapper>>().unwrap();

            if verify_token(&rock.db, &env, &auth) {
                return Either::Left(self.service.call(req));
            }
        }

        Either::Right(ok(
            req.into_response(HttpResponse::Unauthorized().finish().into_body())
        ))
    }
}
