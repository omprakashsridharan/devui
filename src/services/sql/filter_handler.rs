use std::collections::HashMap;

/// Comprehensive PostgreSQL filter handler that handles all common PostgreSQL types
pub struct FilterHandler;

/// Supported filter operations
#[derive(Debug, Clone, PartialEq)]
pub enum FilterOperation {
    /// Exact match (=)
    Equals,
    /// Case-insensitive contains (ILIKE '%value%')
    Contains,
    /// Case-insensitive starts with (ILIKE 'value%')
    StartsWith,
    /// Case-insensitive ends with (ILIKE '%value')
    EndsWith,
    /// Greater than (>)
    GreaterThan,
    /// Less than (<)
    LessThan,
    /// Greater than or equal (>=)
    GreaterThanOrEqual,
    /// Less than or equal (<=)
    LessThanOrEqual,
    /// Not equal (!=)
    NotEquals,
    /// Is null (IS NULL)
    IsNull,
    /// Is not null (IS NOT NULL)
    IsNotNull,
    /// In list (IN (...))
    In,
    /// Not in list (NOT IN (...))
    NotIn,
}

impl FilterOperation {
    /// Parse filter operation from string
    pub fn from_str(op: &str) -> Option<Self> {
        match op.to_lowercase().as_str() {
            "eq" | "=" | "equals" => Some(FilterOperation::Equals),
            "contains" | "like" | "~" => Some(FilterOperation::Contains),
            "startswith" | "starts" | "^" => Some(FilterOperation::StartsWith),
            "endswith" | "ends" | "$" => Some(FilterOperation::EndsWith),
            "gt" | ">" | "greater" => Some(FilterOperation::GreaterThan),
            "lt" | "<" | "less" => Some(FilterOperation::LessThan),
            "gte" | ">=" | "greaterorequal" => Some(FilterOperation::GreaterThanOrEqual),
            "lte" | "<=" | "lessorequal" => Some(FilterOperation::LessThanOrEqual),
            "ne" | "!=" | "notequals" => Some(FilterOperation::NotEquals),
            "null" | "isnull" => Some(FilterOperation::IsNull),
            "notnull" | "isnotnull" => Some(FilterOperation::IsNotNull),
            "in" => Some(FilterOperation::In),
            "notin" | "not_in" => Some(FilterOperation::NotIn),
            _ => None,
        }
    }
}

/// Filter configuration for a column
#[derive(Debug, Clone)]
pub struct ColumnFilter {
    pub column_name: String,
    pub data_type: String,
    pub operation: FilterOperation,
    pub value: String,
}

impl FilterHandler {
    /// Build WHERE clauses from filters
    pub fn build_where_clauses(
        filters: &HashMap<String, String>,
        column_info: &[(String, String)], // (column_name, data_type)
    ) -> Result<Vec<String>, FilterError> {
        let mut where_clauses = Vec::new();

        for (column_name, data_type) in column_info {
            if let Some(filter_value) = filters.get(column_name) {
                if filter_value.is_empty() {
                    continue;
                }

                let filter = Self::parse_filter(column_name, data_type, filter_value)?;
                let where_clause = Self::build_where_clause(&filter)?;
                where_clauses.push(where_clause);
            }
        }

        Ok(where_clauses)
    }

    /// Parse a filter string into a ColumnFilter
    fn parse_filter(
        column_name: &str,
        data_type: &str,
        filter_value: &str,
    ) -> Result<ColumnFilter, FilterError> {
        // Check if the filter contains an operation prefix
        let (operation, value) = if let Some((op_str, val)) = Self::extract_operation(filter_value)
        {
            let operation = FilterOperation::from_str(op_str)
                .ok_or_else(|| FilterError::InvalidOperation(op_str.to_string()))?;
            (operation, val)
        } else {
            // Default to contains for most types, equals for exact types
            let default_op = Self::get_default_operation(data_type);
            (default_op, filter_value.to_string())
        };

        Ok(ColumnFilter {
            column_name: column_name.to_string(),
            data_type: data_type.to_string(),
            operation,
            value,
        })
    }

