use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Request, State},
    http::{Method, StatusCode},
    middleware::Next,
    response::Response,
    routing::{get, post},
};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Days, Utc};
use cookie::{Cookie, time::Duration};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::auth::auth_repo::{PsqlAuthRepo, SignUpParams};
use crate::user::{UserRes, UserRow};

#[derive(Clone)]
pub struct AuthRouteState {
    pub auth_repo: Arc<PsqlAuthRepo>,
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "auth.ts")]
struct LoginReq {
    email: String,
    password: String,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "auth.ts")]
struct LoginRes {
    session_id: Uuid,
    expires_at: DateTime<Utc>,
}

pub fn get_auth_router(auth_repo: Arc<PsqlAuthRepo>) -> Router {
    Router::new()
        .route("/me", get(get_me))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/sign-up", post(sign_up))
        .with_state(AuthRouteState { auth_repo })
}

pub type CurrentUser = UserRow;

struct UnAuthedRoute {
    path: &'static str,
    method: Method,
}

impl UnAuthedRoute {
    fn matches(&self, req: &Request) -> bool {
        self.path == req.uri().path() && self.method == req.method()
    }
}

static SIGN_UP_ROUTE: UnAuthedRoute = UnAuthedRoute {
    path: "/auth/sign-up",
    method: Method::POST,
};

static LOGIN_ROUTE: UnAuthedRoute = UnAuthedRoute {
    path: "/auth/login",
    method: Method::POST,
};

static UNAUTHENTICATED_ROUTES: &[&UnAuthedRoute] = &[&LOGIN_ROUTE, &SIGN_UP_ROUTE];

fn matches_unauthenticated_route(req: &Request) -> bool {
    UNAUTHENTICATED_ROUTES.iter().any(|r| r.matches(req))
}

pub async fn authenticated(
    State(state): State<AuthRouteState>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if matches_unauthenticated_route(&req) {
        return Ok(next.run(req).await);
    }

    let session_id = jar
        .get("session_id")
        .map(Cookie::value)
        .map(Uuid::parse_str)
        .and_then(Result::ok)
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;

    let current_user = state
        .auth_repo
        .get_user_by_session_id(session_id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}

pub async fn get_me(
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<UserRes>, StatusCode> {
    Ok(Json(UserRes::from_row(&current_user)))
}

async fn login(
    State(state): State<AuthRouteState>,
    jar: CookieJar,
    Json(req): Json<LoginReq>,
) -> Result<(CookieJar, Json<LoginRes>), StatusCode> {
    let session_row = state
        .auth_repo
        .login(&req.email, &req.password)
        .await
        .map_err(|_e| StatusCode::UNAUTHORIZED)?;

    // TODO: .secure cookie in prod-like
    let cookie = Cookie::build(("session_id", session_row.id.to_string()))
        .path("/")
        .http_only(true)
        .max_age(Duration::days(7))
        .build();

    let res_json = Json(LoginRes {
        session_id: session_row.id,
        expires_at: Utc::now()
            .checked_add_days(Days::new(7))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
    });

    return Ok((jar.add(cookie), res_json));
}

#[derive(Deserialize, TS)]
#[ts(export, export_to = "auth.ts")]

struct SignUpReq {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

impl SignUpReq {
    fn to_params(&self) -> SignUpParams {
        return SignUpParams {
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            password: self.password.clone(),
        };
    }
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "auth.ts")]

struct SignUpRes {
    user: UserRes,
    session: LoginRes,
}

async fn sign_up(
    State(state): State<AuthRouteState>,
    jar: CookieJar,
    Json(req): Json<SignUpReq>,
) -> Result<(CookieJar, Json<SignUpRes>), StatusCode> {
    let result = state
        .auth_repo
        .sign_up(req.to_params())
        .await
        .map_err(|_e| StatusCode::BAD_REQUEST)?;

    // TODO: .secure cookie in prod-like
    let cookie = Cookie::build(("session_id", result.session.id.to_string()))
        .path("/")
        .http_only(true)
        .max_age(Duration::days(7))
        .build();

    let session = LoginRes {
        session_id: result.session.id,
        expires_at: Utc::now()
            .checked_add_days(Days::new(7))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    let res_json = Json(SignUpRes {
        user: UserRes::from_row(&result.user),
        session,
    });

    return Ok((jar.add(cookie), res_json));
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "auth.ts")]
struct LogoutRes {
    session_id: Uuid,
}

async fn logout(
    State(state): State<AuthRouteState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<LogoutRes>), StatusCode> {
    let session_id = jar
        .get("session_id")
        .map(Cookie::value)
        .map(Uuid::parse_str)
        .and_then(Result::ok)
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;

    let session_row = state
        .auth_repo
        .delete_session(session_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: .secure cookie in prod-like
    let cookie = Cookie::build(("session_id", session_row.id.to_string()))
        .path("/")
        .http_only(true)
        .build();

    let res_json = Json(LogoutRes {
        session_id: session_row.id,
    });

    return Ok((jar.remove(cookie), res_json));
}
