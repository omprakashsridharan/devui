# DevUI - PostgreSQL SQL Features

A comprehensive Rust library providing PostgreSQL database management capabilities through a modern React-based web interface. This library offers a rich set of features for exploring, querying, and managing PostgreSQL databases.

## Features

### 🗄️ **Database Schema Exploration**

- **Table Discovery**: Automatically discover all tables and their schemas
- **Column Metadata**: Rich column information including:
  - Data types with PostgreSQL-specific type information
  - Nullability constraints
  - Primary key identification
  - Default values
  - ENUM type values (automatic extraction)
  - Foreign key relationships with referenced table information

### 📊 **Advanced Data Filtering**

Comprehensive filtering system with support for all PostgreSQL data types:

#### Filter Operations

| Operation | Syntax | Description | Example |
|-----------|--------|-------------|---------|
| Equals | `eq:value` or just `value` | Exact match | `eq:42` or `42` |
| Contains | `contains:text` | Case-insensitive text search | `contains:john` |
| Starts With | `startswith:text` | Text prefix match | `startswith:admin` |
| Ends With | `endswith:text` | Text suffix match | `endswith:.com` |
| Greater Than | `gt:100` | Numeric/date comparison | `gt:2023-01-01` |
| Less Than | `lt:100` | Numeric/date comparison | `lt:50` |
| Greater/Equal | `gte:100` | Numeric/date comparison | `gte:100` |
| Less/Equal | `lte:100` | Numeric/date comparison | `lte:100` |
| Not Equals | `ne:value` | Exclusion | `ne:deleted` |
| Is Null | `null:` | Null check | `null:` |
| Is Not Null | `notnull:` | Non-null check | `notnull:` |
| In List | `in:val1,val2,val3` | Multiple values | `in:active,inactive` |
| Not In | `notin:val1,val2` | Exclusion list | `notin:deleted,archived` |

#### Type-Specific Handling

- **Numeric Types** (`integer`, `bigint`, `smallint`, `numeric`, `decimal`, `real`, `double precision`):
  - Default operation: Exact match
  - Supports all comparison operators
  - Automatic numeric validation

- **Text Types** (`text`, `varchar`, `char`):
  - Default operation: Case-insensitive contains (ILIKE)
  - Supports pattern matching operations
  - Lexicographic comparisons

- **Boolean Types** (`boolean`, `bool`):
  - Default operation: Exact match
  - Flexible parsing: `true`, `false`, `1`, `0`, `yes`, `no`, `on`, `off`, `t`, `f`, `y`, `n`

- **Date/Time Types** (`timestamp`, `timestamptz`, `date`, `time`, `timetz`):
  - Default operation: Text search
  - Supports comparison operators for date ranges
  - Flexible date format parsing

- **JSON Types** (`json`, `jsonb`):
  - Default operation: Text search within JSON
  - Full-text search capabilities

- **UUID Type**:
  - Exact match only
  - Proper UUID format validation

- **ENUM Types**:
  - Automatic detection and value extraction
  - Dropdown selection in UI for enum columns
  - Full enum value list in metadata

### 🔗 **Foreign Key Navigation**

- **Automatic Detection**: Identifies foreign key relationships from PostgreSQL system catalogs
- **Visual Indicators**: Foreign key columns marked with "FK" badges
- **One-Click Navigation**: Click foreign key values to navigate to referenced tables
- **Smart Filtering**: Automatically applies filters when navigating via foreign keys
- **Breadcrumb Tracking**: Visual indicators show when viewing filtered data from foreign key navigation

### 📄 **Pagination**

- **Server-Side Pagination**: Efficient pagination at the database level
- **Configurable Page Size**: Adjustable records per page (default: 10)
- **Total Row Count**: Accurate count regardless of pagination
- **URL Parameters**: Pagination state in URL for bookmarking

### 🎨 **Modern Web Interface**

#### Table View Features

- **Sticky Headers**: Table headers remain visible while scrolling
- **Responsive Layout**: Adapts to different screen sizes
- **Column Tooltips**: Rich metadata on hover:
  - Data type information
  - Nullability status
  - Primary key indicators
  - Foreign key relationships
  - Default values
  - ENUM allowed values