    /// Extract operation from filter value (e.g., "gt:100" -> ("gt", "100"))
    fn extract_operation(filter_value: &str) -> Option<(&str, String)> {
        if let Some(colon_pos) = filter_value.find(':') {
            let (op, val) = filter_value.split_at(colon_pos);
            Some((op, val[1..].to_string()))
        } else {
            None
        }
    }

    /// Get default operation for a data type
    fn get_default_operation(data_type: &str) -> FilterOperation {
        match data_type.to_lowercase().as_str() {
            // Exact match for numeric types
            "integer" | "bigint" | "smallint" | "numeric" | "decimal" | "real"
            | "double precision" | "int2" | "int4" | "int8" | "float4" | "float8" => {
                FilterOperation::Equals
            }

            // Exact match for boolean
            "boolean" | "bool" => FilterOperation::Equals,

            // Exact match for UUID
            "uuid" => FilterOperation::Equals,

            // Contains for text types
            "text" | "varchar" | "char" | "character" | "character varying" => {
                FilterOperation::Contains
            }

            // Contains for date/time types
            "timestamp" | "timestamptz" | "date" | "time" | "timetz" => FilterOperation::Contains,

            // Contains for JSON types
            "json" | "jsonb" => FilterOperation::Contains,

            // Contains for arrays
            "array" => FilterOperation::Contains,

            // Contains for user-defined types
            "user-defined" => FilterOperation::Contains,

            // Default to contains
            _ => FilterOperation::Contains,
        }
    }

    /// Build WHERE clause from ColumnFilter
    fn build_where_clause(filter: &ColumnFilter) -> Result<String, FilterError> {
        let escaped_value = Self::escape_sql_value(&filter.value);

        match filter.operation {
            FilterOperation::Equals => {
                Self::build_equals_clause(&filter.column_name, &filter.data_type, &escaped_value)
            }
            FilterOperation::Contains => Ok(format!(
                "{}::text ILIKE '%{}%'",
                filter.column_name, escaped_value
            )),
            FilterOperation::StartsWith => Ok(format!(
                "{}::text ILIKE '{}%'",
                filter.column_name, escaped_value
            )),
            FilterOperation::EndsWith => Ok(format!(
                "{}::text ILIKE '%{}'",
                filter.column_name, escaped_value
            )),
            FilterOperation::GreaterThan => Self::build_comparison_clause(
                &filter.column_name,
                &filter.data_type,
                ">",
                &escaped_value,
            ),
            FilterOperation::LessThan => Self::build_comparison_clause(
                &filter.column_name,
                &filter.data_type,
                "<",
                &escaped_value,
            ),
            FilterOperation::GreaterThanOrEqual => Self::build_comparison_clause(
                &filter.column_name,
                &filter.data_type,
                ">=",
                &escaped_value,
            ),
            FilterOperation::LessThanOrEqual => Self::build_comparison_clause(
                &filter.column_name,
                &filter.data_type,
                "<=",
                &escaped_value,
            ),
            FilterOperation::NotEquals => Self::build_not_equals_clause(
                &filter.column_name,
                &filter.data_type,
                &escaped_value,
            ),
            FilterOperation::IsNull => Ok(format!("{} IS NULL", filter.column_name)),
            FilterOperation::IsNotNull => Ok(format!("{} IS NOT NULL", filter.column_name)),
            FilterOperation::In => {
                Self::build_in_clause(&filter.column_name, &filter.data_type, &filter.value)
            }
            FilterOperation::NotIn => {
                Self::build_not_in_clause(&filter.column_name, &filter.data_type, &filter.value)
            }
        }
    }

