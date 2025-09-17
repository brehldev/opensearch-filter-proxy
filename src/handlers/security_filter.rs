use serde_json::{Value, json};

/// A service responsible for applying global filters to queries.
///
/// This service provides methods to modify and enhance queries with
/// additional security filters. It ensures that the filters are properly
/// integrated into the query structure, whether the query already contains
/// a `bool` clause or not.
#[derive(Clone)]
pub struct SecurityFilterService;

impl SecurityFilterService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn apply(&self, mut query: Value, filter_snippet: Value) -> Value {
        match query.get_mut("query") {
            Some(query_obj) => {
                if let Some(bool_query) = query_obj.get_mut("bool") {
                    self.add_filter_to_bool_query(bool_query, filter_snippet);
                } else {
                    *query_obj = self.wrap_in_bool_query(query_obj.clone(), filter_snippet);
                }
            }
            None => {
                // throw error or handle case where "query" is missing
            }
        }
        query
    }

    fn wrap_in_bool_query(&self, original_query: Value, filter: Value) -> Value {
        json!({
            "bool": {
                "must": [original_query],
                "filter": filter
            }
        })
    }

    fn add_filter_to_bool_query(&self, bool_query: &mut Value, filter: Value) {
        match bool_query.get_mut("filter") {
            Some(existing_filter) => {
                *existing_filter = self.merge_filters(existing_filter.clone(), filter);
            }
            None => {
                bool_query["filter"] = filter;
            }
        }
    }

    fn merge_filters(&self, existing: Value, new_filter: Value) -> Value {
        if existing.is_array() {
            let mut filters = existing.as_array().unwrap().clone();
            filters.push(new_filter);
            json!({
                "bool": {
                    "filter": filters
                }
            })
        } else {
            json!({
                "bool": {
                    "must": [existing, new_filter]
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_wrap_non_bool_query() {
        let service = SecurityFilterService::new();
        let query = json!({
            "query": {
                "term": {"field": "value"}
            }
        });
        let filter = json!({"term": {"user": "john"}});

        let result = service.apply(query, filter);

        assert_eq!(
            result,
            json!({"query":{"bool":{"filter":{"term":{"user":"john"}},"must":[{"term":{"field":"value"}}]}}})
        );
    }

    #[test]
    fn test_add_filter_to_existing_bool_query() {
        let service = SecurityFilterService::new();
        let query = json!({
            "query": {
                "bool": {
                    "must": [
                        {"term": {"field": "value"}}
                    ]
                }
            }
        });
        let filter = json!({"term": {"user": "john"}});

        let result = service.apply(query, filter);

        println!("Result: {}", result);

        assert_eq!(
            result,
            json!({"query":{"bool":{"filter":{"term":{"user":"john"}},"must":[{"term":{"field":"value"}}]}}})
        );
    }

    #[test]
    fn test_add_filter_to_existing_bool_query_with_existing_filter() {
        let service = SecurityFilterService::new();
        let query = json!({
            "query": {
                "bool": {
                    "must": [
                        {"term": {"field": "value"}}
                    ],
                    "filter": {"term": {"status": "active"}}
                }
            }
        });
        let filter = json!({"term": {"user": "john"}});

        let result = service.apply(query, filter);

        println!("Result: {}", result);

        assert_eq!(
            result,
            json!({"query":{"bool":{"filter":{"bool":{"must":[{"term":{"status":"active"}},{"term":{"user":"john"}}]}},"must":[{"term":{"field":"value"}}]}}})
        );
    }

    #[test]
    fn test_add_filter_to_existing_bool_query_with_existing_filter_array() {
        let service = SecurityFilterService::new();
        let query = json!({
            "query": {
                "bool": {
                    "must": [
                        {"term": {"field": "value"}}
                    ],
                    "filter": [
                        {"term": {"status": "active"}},
                        {"term": {"category": "books"}}
                    ]
                }
            }
        });
        let filter = json!({"term": {"user": "john"}});

        let result = service.apply(query, filter);

        println!("Result: {}", result);

        assert_eq!(
            result,
            json!({"query":{"bool":{"filter":{"bool":{"filter":[{"term":{"status":"active"}},{"term":{"category":"books"}},{"term":{"user":"john"}}]}},"must":[{"term":{"field":"value"}}]}}})
        );
    }
}
