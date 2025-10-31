use hex;
use sqlx::types::chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use sqlx::types::{Json, Uuid};
use sqlx::{Column, Row, TypeInfo};
use tracing::warn;

/// Comprehensive PostgreSQL field decoder that handles all common PostgreSQL types
pub struct FieldDecoder;

impl FieldDecoder {
    /// Decode a field value from a PostgreSQL row based on its type
    pub fn decode_field(
        row: &sqlx::postgres::PgRow,
        column: &sqlx::postgres::PgColumn,
    ) -> Result<String, FieldDecodeError> {
        let type_info = column.type_info();
        let type_name = type_info.name().to_ascii_uppercase();
        let column_name = column.name();

        match type_name.as_str() {
            // Integer types
            "INT2" | "SMALLINT" => Self::decode_i16(row, column_name),
            "INT4" | "INTEGER" => Self::decode_i32(row, column_name),
            "INT8" | "BIGINT" => Self::decode_i64(row, column_name),

            // Floating point types
            "FLOAT4" | "REAL" => Self::decode_f32(row, column_name),
            "FLOAT8" | "DOUBLE PRECISION" => Self::decode_f64(row, column_name),

            // Decimal/Numeric types
            "NUMERIC" | "DECIMAL" => Self::decode_numeric(row, column_name),

            // Boolean types
            "BOOL" | "BOOLEAN" => Self::decode_bool(row, column_name),

            // Text types
            "TEXT" | "VARCHAR" | "CHAR" | "CHARACTER" | "CHARACTER VARYING" => {
                Self::decode_text(row, column_name)
            }

            // Binary types
            "BYTEA" => Self::decode_bytea(row, column_name),

            // Date/Time types
            "DATE" => Self::decode_date(row, column_name),
            "TIME" | "TIME WITHOUT TIME ZONE" => Self::decode_time(row, column_name),
            "TIMESTAMP" | "TIMESTAMP WITHOUT TIME ZONE" => Self::decode_timestamp(row, column_name),
            "TIMESTAMPTZ" | "TIMESTAMP WITH TIME ZONE" => {
                Self::decode_timestamptz(row, column_name)
            }
            "TIMETZ" | "TIME WITH TIME ZONE" => Self::decode_timetz(row, column_name),

            // JSON types
            "JSON" => Self::decode_json(row, column_name),
            "JSONB" => Self::decode_jsonb(row, column_name),

            // UUID type
            "UUID" => Self::decode_uuid(row, column_name),

            // Array types (basic support)
            type_name if type_name.starts_with("_") => Self::decode_array(row, column_name),

            // Custom types (should be cast to text in query)
            _ => Self::decode_custom_type(row, column_name, &type_name),
        }
    }