    /// Build equals clause with type-specific handling
    fn build_equals_clause(
        column_name: &str,
        data_type: &str,
        value: &str,
    ) -> Result<String, FilterError> {
        match data_type.to_lowercase().as_str() {
            // Numeric types - try exact match first, fallback to text search
            "integer" | "bigint" | "smallint" | "numeric" | "decimal" | "real"
            | "double precision" | "int2" | "int4" | "int8" | "float4" | "float8" => {
                if Self::is_numeric(value) {
                    Ok(format!("{} = {}", column_name, value))
                } else {
                    Ok(format!("{}::text ILIKE '%{}%'", column_name, value))
                }
            }

            // Boolean types
            "boolean" | "bool" => {
                let bool_value = Self::parse_boolean(value)?;
                Ok(format!("{} = {}", column_name, bool_value))
            }

            // UUID - exact match
            "uuid" => Ok(format!("{} = '{}'", column_name, value)),

            // Text types - case-insensitive match
            "text" | "varchar" | "char" | "character" | "character varying" => {
                Ok(format!("{} ILIKE '{}'", column_name, value))
            }

            // Date/time types - text search
            "timestamp" | "timestamptz" | "date" | "time" | "timetz" => {
                Ok(format!("{}::text ILIKE '%{}%'", column_name, value))
            }

            // JSON types - text search
            "json" | "jsonb" => Ok(format!("{}::text ILIKE '%{}%'", column_name, value)),

            // Arrays - text search
            "array" => Ok(format!("{}::text ILIKE '%{}%'", column_name, value)),

            // User-defined types - text search
            "user-defined" => Ok(format!("{}::text ILIKE '%{}%'", column_name, value)),

            // Default - text search
            _ => Ok(format!("{}::text ILIKE '%{}%'", column_name, value)),
        }
    }

    /// Build comparison clause (>, <, >=, <=)
    fn build_comparison_clause(
        column_name: &str,
        data_type: &str,
        operator: &str,
        value: &str,
    ) -> Result<String, FilterError> {
        match data_type.to_lowercase().as_str() {
            // Numeric types
            "integer" | "bigint" | "smallint" | "numeric" | "decimal" | "real"
            | "double precision" | "int2" | "int4" | "int8" | "float4" | "float8" => {
                if Self::is_numeric(value) {
                    Ok(format!("{} {} {}", column_name, operator, value))
                } else {
                    Err(FilterError::InvalidValue(format!(
                        "Non-numeric value '{}' for numeric comparison",
                        value
                    )))
                }
            }

            // Date/time types
            "timestamp" | "timestamptz" | "date" | "time" | "timetz" => {
                Ok(format!("{} {} '{}'", column_name, operator, value))
            }

            // Text types - lexicographic comparison
            "text" | "varchar" | "char" | "character" | "character varying" => {
                Ok(format!("{} {} '{}'", column_name, operator, value))
            }

            // Other types - convert to text for comparison
            _ => Ok(format!("{}::text {} '{}'", column_name, operator, value)),
        }
    }

    /// Build not equals clause
    fn build_not_equals_clause(
        column_name: &str,
        data_type: &str,
        value: &str,
    ) -> Result<String, FilterError> {
        match data_type.to_lowercase().as_str() {
            // Numeric types
            "integer" | "bigint" | "smallint" | "numeric" | "decimal" | "real"
            | "double precision" | "int2" | "int4" | "int8" | "float4" | "float8" => {
                if Self::is_numeric(value) {
                    Ok(format!("{} != {}", column_name, value))
                } else {
                    Ok(format!("{}::text NOT ILIKE '%{}%'", column_name, value))
                }
            }

            // Boolean types
            "boolean" | "bool" => {
                let bool_value = Self::parse_boolean(value)?;
                Ok(format!("{} != {}", column_name, bool_value))
            }

            // UUID
            "uuid" => Ok(format!("{} != '{}'", column_name, value)),

            // Text types
            "text" | "varchar" | "char" | "character" | "character varying" => {
                Ok(format!("{} NOT ILIKE '{}'", column_name, value))
            }

            // Other types
            _ => Ok(format!("{}::text NOT ILIKE '%{}%'", column_name, value)),
        }
    }

