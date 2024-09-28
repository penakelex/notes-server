use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::{header, Request};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use chrono::Utc;

use crate::context::AuthTokenContext;
use crate::error::{Error, Result, UserError};
use crate::log::log_layer;
use crate::state::ApplicationState;
use crate::web::jwt_controller::TokenClaims;

const AUTH_TOKEN: &str = "auth-token";

const AUTH_MIDDLEWARE: &str = "AUTH_MIDDLEWARE";

pub async fn set_auth_token_middleware(
    State(state): State<ApplicationState>,
    mut response: Response,
) -> Result<Response> {
    log_layer(AUTH_MIDDLEWARE, "set_auth_token");
    
    let context = response.extensions()
        .get::<AuthTokenContext>()
        .ok_or(Error::User(UserError::AuthFail))?;

    let user_id = context.user_id();

    let session = state.database.sessions
        .create_session(user_id, state.settings.jwt.validity_days)
        .await?;

    let claims = TokenClaims {
        sub: session.id,
        exp: session.expires_at,
    };

    let token = state.jwt.generate_token(&claims)?;

    let cookie = Cookie::build((AUTH_TOKEN, token))
        .path("/")
        .max_age(time::Duration::days(state.settings.jwt.validity_days as i64))
        .build();

    response.headers_mut().insert(
        header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );

    println!(">> {AUTH_MIDDLEWARE:<30} - setting ended auth token");

    Ok(response)
}

pub async fn token_context_resolver_middleware(
    cookies: CookieJar,
    State(state): State<ApplicationState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response> {
    log_layer(AUTH_MIDDLEWARE, "token_context_resolver");
    
    let auth_token = cookies
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string());

    let context = match auth_token
        .ok_or(Error::User(UserError::AuthFail))
        .and_then(|auth_token| state.jwt
            .get_claims_from_token(auth_token.as_ref())
        )
    {
        Ok(claims) => {
            let current_time = Utc::now();
            if claims.exp < (current_time.timestamp() as usize) {
                Err(Error::User(UserError::AuthFail))
            } else {
                match state.database.sessions
                    .session_validity(claims.sub, claims.exp).await
                {
                    Ok(user_id) => Ok(AuthTokenContext::new(user_id)),
                    Err(_) => Err(Error::User(UserError::AuthFail))
                }
            }
        }
        Err(error) => Err(error)
    };

    request.extensions_mut().insert(context);

    Ok(next.run(request).await)
}

pub async fn require_auth_middleware(
    context: Result<AuthTokenContext>,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    log_layer(AUTH_MIDDLEWARE, "require_auth");
    
    context?;

    Ok(next.run(request).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AuthTokenContext {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts.extensions
            .get::<Result<AuthTokenContext>>()
            .ok_or(Error::User(UserError::AuthFail))?
            .clone()
    }
}