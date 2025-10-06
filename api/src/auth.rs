use std::{error::Error, sync::Arc};

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
use sqlx::{Pool, Postgres};
use ts_rs::TS;
use uuid::Uuid;

use crate::user::{PsqlUserStore, UserInsert, UserRes, UserRow};

#[derive(Clone)]
pub struct AuthRouteState {
    pub db: Pool<Postgres>,
    pub store: Arc<PsqlUserStore>,
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

pub fn get_auth_router(store: Arc<PsqlUserStore>, db: Pool<Postgres>) -> Router {
    Router::new()
        .route("/me", get(get_me))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/sign-up", post(sign_up))
        .with_state(AuthRouteState { db, store })
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

    let current_user = get_user_by_session_id(&state.db, session_id)
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

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

async fn login(
    State(state): State<AuthRouteState>,
    jar: CookieJar,
    Json(req): Json<LoginReq>,
) -> Result<(CookieJar, Json<LoginRes>), StatusCode> {
    let user_row = state
        .store
        .get_email(&req.email)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // TODO: how does lib handle changes to default params? poorly? backwards compat??
    let argon2 = Argon2::default();
    let verify_result = PasswordHash::new(&user_row.password_hash)
        .and_then(|hash| argon2.verify_password(req.password.as_bytes(), &hash));

    if verify_result.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let session_row = create_session(
        &state.db,
        InsertSession {
            user_id: user_row.id,
        },
    )
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

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
    // TODO: how does lib handle changes to default params? poorly? backwards compat??
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let user_row = state
        .store
        .create(UserInsert {
            first_name: req.first_name,
            last_name: req.last_name,
            email: req.email,
            password_hash,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let session_row = create_session(
        &state.db,
        InsertSession {
            user_id: user_row.id,
        },
    )
    .await
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // TODO: .secure cookie in prod-like
    let cookie = Cookie::build(("session_id", session_row.id.to_string()))
        .path("/")
        .http_only(true)
        .max_age(Duration::days(7))
        .build();

    let session = LoginRes {
        session_id: session_row.id,
        expires_at: Utc::now()
            .checked_add_days(Days::new(7))
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    let res_json = Json(SignUpRes {
        user: UserRes::from_row(&user_row),
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

    let session_row = delete_session(&state.db, session_id)
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

struct InsertSession {
    user_id: i64,
}

#[allow(dead_code)]
struct SessionRow {
    id: Uuid,
    user_id: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

async fn create_session(
    db: &Pool<Postgres>,
    params: InsertSession,
) -> Result<SessionRow, Box<dyn Error>> {
    Ok(sqlx::query_as!(
        SessionRow,
        "
            INSERT INTO session
            (user_id)
            VALUES
            ($1)
            RETURNING *
        ",
        params.user_id,
    )
    .fetch_one(db)
    .await?)
}

async fn delete_session(
    db: &Pool<Postgres>,
    session_id: Uuid,
) -> Result<SessionRow, Box<dyn Error>> {
    Ok(sqlx::query_as!(
        SessionRow,
        "
            DELETE FROM session
            WHERE id = ($1)
            RETURNING *
        ",
        session_id,
    )
    .fetch_one(db)
    .await?)
}

async fn get_user_by_session_id(
    db: &Pool<Postgres>,
    session_id: Uuid,
) -> Result<UserRow, Box<dyn Error>> {
    Ok(sqlx::query_as!(
        UserRow,
        "
            SELECT rr_user.* FROM rr_user
            JOIN session
            ON rr_user.id = session.user_id
            WHERE session.id = $1",
        session_id
    )
    .fetch_one(db)
    .await?)
}
