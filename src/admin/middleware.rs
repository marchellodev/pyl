use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::Method;
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};

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
        // We only need to hook into the `start` for this middleware.

        println!("{}", req.path());
        println!("{}", req.method());
        if req.path() == "/api/admin/users" && req.method() == Method::POST {
            return Either::Left(self.service.call(req));
        }

        let auth = req.headers().get(actix_web::http::header::AUTHORIZATION);
        // TODO verify token
        println!("{:?}", auth);

        let is_logged_in = true; // Change this to see the change in outcome in the browser

        if is_logged_in {
            Either::Left(self.service.call(req))
        } else {
            Either::Right(ok(
                req.into_response(HttpResponse::Unauthorized().finish().into_body())
            ))
            // Don't forward to /login if we are already on /login
        }
    }
}