#### Data Display

- **Type-Aware Rendering**: Proper formatting for different data types
  - Dates/timestamps with proper formatting
  - Numeric values with appropriate precision
  - Boolean values as checkmarks/indicators
  - JSON formatted with syntax highlighting
  - Arrays displayed as lists
  - Binary data (BYTEA) as hex strings

- **Null Handling**: Clear indication of NULL values
- **Truncation**: Long values truncated with tooltips showing full content
- **Sortable Columns**: Click column headers to sort (if implemented)

#### Filter UI

- **Dynamic Input Types**:
  - Text inputs for text columns
  - Number inputs for numeric columns
  - Date/time pickers for temporal columns
  - Dropdown selects for ENUM columns
  - Checkboxes for boolean columns

- **Filter Indicators**: Clear visual feedback when filters are active
- **Quick Clear**: One-click filter clearing

### 🛡️ **Security**

- **SQL Injection Prevention**: All user input properly escaped
- **Parameterized Queries**: Uses SQLx for safe query execution
- **Table Name Validation**: Proper escaping of table and schema names
- **Connection Pooling**: Secure connection management

## Supported PostgreSQL Data Types

### Numeric Types
- `SMALLINT` / `INT2`
- `INTEGER` / `INT4`
- `BIGINT` / `INT8`
- `NUMERIC` / `DECIMAL`
- `REAL` / `FLOAT4`
- `DOUBLE PRECISION` / `FLOAT8`

### Text Types
- `TEXT`
- `VARCHAR(n)`
- `CHAR(n)` / `CHARACTER(n)`
- `CHARACTER VARYING(n)`

### Date/Time Types
- `DATE`
- `TIME` / `TIME WITHOUT TIME ZONE`
- `TIMESTAMP` / `TIMESTAMP WITHOUT TIME ZONE`
- `TIMESTAMPTZ` / `TIMESTAMP WITH TIME ZONE`
- `TIMETZ` / `TIME WITH TIME ZONE`

### Boolean Types
- `BOOLEAN` / `BOOL`

### Binary Types
- `BYTEA`

### JSON Types
- `JSON`
- `JSONB`

### UUID Types
- `UUID`

### Array Types
- Array of any base type (e.g., `INTEGER[]`, `TEXT[]`)

### ENUM Types
- User-defined ENUM types (automatic value extraction)

### Custom Types
- User-defined composite types (cast to text)

## API Endpoints

### List Tables
```
GET /services/sql/connections/{connection_id}/tables
```

Returns a list of all tables with complete column metadata.

**Response:**
```json
[
  {
    "name": "users",
    "schema": "public",
    "columns": [
      {
        "name": "id",
        "data_type": "integer",
        "is_nullable": false,
        "is_primary_key": true,
        "default_value": "nextval('users_id_seq'::regclass)",
        "enum_values": null,
        "foreign_key": null
      },
      {
        "name": "status",
        "data_type": "USER-DEFINED",
        "is_nullable": false,
        "is_primary_key": false,
        "default_value": null,
        "enum_values": ["active", "inactive", "pending"],
        "foreign_key": null
      },
      {
        "name": "department_id",
        "data_type": "integer",
        "is_nullable": true,
        "is_primary_key": false,
        "default_value": null,
        "enum_values": null,
        "foreign_key": {
          "referenced_table": "departments",
          "referenced_schema": "public",
          "referenced_column": "id"
        }
      }
    ]
  }
]
```

### Get Table Data
```
GET /services/sql/connections/{connection_id}/tables/{table_name}?page=1&page_size=10&column1=value1&column2=gt:100
```

Retrieve paginated table data with optional filters.

**Query Parameters:**
- `page`: Page number (default: 1)
- `page_size`: Records per page (default: 10)
- `{column_name}`: Filter values (supports operation prefixes)