    /// Build IN clause
    fn build_in_clause(
        column_name: &str,
        data_type: &str,
        value: &str,
    ) -> Result<String, FilterError> {
        let values: Vec<&str> = value.split(',').map(|v| v.trim()).collect();
        if values.is_empty() {
            return Err(FilterError::InvalidValue("Empty IN list".to_string()));
        }

        let formatted_values = values
            .iter()
            .map(|v| {
                let escaped = Self::escape_sql_value(v);
                match data_type.to_lowercase().as_str() {
                    "integer" | "bigint" | "smallint" | "numeric" | "decimal" | "real"
                    | "double precision" | "int2" | "int4" | "int8" | "float4" | "float8" => {
                        if Self::is_numeric(v) {
                            escaped
                        } else {
                            format!("'{}'", escaped)
                        }
                    }
                    "boolean" | "bool" => Self::parse_boolean(v)
                        .map(|b| b.to_string())
                        .unwrap_or_else(|_| format!("'{}'", escaped)),
                    _ => format!("'{}'", escaped),
                }
            })
            .collect::<Vec<String>>()
            .join(", ");

        Ok(format!("{} IN ({})", column_name, formatted_values))
    }

    /// Build NOT IN clause
    fn build_not_in_clause(
        column_name: &str,
        data_type: &str,
        value: &str,
    ) -> Result<String, FilterError> {
        let in_clause = Self::build_in_clause(column_name, data_type, value)?;
        Ok(in_clause.replace(" IN (", " NOT IN ("))
    }

    /// Escape SQL value to prevent injection
    fn escape_sql_value(value: &str) -> String {
        value.replace("'", "''")
    }

    /// Check if a string represents a numeric value
    fn is_numeric(value: &str) -> bool {
        value.parse::<f64>().is_ok()
    }

    /// Parse boolean value
    fn parse_boolean(value: &str) -> Result<bool, FilterError> {
        match value.to_lowercase().as_str() {
            "true" | "t" | "1" | "yes" | "y" | "on" => Ok(true),
            "false" | "f" | "0" | "no" | "n" | "off" => Ok(false),
            _ => Err(FilterError::InvalidValue(format!(
                "Invalid boolean value: '{}'",
                value
            ))),
        }
    }
}

/// Error type for filter handling failures
#[derive(Debug, thiserror::Error)]
pub enum FilterError {
    #[error("Invalid filter operation: {0}")]
    InvalidOperation(String),