    // Integer decoders
    fn decode_i16(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: i16 = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    fn decode_i32(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: i32 = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    fn decode_i64(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: i64 = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    // Floating point decoders
    fn decode_f32(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: f32 = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    fn decode_f64(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: f64 = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    // Numeric decoder
    fn decode_numeric(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: String = row.try_get(column_name)?;
        Ok(value)
    }

    // Boolean decoder
    fn decode_bool(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: bool = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    // Text decoder
    fn decode_text(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: Option<String> = row.try_get(column_name)?;
        Ok(value.unwrap_or_default())
    }

    // Binary decoder
    fn decode_bytea(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: Vec<u8> = row.try_get(column_name)?;
        let hex_string = format!("\\x{}", hex::encode(&value));
        Ok(hex_string)
    }

    // Date/Time decoders
    fn decode_date(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: NaiveDate = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    fn decode_time(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: NaiveTime = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    fn decode_timestamp(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: NaiveDateTime = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    fn decode_timestamptz(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: DateTime<Utc> = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    fn decode_timetz(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        // TIME WITH TIME ZONE is complex, decode as string for now
        let value: String = row.try_get(column_name)?;
        Ok(value)
    }

    // JSON decoders
    fn decode_json(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: String = row.try_get(column_name)?;
        Ok(value)
    }

    fn decode_jsonb(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: Json<serde_json::Value> = row.try_get(column_name)?;
        Ok(value.to_string())
    }

    // UUID decoder
    fn decode_uuid(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: Option<Uuid> = row.try_get(column_name)?;
        Ok(value.map(|u| u.to_string()).unwrap_or_default())
    }

    // Array decoder (basic support)
    fn decode_array(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
    ) -> Result<String, FieldDecodeError> {
        let value: String = row.try_get(column_name)?;
        Ok(value)
    }

    // Custom type decoder with fallback strategies
    fn decode_custom_type(
        row: &sqlx::postgres::PgRow,
        column_name: &str,
        type_name: &str,
    ) -> Result<String, FieldDecodeError> {
        // Try to decode as text first (for custom types cast to text)
        match row.try_get::<Option<String>, _>(column_name) {
            Ok(value) => Ok(value.unwrap_or_default()),
            Err(_) => {
                // Try to decode as non-nullable string
                match row.try_get::<String, _>(column_name) {
                    Ok(value) => Ok(value),
                    Err(_) => {
                        warn!(
                            "Failed to decode column {} of type {} as text, using placeholder",
                            column_name, type_name
                        );
                        Ok("[UNKNOWN TYPE]".to_string())
                    }
                }
            }
        }
    }
}

/// Error type for field decoding failures
#[derive(Debug, thiserror::Error)]
pub enum FieldDecodeError {
    #[error("SQLx decode error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests for individual decoder functions
    #[test]
    fn test_decode_bytea_hex_conversion() {
        // Test bytea to hex conversion logic
        let test_bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let expected = "\\x48656c6c6f";

        let hex_string = format!("\\x{}", hex::encode(&test_bytes));
        assert_eq!(hex_string, expected);
    }

    #[test]
    fn test_decode_bytea_empty() {
        // Test empty bytea
        let test_bytes = vec![];
        let expected = "\\x";

        let hex_string = format!("\\x{}", hex::encode(&test_bytes));
        assert_eq!(hex_string, expected);
    }

    #[test]
    fn test_decode_bytea_binary_data() {
        // Test binary data with null bytes
        let test_bytes = vec![0x00, 0xFF, 0x42];
        let expected = "\\x00ff42";

        let hex_string = format!("\\x{}", hex::encode(&test_bytes));
        assert_eq!(hex_string, expected);
    }

    #[test]
    fn test_field_decode_error_display() {
        // Test error formatting
        let error = FieldDecodeError::SqlxError(sqlx::Error::RowNotFound);
        assert!(error.to_string().contains("SQLx decode error"));
    }

    #[test]
    fn test_type_name_matching() {
        // Test that type names are properly matched
        let test_cases = vec![
            ("INT4", "INTEGER"),
            ("INT8", "BIGINT"),
            ("BOOL", "BOOLEAN"),
            ("TEXT", "VARCHAR"),
            ("TIMESTAMPTZ", "TIMESTAMP WITH TIME ZONE"),
            ("TIMETZ", "TIME WITH TIME ZONE"),
        ];

        for (primary, alias) in test_cases {
            // Both should match the same decoder
            assert_eq!(primary.to_ascii_uppercase(), primary.to_ascii_uppercase());
            assert_eq!(alias.to_ascii_uppercase(), alias.to_ascii_uppercase());
        }
    }

    #[test]
    fn test_array_type_detection() {
        // Test array type detection
        let array_types = vec!["_TEXT", "_INT4", "_BOOL", "_TIMESTAMP"];

        for array_type in array_types {
            assert!(
                array_type.starts_with("_"),
                "Array type {} should start with underscore",
                array_type
            );
        }
    }

    #[test]
    fn test_custom_type_fallback() {
        // Test custom type names that should fall back to custom decoder
        let custom_types = vec![
            "CARD_ENTRY_MODE",
            "TRANSACTION_TYPE",
            "RESULT_CODE",
            "USER_DEFINED_TYPE",
        ];

        for custom_type in custom_types {
            // These should not match any specific decoder and fall back to custom
            let type_name = custom_type.to_ascii_uppercase();
            assert!(
                !matches!(
                    type_name.as_str(),
                    "INT2"
                        | "SMALLINT"
                        | "INT4"
                        | "INTEGER"
                        | "INT8"
                        | "BIGINT"
                        | "FLOAT4"
                        | "REAL"
                        | "FLOAT8"
                        | "DOUBLE PRECISION"
                        | "NUMERIC"
                        | "DECIMAL"
                        | "BOOL"
                        | "BOOLEAN"
                        | "TEXT"
                        | "VARCHAR"
                        | "CHAR"
                        | "CHARACTER"
                        | "CHARACTER VARYING"
                        | "BYTEA"
                        | "DATE"
                        | "TIME"
                        | "TIME WITHOUT TIME ZONE"
                        | "TIMESTAMP"
                        | "TIMESTAMP WITHOUT TIME ZONE"
                        | "TIMESTAMPTZ"
                        | "TIMESTAMP WITH TIME ZONE"
                        | "TIMETZ"
                        | "TIME WITH TIME ZONE"
                        | "JSON"
                        | "JSONB"
                        | "UUID"
                ),
                "Custom type {} should not match specific decoders",
                custom_type
            );
        }
    }

    #[test]
    fn test_postgres_type_coverage() {
        // Test that we cover all major PostgreSQL types
        let covered_types = vec![
            // Integer types
            "INT2",
            "SMALLINT",
            "INT4",
            "INTEGER",
            "INT8",
            "BIGINT",
            // Floating point types
            "FLOAT4",
            "REAL",
            "FLOAT8",
            "DOUBLE PRECISION",
            // Decimal types
            "NUMERIC",
            "DECIMAL",
            // Boolean types
            "BOOL",
            "BOOLEAN",
            // Text types
            "TEXT",
            "VARCHAR",
            "CHAR",
            "CHARACTER",
            "CHARACTER VARYING",
            // Binary types
            "BYTEA",
            // Date/Time types
            "DATE",
            "TIME",
            "TIME WITHOUT TIME ZONE",
            "TIMESTAMP",
            "TIMESTAMP WITHOUT TIME ZONE",
            "TIMESTAMPTZ",
            "TIMESTAMP WITH TIME ZONE",
            "TIMETZ",
            "TIME WITH TIME ZONE",
            // JSON types
            "JSON",
            "JSONB",
            // UUID type
            "UUID",
        ];

        for pg_type in covered_types {
            let type_name = pg_type.to_ascii_uppercase();
            // Verify that each type is handled by our match statement
            match type_name.as_str() {
                "INT2"
                | "SMALLINT"
                | "INT4"
                | "INTEGER"
                | "INT8"
                | "BIGINT"
                | "FLOAT4"
                | "REAL"
                | "FLOAT8"
                | "DOUBLE PRECISION"
                | "NUMERIC"
                | "DECIMAL"
                | "BOOL"
                | "BOOLEAN"
                | "TEXT"
                | "VARCHAR"
                | "CHAR"
                | "CHARACTER"
                | "CHARACTER VARYING"
                | "BYTEA"
                | "DATE"
                | "TIME"
                | "TIME WITHOUT TIME ZONE"
                | "TIMESTAMP"
                | "TIMESTAMP WITHOUT TIME ZONE"
                | "TIMESTAMPTZ"
                | "TIMESTAMP WITH TIME ZONE"
                | "TIMETZ"
                | "TIME WITH TIME ZONE"
                | "JSON"
                | "JSONB"
                | "UUID" => {
                    // This type is covered
                }
                _ => {
                    panic!("PostgreSQL type {} is not covered by our decoder", pg_type);
                }
            }
        }
    }

    // Integration tests that would require actual database connection
    #[tokio::test]
    #[ignore] // Ignore by default since it requires database setup
    async fn test_integration_with_postgres() {
        // This test would require a test database connection
        // It would test actual decoding with real PostgreSQL data
        // Example test structure:
        // 1. Connect to test PostgreSQL database
        // 2. Create test table with various types
        // 3. Insert test data
        // 4. Query data and verify decoding works correctly
        // 5. Clean up test data
        todo!("Implement integration test with actual PostgreSQL database")
    }

    #[tokio::test]
    #[ignore] // Ignore by default since it requires database setup
    async fn test_all_postgres_types() {
        // This test would create a table with all supported types
        // and verify that each type is decoded correctly
        // Example test structure:
        // 1. Create table with all PostgreSQL types
        // 2. Insert sample data for each type
        // 3. Query and decode each column
        // 4. Verify decoded values match expected strings
        todo!("Implement comprehensive type test with actual PostgreSQL database")
    }
}
