//! Volcengine Agent 定制：只允许接入 `api.volcengine.org` 中转站。
//!
//! 这里是唯一的「允许名单」。网络层（`protocol_proxy::send_upstream_request_with_header_timeout`）
//! 与设置归一化（`settings::normalize_settings_config_sections`）都调用本模块，
//! 因此即使用户手改 settings.json 或 config.toml，上游请求也只会发往该中转站。
//!
//! 本机回环地址（本地协议代理、网关）不算上游，予以放行。

use anyhow::bail;

/// 中转站主机名（唯一允许的远端）。
pub const ALLOWED_HOST: &str = "api.volcengine.org";
/// 默认写入供应商的 Base URL（OpenAI 兼容，New API 网关）。
pub const BASE_URL: &str = "https://api.volcengine.org/v1";
/// 内置供应商的固定 id / 名称。
pub const PROFILE_ID: &str = "volcengine";
pub const PROFILE_NAME: &str = "Volcengine";
/// 站点首页，用于引导用户获取令牌。
pub const SITE_URL: &str = "https://api.volcengine.org/";

fn is_loopback_host(host: &str) -> bool {
    matches!(host, "127.0.0.1" | "localhost" | "::1" | "[::1]")
}

/// URL 是否允许作为上游：`https://api.volcengine.org` 或本机回环。
pub fn is_allowed_url(url: &reqwest::Url) -> bool {
    let Some(host) = url.host_str() else {
        return false;
    };
    let host = host.to_ascii_lowercase();
    if is_loopback_host(&host) {
        return matches!(url.scheme(), "http" | "https");
    }
    url.scheme() == "https" && host == ALLOWED_HOST && url.port().is_none_or(|port| port == 443)
}

/// 字符串形式的 Base URL 是否允许；空串视为允许（由调用方补默认值）。
pub fn is_allowed_base_url(base_url: &str) -> bool {
    let trimmed = base_url.trim();
    if trimmed.is_empty() {
        return true;
    }
    reqwest::Url::parse(trimmed).is_ok_and(|url| is_allowed_url(&url))
}

/// 锁是否生效。发布构建恒为 true；只有测试时打开 `upstream-unlocked` 才关闭，
/// 供上游原有用例使用假主机名。允许名单本身（[`is_allowed_url`] 等）不受影响。
pub const fn enforced() -> bool {
    !cfg!(feature = "upstream-unlocked")
}

/// 网络层闸门：不在允许名单就报错，不发请求。
pub fn ensure_allowed_url(url: &reqwest::Url) -> anyhow::Result<()> {
    if !enforced() {
        return Ok(());
    }
    check_allowed_url(url)
}

/// 不看 [`enforced`] 的纯校验，供闸门和测试使用。
pub fn check_allowed_url(url: &reqwest::Url) -> anyhow::Result<()> {
    if is_allowed_url(url) {
        return Ok(());
    }
    bail!(
        "Volcengine Agent 只允许连接 {ALLOWED_HOST}，已拒绝请求：{}",
        url.host_str().unwrap_or("(未知主机)")
    )
}

/// 把任意 Base URL 收敛为允许值：合法保留（去尾斜杠），其余换成默认 [`BASE_URL`]。
pub fn coerce_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim();
    if !trimmed.is_empty() && is_allowed_base_url(trimmed) {
        trimmed.trim_end_matches('/').to_string()
    } else {
        BASE_URL.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn url(value: &str) -> reqwest::Url {
        reqwest::Url::parse(value).unwrap()
    }

    #[test]
    fn allows_only_the_relay_and_loopback() {
        assert!(is_allowed_url(&url("https://api.volcengine.org/v1/responses")));
        assert!(is_allowed_url(&url("https://API.volcengine.org/v1")));
        assert!(is_allowed_url(&url("http://127.0.0.1:57321/v1")));
        assert!(is_allowed_url(&url("http://localhost:8080/")));
    }

    #[test]
    fn rejects_everything_else() {
        assert!(!is_allowed_url(&url("https://api.openai.com/v1")));
        assert!(!is_allowed_url(&url("https://evil.com/api.volcengine.org")));
        assert!(!is_allowed_url(&url("https://api.volcengine.org.evil.com/v1")));
        assert!(!is_allowed_url(&url("http://api.volcengine.org/v1")));
        assert!(!is_allowed_url(&url("https://api.volcengine.org:8443/v1")));
        assert!(!is_allowed_url(&url("https://user@evil.com@api.volcengine.org.evil.com/")));
    }

    #[test]
    fn check_rejects_foreign_hosts_with_a_clear_error() {
        assert!(check_allowed_url(&url("https://api.volcengine.org/v1/responses")).is_ok());
        let error = check_allowed_url(&url("https://api.openai.com/v1")).unwrap_err();
        assert!(error.to_string().contains("api.openai.com"));
    }

    #[test]
    fn coerce_falls_back_to_default() {
        assert_eq!(coerce_base_url(""), BASE_URL);
        assert_eq!(coerce_base_url("https://api.openai.com/v1"), BASE_URL);
        assert_eq!(
            coerce_base_url("https://api.volcengine.org/v1/"),
            BASE_URL.trim_end_matches('/')
        );
    }
}
