use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Pager {
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub page_size: Option<i64>,
}

impl Pager {
    pub fn get_limit(&self) -> i64 {
        self.page_size.unwrap_or(10).max(1).min(1000) // 可配置上限
    }

    pub fn get_offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.get_limit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_default_pager() {
        let pager = Pager {
            page: None,
            page_size: None,
        };
        assert_eq!(pager.get_limit(), 10);
        assert_eq!(pager.get_offset(), 0); // (1 - 1) * 10 = 0
    }

    #[test]
    fn test_custom_page_and_page_size() {
        let pager = Pager {
            page: Some(3),
            page_size: Some(20),
        };
        assert_eq!(pager.get_limit(), 20);
        assert_eq!(pager.get_offset(), 40); // (3 - 1) * 20 = 40
    }

    #[test]
    fn test_page_size_too_small() {
        let pager = Pager {
            page: Some(2),
            page_size: Some(0),
        };
        assert_eq!(pager.get_limit(), 1); // clamped to min=1
        assert_eq!(pager.get_offset(), 1); // (2-1)*1 = 1
    }

    #[test]
    fn test_page_size_negative() {
        let pager = Pager {
            page: Some(1),
            page_size: Some(-5),
        };
        assert_eq!(pager.get_limit(), 1);
        assert_eq!(pager.get_offset(), 0);
    }

    #[test]
    fn test_page_size_too_large() {
        let pager = Pager {
            page: Some(1),
            page_size: Some(200),
        };
        assert_eq!(pager.get_limit(), 100); // clamped to max=100
        assert_eq!(pager.get_offset(), 0);
    }

    #[test]
    fn test_page_zero() {
        let pager = Pager {
            page: Some(0),
            page_size: Some(10),
        };
        assert_eq!(pager.get_limit(), 10);
        assert_eq!(pager.get_offset(), 0); // page clamped to 1 → (1-1)*10 = 0
    }

    #[test]
    fn test_page_negative() {
        let pager = Pager {
            page: Some(-10),
            page_size: Some(5),
        };
        assert_eq!(pager.get_limit(), 5);
        assert_eq!(pager.get_offset(), 0); // page clamped to 1
    }

    #[test]
    fn test_only_page_provided() {
        let pager = Pager {
            page: Some(5),
            page_size: None,
        };
        assert_eq!(pager.get_limit(), 10);
        assert_eq!(pager.get_offset(), 40); // (5-1)*10 = 40
    }

    #[test]
    fn test_only_page_size_provided() {
        let pager = Pager {
            page: None,
            page_size: Some(25),
        };
        assert_eq!(pager.get_limit(), 25);
        assert_eq!(pager.get_offset(), 0); // default page = 1
    }

    #[test]
    fn test_serde_deserialize_from_empty_map() {
        let json = "{}";
        let pager: Pager = serde_json::from_str(json).unwrap();
        assert_eq!(pager.page, None);
        assert_eq!(pager.page_size, None);
        assert_eq!(pager.get_limit(), 10);
        assert_eq!(pager.get_offset(), 0);
    }

    #[test]
    fn test_serde_deserialize_with_values() {
        let json = r#"{"page": 2, "page_size": 15}"#;
        let pager: Pager = serde_json::from_str(json).unwrap();
        assert_eq!(pager.page, Some(2));
        assert_eq!(pager.page_size, Some(15));
        assert_eq!(pager.get_limit(), 15);
        assert_eq!(pager.get_offset(), 15);
    }

    #[test]
    fn test_serde_deserialize_with_nulls() {
        let json = r#"{"page": null, "page_size": null}"#;
        let pager: Pager = serde_json::from_str(json).unwrap();
        assert_eq!(pager.page, None);
        assert_eq!(pager.page_size, None);
    }

    #[test]
    fn test_serde_deserialize_missing_fields() {
        let json = r#"{"page": 3}"#;
        let pager: Pager = serde_json::from_str(json).unwrap();
        assert_eq!(pager.page, Some(3));
        assert_eq!(pager.page_size, None);
        assert_eq!(pager.get_offset(), 20); // (3-1)*10
    }

    #[test]
    fn test_edge_case_max_page_size() {
        let pager = Pager {
            page: Some(1),
            page_size: Some(100),
        };
        assert_eq!(pager.get_limit(), 100);
        assert_eq!(pager.get_offset(), 0);
    }

    #[test]
    fn test_edge_case_page_1_with_min_limit() {
        let pager = Pager {
            page: Some(1),
            page_size: Some(1),
        };
        assert_eq!(pager.get_limit(), 1);
        assert_eq!(pager.get_offset(), 0);
    }
}
