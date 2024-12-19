use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Session {
    pub session_token: String,
}

// const SESSION_URL: &str = "https://command.app.murawat.com/api/auth/session";
const SESSION_URL: &str = "http://localhost:5002/api/auth/session";

pub async fn get_session_token() -> Result<String, String> {
    return Ok("eyJhbGciOiJkaXIiLCJlbmMiOiJBMjU2R0NNIn0..2BeGT-Tckqdvj1LW.35jgvKu2s2_B47DHiPDmKiDKRRIbYht9waYnELXlvQAXcdvrFq0STq9zkKdcBjV4rlWy58jmxqVOs56wEopidFcX_BQ7MoamrQpzaPVZNvYQ8aYMUZWmUwObfC2XrVamrLFt_8meE7qLRihRipLpkCHlBT7On-JGQimY9NeAuBwJ0tJQzCiuzM48xjvLsxQ4Cyvb4ShhQVhWbLivAJmGt7vr5s9FevBbz2fztnIDw3yXYrYYbShrnXzT0cOliKhN-jdrmeW_gIN7Agh6dFAC4dmvOL6HGWx5qsR_g5I6z_qVElKRHPCpJ7CGFcxh57pto4RS6YCjoZwZvCRIMRE_Pm_BeBcvr9aOy_noTE78mItUf8TwcUavwmpr8eaidGFkaZSWrMI.mR2ZTjbJOReo73K7aaQwdA".to_string());

    let client = Client::new();

    let response = client.get(SESSION_URL).send().await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let session_response = resp.json::<Session>().await;

                match session_response {
                    Ok(session_result) => {
                        return Ok(session_result.session_token);
                    }
                    Err(_e) => {
                        return Err("Failed to authenticate".to_string());
                    }
                }
            } else {
                return Err("Failed to authenticate".to_string());
            }
        }
        Err(_e) => return Err("Failed to authenticate".to_string()),
    }
}
