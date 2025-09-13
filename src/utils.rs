use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;

pub fn is_banned_path(path: &str) -> bool {
    static BANNED_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^/?(?:[^/]+/)?_search(?:/.*)?$").expect("valid banned path regex")
    });

    BANNED_RE.is_match(path)
}

pub fn is_banned_query_param(
    params: &HashMap<String, String>,
    banned_params: &Option<Vec<String>>,
) -> bool {
    let Some(banned_params) = banned_params else {
        return false;
    };

    params.keys().any(|key| banned_params.contains(key))
}

#[cfg(test)]
mod is_banned_path_tests {
    use super::*;

    #[test]
    fn bans_root_search_with_leading_slash() {
        assert!(
            is_banned_path("/proxy/_search"),
            "Expected '/_search' to be banned"
        );
    }

    #[test]
    fn bans_root_search_without_leading_slash() {
        assert!(
            is_banned_path("proxy/_search"),
            "Expected '_search' to be banned"
        );
    }

    #[test]
    fn bans_nested_search_path() {
        assert!(is_banned_path("/proxy/_search/stats"));
    }

    #[test]
    fn does_not_ban_similar_prefix() {
        assert!(!is_banned_path("/_searchable"));
    }

    #[test]
    fn does_not_ban_unrelated_path() {
        assert!(!is_banned_path("/public/assets"));
    }

    #[test]
    fn is_case_sensitive_for_path() {
        assert!(!is_banned_path("/_SEARCH"));
    }

    #[test]
    fn empty_path_not_banned() {
        assert!(!is_banned_path(""));
    }
}

#[cfg(test)]
mod is_banned_query_param_tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn bans_single_q_param() {
        let mut params = HashMap::new();
        params.insert("q".into(), "value".into());
        let banned_params = Some(vec!["q".to_string(), "query".to_string()]);
        assert!(is_banned_query_param(&params, &banned_params));
    }

    #[test]
    fn bans_single_query_param() {
        let mut params = HashMap::new();
        params.insert("query".into(), "v".into());
        let banned_params = Some(vec!["q".to_string(), "query".to_string()]);
        assert!(is_banned_query_param(&params, &banned_params));
    }

    #[test]
    fn bans_when_multiple_and_one_banned() {
        let mut params = HashMap::new();
        params.insert("page".into(), "1".into());
        params.insert("q".into(), "rust".into());
        let banned_params = Some(vec!["q".to_string(), "query".to_string()]);
        assert!(is_banned_query_param(&params, &banned_params));
    }

    #[test]
    fn bans_when_both_banned_present() {
        let mut params = HashMap::new();
        params.insert("q".into(), "rust".into());
        params.insert("query".into(), "rust".into());
        let banned_params = Some(vec!["q".to_string(), "query".to_string()]);
        assert!(is_banned_query_param(&params, &banned_params));
    }

    #[test]
    fn does_not_ban_when_only_unrelated() {
        let mut params = HashMap::new();
        params.insert("page".into(), "2".into());
        params.insert("lang".into(), "en".into());
        let banned_params = Some(vec!["q".to_string(), "query".to_string()]);
        assert!(!is_banned_query_param(&params, &banned_params));
    }

    #[test]
    fn empty_params_not_banned() {
        let params: HashMap<String, String> = HashMap::new();
        let banned_params = Some(vec!["q".to_string(), "query".to_string()]);
        assert!(!is_banned_query_param(&params, &banned_params));
    }

    #[test]
    fn case_sensitive_param_keys() {
        let mut params = HashMap::new();
        params.insert("Q".into(), "x".into());
        let banned_params = Some(vec!["q".to_string(), "query".to_string()]);
        assert!(!is_banned_query_param(&params, &banned_params));
    }
}