    #[error("Invalid filter value: {0}")]
    InvalidValue(String),
    // #[error("Filter parsing error: {0}")]
    // ParseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_operation_parsing() {
        assert_eq!(
            FilterOperation::from_str("eq"),
            Some(FilterOperation::Equals)
        );
        assert_eq!(
            FilterOperation::from_str("="),
            Some(FilterOperation::Equals)
        );
        assert_eq!(
            FilterOperation::from_str("contains"),
            Some(FilterOperation::Contains)
        );
        assert_eq!(
            FilterOperation::from_str("gt"),
            Some(FilterOperation::GreaterThan)
        );
        assert_eq!(
            FilterOperation::from_str(">"),
            Some(FilterOperation::GreaterThan)
        );
        assert_eq!(
            FilterOperation::from_str("null"),
            Some(FilterOperation::IsNull)
        );
        assert_eq!(FilterOperation::from_str("in"), Some(FilterOperation::In));
        assert_eq!(FilterOperation::from_str("invalid"), None);
    }

    #[test]
    fn test_extract_operation() {
        assert_eq!(
            FilterHandler::extract_operation("gt:100"),
            Some(("gt", "100".to_string()))
        );
        assert_eq!(
            FilterHandler::extract_operation("contains:test"),
            Some(("contains", "test".to_string()))
        );
        assert_eq!(FilterHandler::extract_operation("no_colon"), None);
        assert_eq!(FilterHandler::extract_operation(""), None);
    }

    #[test]
    fn test_get_default_operation() {
        assert_eq!(
            FilterHandler::get_default_operation("integer"),
            FilterOperation::Equals
        );
        assert_eq!(
            FilterHandler::get_default_operation("boolean"),
            FilterOperation::Equals
        );
        assert_eq!(
            FilterHandler::get_default_operation("text"),
            FilterOperation::Contains
        );
        assert_eq!(
            FilterHandler::get_default_operation("timestamp"),
            FilterOperation::Contains
        );
        assert_eq!(
            FilterHandler::get_default_operation("unknown"),
            FilterOperation::Contains
        );
    }

    #[test]
    fn test_is_numeric() {
        assert!(FilterHandler::is_numeric("123"));
        assert!(FilterHandler::is_numeric("123.45"));
        assert!(FilterHandler::is_numeric("-123"));
        assert!(FilterHandler::is_numeric("0"));
        assert!(!FilterHandler::is_numeric("abc"));
        assert!(!FilterHandler::is_numeric(""));
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(FilterHandler::parse_boolean("true").unwrap(), true);
        assert_eq!(FilterHandler::parse_boolean("false").unwrap(), false);
        assert_eq!(FilterHandler::parse_boolean("1").unwrap(), true);
        assert_eq!(FilterHandler::parse_boolean("0").unwrap(), false);
        assert_eq!(FilterHandler::parse_boolean("yes").unwrap(), true);
        assert_eq!(FilterHandler::parse_boolean("no").unwrap(), false);
        assert!(FilterHandler::parse_boolean("invalid").is_err());
    }

    #[test]
    fn test_escape_sql_value() {
        assert_eq!(FilterHandler::escape_sql_value("test"), "test");
        assert_eq!(FilterHandler::escape_sql_value("test'value"), "test''value");
        assert_eq!(FilterHandler::escape_sql_value(""), "");
    }

    #[test]
    fn test_build_equals_clause_numeric() {
        let result = FilterHandler::build_equals_clause("id", "integer", "123").unwrap();
        assert_eq!(result, "id = 123");

        let result = FilterHandler::build_equals_clause("id", "integer", "abc").unwrap();
        assert_eq!(result, "id::text ILIKE '%abc%'");
    }

    #[test]
    fn test_build_equals_clause_boolean() {
        let result = FilterHandler::build_equals_clause("active", "boolean", "true").unwrap();
        assert_eq!(result, "active = true");

        let result = FilterHandler::build_equals_clause("active", "boolean", "false").unwrap();
        assert_eq!(result, "active = false");
    }

    #[test]
    fn test_build_equals_clause_text() {
        let result = FilterHandler::build_equals_clause("name", "text", "test").unwrap();
        assert_eq!(result, "name ILIKE 'test'");
    }

    #[test]
    fn test_build_comparison_clause() {
        let result = FilterHandler::build_comparison_clause("age", "integer", ">", "18").unwrap();
        assert_eq!(result, "age > 18");

        let result = FilterHandler::build_comparison_clause("age", "integer", ">", "abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_in_clause() {
        let result = FilterHandler::build_in_clause("status", "text", "active,inactive").unwrap();
        assert_eq!(result, "status IN ('active', 'inactive')");

        let result = FilterHandler::build_in_clause("id", "integer", "1,2,3").unwrap();
        assert_eq!(result, "id IN (1, 2, 3)");

        let result = FilterHandler::build_in_clause("active", "boolean", "true,false").unwrap();
        assert_eq!(result, "active IN (true, false)");
    }

    #[test]
    fn test_build_not_in_clause() {
        let result =
            FilterHandler::build_not_in_clause("status", "text", "active,inactive").unwrap();
        assert_eq!(result, "status NOT IN ('active', 'inactive')");
    }

    #[test]
    fn test_build_where_clauses() {
        let mut filters = HashMap::new();
        filters.insert("name".to_string(), "contains:test".to_string());
        filters.insert("age".to_string(), "gt:18".to_string());
        filters.insert("active".to_string(), "true".to_string());

        let column_info = vec![
            ("name".to_string(), "text".to_string()),
            ("age".to_string(), "integer".to_string()),
            ("active".to_string(), "boolean".to_string()),
        ];

        let result = FilterHandler::build_where_clauses(&filters, &column_info).unwrap();

        assert_eq!(result.len(), 3);
        assert!(result
            .iter()
            .any(|clause| clause.contains("name::text ILIKE '%test%'")));
        assert!(result.iter().any(|clause| clause.contains("age > 18")));
        assert!(result.iter().any(|clause| clause.contains("active = true")));
    }

    #[test]
    fn test_filter_error_display() {
        let error = FilterError::InvalidOperation("invalid".to_string());
        assert!(error.to_string().contains("Invalid filter operation"));

        let error = FilterError::InvalidValue("bad value".to_string());
        assert!(error.to_string().contains("Invalid filter value"));
    }

    #[test]
    fn test_comprehensive_filter_types() {
        let test_cases = vec![
            // Integer types
            ("integer", "123", "column = 123"),
            ("bigint", "456", "column = 456"),
            ("smallint", "789", "column = 789"),
            ("numeric", "123.45", "column = 123.45"),
            ("decimal", "67.89", "column = 67.89"),
            ("real", "1.23", "column = 1.23"),
            ("double precision", "4.56", "column = 4.56"),
            // Boolean types
            ("boolean", "true", "column = true"),
            ("bool", "false", "column = false"),
            // Text types
            ("text", "test", "column ILIKE 'test'"),
            ("varchar", "value", "column ILIKE 'value'"),
            ("char", "char", "column ILIKE 'char'"),
            // Date/time types
            (
                "timestamp",
                "2023-01-01",
                "column::text ILIKE '%2023-01-01%'",
            ),
            (
                "timestamptz",
                "2023-01-01",
                "column::text ILIKE '%2023-01-01%'",
            ),
            ("date", "2023-01-01", "column::text ILIKE '%2023-01-01%'"),
            ("time", "12:00:00", "column::text ILIKE '%12:00:00%'"),
            // JSON types
            ("json", "key", "column::text ILIKE '%key%'"),
            ("jsonb", "value", "column::text ILIKE '%value%'"),
            // UUID
            (
                "uuid",
                "123e4567-e89b-12d3-a456-426614174000",
                "column = '123e4567-e89b-12d3-a456-426614174000'",
            ),
            // Arrays
            ("array", "item", "column::text ILIKE '%item%'"),
            // User-defined types
            ("user-defined", "custom", "column::text ILIKE '%custom%'"),
        ];

        for (data_type, value, expected_pattern) in test_cases {
            let result = FilterHandler::build_equals_clause("column", data_type, value).unwrap();
            assert!(
                result.contains(expected_pattern),
                "Failed for {}: expected pattern '{}' in result '{}'",
                data_type,
                expected_pattern,
                result
            );
        }
    }

    #[test]
    fn test_advanced_filter_operations() {
        let test_cases = vec![
            (
                FilterOperation::Contains,
                "text",
                "test",
                "column::text ILIKE '%test%'",
            ),
            (
                FilterOperation::StartsWith,
                "text",
                "test",
                "column::text ILIKE 'test%'",
            ),
            (
                FilterOperation::EndsWith,
                "text",
                "test",
                "column::text ILIKE '%test'",
            ),
            (FilterOperation::GreaterThan, "integer", "10", "column > 10"),
            (FilterOperation::LessThan, "integer", "20", "column < 20"),
            (
                FilterOperation::GreaterThanOrEqual,
                "integer",
                "15",
                "column >= 15",
            ),
            (
                FilterOperation::LessThanOrEqual,
                "integer",
                "25",
                "column <= 25",
            ),
            (
                FilterOperation::NotEquals,
                "text",
                "test",
                "column NOT ILIKE 'test'",
            ),
            (FilterOperation::IsNull, "text", "", "column IS NULL"),
            (FilterOperation::IsNotNull, "text", "", "column IS NOT NULL"),
        ];

        for (operation, data_type, value, expected_pattern) in test_cases {
            let filter = ColumnFilter {
                column_name: "column".to_string(),
                data_type: data_type.to_string(),
                operation: operation.clone(),
                value: value.to_string(),
            };

            let result = FilterHandler::build_where_clause(&filter).unwrap();
            assert!(
                result.contains(expected_pattern),
                "Failed for {:?}: expected pattern '{}' in result '{}'",
                operation,
                expected_pattern,
                result
            );
        }
    }
}