**Response:**
```json
{
  "columns": ["id", "name", "email", "status", "created_at"],
  "data": [
    {
      "id": "1",
      "name": "John Doe",
      "email": "john@example.com",
      "status": "active",
      "created_at": "2023-01-01 12:00:00"
    }
  ],
  "total_rows": 150,
  "column_info": [
    {
      "name": "id",
      "data_type": "integer",
      "is_nullable": false,
      "is_primary_key": true,
      "default_value": null,
      "enum_values": null,
      "foreign_key": null
    }
  ]
}
```

## Usage Example

```rust
use devui::services::sql::{SqlService, DatabaseConfig};
use devui::services::sql::postgres::PostgresConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create PostgreSQL configuration
    let config = DatabaseConfig::Postgres(PostgresConfig {
        host: "localhost".to_string(),
        port: 5432,
        database: "mydb".to_string(),
        username: "user".to_string(),
        password: "password".to_string(),
    });

    // Create SQL service
    let sql_service = SqlService::new();

    // Add connection
    sql_service
        .add_connection("my_connection".to_string(), config)
        .await?;

    // List tables
    let tables = sql_service
        .tables("my_connection".to_string())
        .await?;

    println!("Found {} tables", tables.len());

    // Get table data with filters
    let filters = std::collections::HashMap::from([
        ("status".to_string(), "active".to_string()),
        ("age".to_string(), "gt:18".to_string()),
    ]);

    let table_data = sql_service
        .table_data(
            "my_connection".to_string(),
            "users".to_string(),
            Some(filters),
            Some(1),    // page
            Some(10),   // page_size
        )
        .await?;

    println!("Total rows: {}", table_data.total_rows);
    println!("Showing {} rows", table_data.rows.len());

    Ok(())
}
```

## Architecture

### Backend (Rust)

- **Connection Pool**: SQLx-based connection pooling for PostgreSQL
- **Type System**: Comprehensive type detection and handling
- **Filter Engine**: Powerful filter builder with type-aware operations
- **Schema Discovery**: Automatic schema introspection using `information_schema`
- **Foreign Key Resolution**: Automatic foreign key relationship discovery

### Frontend (React + TypeScript)

- **Material-UI**: Modern, responsive UI components
- **Type-Safe API Client**: Fully typed API interactions
- **State Management**: React hooks for efficient state handling
- **Real-time Updates**: Live data refresh capabilities

## Requirements

- Rust 1.70+
- PostgreSQL 12+
- Node.js 18+ (for frontend development)

## Performance

- **Connection Pooling**: Efficient database connection management
- **Server-Side Pagination**: Only fetches requested data
- **Lazy Loading**: Tables and columns loaded on demand
- **Optimized Queries**: Minimal round trips to database

## Security Considerations

- All user input is properly escaped to prevent SQL injection
- Connection credentials should be stored securely (not hardcoded)
- Consider using environment variables or secure configuration management
- Table names are validated and escaped before use in queries
- Filter values are properly parameterized

## Screenshots

> **Note**: To add screenshots, capture the following UI views and place them in a `docs/screenshots/` directory:
>
> 1. **Main Table List View** (`table-list.png`): Shows the schema browser with expandable tables and column details
> 2. **Table Data View** (`table-data.png`): Displays table data with filters, pagination controls, and row count
> 3. **Foreign Key Navigation** (`foreign-key-navigation.png`): Shows clicking on a foreign key value and navigating to the referenced table
> 4. **Filter UI** (`filter-ui.png`): Demonstrates different input types (enum dropdown, date picker, text input)
> 5. **Column Metadata Tooltips** (`column-tooltips.png`): Shows the rich metadata displayed on column hover
> 6. **Foreign Key Filter Indicator** (`fk-filter-banner.png`): Shows the banner indicating filtered view from foreign key navigation
> 7. **Highlighted Row** (`highlighted-row.png`): Shows the highlighted row matching the foreign key filter
>
> Then update this section with:
> ```markdown
> ![Table List View](docs/screenshots/table-list.png)
> ![Table Data View](docs/screenshots/table-data.png)
> ![Foreign Key Navigation](docs/screenshots/foreign-key-navigation.png)
> ```

## License

MIT

