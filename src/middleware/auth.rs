use crate::domain::vo::JWTToken;
use crate::error_info;
use crate::service::CONTEXT;
pub struct Auth;

///Whether the interface is in the whitelist
pub fn is_white_list_api(path: &str) -> bool {
    if path.eq("/") {
        return true;
    }
    for x in &CONTEXT.config.white_list_api {
        if x.contains(path) {
            return true;
        }
    }
    return false;
}

///Check whether the token is valid and has not expired
pub async fn checked_token(token: &str, _path: &str) -> Result<JWTToken, crate::error::Error> {
    //check token alive
    let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
    match token {
        Ok(token) => {
            return Ok(token);
        }
        Err(e) => {
            return Err(crate::error::Error::from(e.to_string()));
        }
    }
}

///Permission to check
pub async fn check_auth(token: &JWTToken, path: &str) -> Result<(), crate::error::Error> {
    let sys_permission = CONTEXT.sys_permission_service.finds_all().await?;
    for token_permission in &token.permissions {
        for x in &sys_permission {
            match &x.permission {
                Some(permission) => match &x.path {
                    None => {}
                    Some(x_path) => {
                        if permission.eq(token_permission) && path.contains(x_path) {
                            return Ok(());
                        }
                    }
                },
                _ => {}
            }
        }
    }
    return Err(crate::error::Error::from(error_info!("access_denied")));
}
