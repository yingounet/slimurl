#[derive(Debug, Clone, Default)]
pub struct DeviceInfo {
    pub device_type: String,
    pub browser: String,
    pub os: String,
}

pub fn parse_user_agent(user_agent: &str) -> DeviceInfo {
    let ua = user_agent.to_lowercase();
    
    DeviceInfo {
        device_type: detect_device_type(&ua),
        browser: detect_browser(&ua),
        os: detect_os(&ua),
    }
}

fn detect_device_type(ua: &str) -> String {
    if ua.contains("bot") || ua.contains("spider") || ua.contains("crawler") || ua.contains("curl") || ua.contains("wget") {
        return "bot".to_string();
    }
    
    if ua.contains("mobile") || ua.contains("android") || ua.contains("iphone") || ua.contains("ipod") {
        if ua.contains("tablet") || ua.contains("ipad") {
            return "tablet".to_string();
        }
        return "mobile".to_string();
    }
    
    if ua.contains("tablet") || ua.contains("ipad") {
        return "tablet".to_string();
    }
    
    "desktop".to_string()
}

fn detect_browser(ua: &str) -> String {
    if ua.contains("edg/") || ua.contains("edge") {
        return "Edge".to_string();
    }
    if ua.contains("opr/") || ua.contains("opera") {
        return "Opera".to_string();
    }
    if ua.contains("firefox") {
        return "Firefox".to_string();
    }
    if ua.contains("chrome") && !ua.contains("chromium") {
        return "Chrome".to_string();
    }
    if ua.contains("safari") && !ua.contains("chrome") {
        return "Safari".to_string();
    }
    if ua.contains("msie") || ua.contains("trident") {
        return "IE".to_string();
    }
    
    "Unknown".to_string()
}

fn detect_os(ua: &str) -> String {
    if ua.contains("windows nt 10") {
        return "Windows 10".to_string();
    }
    if ua.contains("windows nt 6.3") {
        return "Windows 8.1".to_string();
    }
    if ua.contains("windows nt 6.2") {
        return "Windows 8".to_string();
    }
    if ua.contains("windows nt 6.1") {
        return "Windows 7".to_string();
    }
    if ua.contains("windows") {
        return "Windows".to_string();
    }
    if ua.contains("android") {
        return "Android".to_string();
    }
    if ua.contains("iphone") || ua.contains("ipad") || ua.contains("ipod") {
        return "iOS".to_string();
    }
    if ua.contains("mac os x") || ua.contains("macos") {
        return "macOS".to_string();
    }
    if ua.contains("linux") {
        return "Linux".to_string();
    }
    
    "Unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chrome_desktop() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
        let info = parse_user_agent(ua);
        assert_eq!(info.device_type, "desktop");
        assert_eq!(info.browser, "Chrome");
        assert_eq!(info.os, "Windows 10");
    }

    #[test]
    fn test_mobile_safari() {
        let ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1";
        let info = parse_user_agent(ua);
        assert_eq!(info.device_type, "mobile");
        assert_eq!(info.browser, "Safari");
        assert_eq!(info.os, "iOS");
    }

    #[test]
    fn test_bot() {
        let ua = "Googlebot/2.1 (+http://www.google.com/bot.html)";
        let info = parse_user_agent(ua);
        assert_eq!(info.device_type, "bot");
    }
}
