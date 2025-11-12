-- Initialize sample database with relational tables and fake data
-- This script runs automatically when the PostgreSQL container starts

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

-- Products table
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    stock_quantity INTEGER DEFAULT 0,
    category_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_available BOOLEAN DEFAULT TRUE
);

-- Categories table
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    parent_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Orders table
CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    total_amount DECIMAL(10, 2) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Order items table (junction table)
CREATE TABLE IF NOT EXISTS order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL,
    product_id UUID NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    subtotal DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE RESTRICT
);

-- Add foreign key constraint for products.category_id
ALTER TABLE products
ADD CONSTRAINT fk_products_category
FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL;

-- Add foreign key constraint for categories.parent_id (self-referential)
ALTER TABLE categories
ADD CONSTRAINT fk_categories_parent
FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE SET NULL;

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at);
CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);
CREATE INDEX IF NOT EXISTS idx_order_items_product_id ON order_items(product_id);
CREATE INDEX IF NOT EXISTS idx_products_category_id ON products(category_id);
CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id);

-- Insert sample categories
INSERT INTO categories (id, name, description, parent_id) VALUES
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'Electronics', 'Electronic devices and accessories', NULL),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'Computers', 'Desktops, laptops, and tablets', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', 'Phones', 'Smartphones and accessories', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14', 'Books', 'Physical and digital books', NULL),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a15', 'Fiction', 'Fiction novels and stories', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14'),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a16', 'Non-Fiction', 'Educational and informational books', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a14'),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a17', 'Clothing', 'Apparel and fashion items', NULL),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a18', 'Men''s Clothing', 'Clothing for men', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a17'),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a19', 'Women''s Clothing', 'Clothing for women', 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a17')
ON CONFLICT (id) DO NOTHING;

-- Insert sample users
INSERT INTO users (id, email, first_name, last_name, is_active) VALUES
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a21', 'alice.johnson@example.com', 'Alice', 'Johnson', TRUE),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a22', 'bob.smith@example.com', 'Bob', 'Smith', TRUE),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a23', 'carol.williams@example.com', 'Carol', 'Williams', TRUE),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a24', 'david.brown@example.com', 'David', 'Brown', TRUE),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a25', 'eve.davis@example.com', 'Eve', 'Davis', TRUE),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a26', 'frank.miller@example.com', 'Frank', 'Miller', FALSE),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a27', 'grace.wilson@example.com', 'Grace', 'Wilson', TRUE),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a28', 'henry.moore@example.com', 'Henry', 'Moore', TRUE)
ON CONFLICT (id) DO NOTHING;

-- Insert sample products
INSERT INTO products (id, name, description, price, stock_quantity, category_id, is_available) VALUES
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a31', 'MacBook Pro 16"', 'Apple MacBook Pro with M2 chip, 16GB RAM, 512GB SSD', 2499.99, 15, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a32', 'iPhone 15 Pro', 'Apple iPhone 15 Pro, 256GB, Titanium Blue', 999.99, 30, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a13', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a33', 'iPad Air', 'Apple iPad Air, M1 chip, 256GB, Wi-Fi', 749.99, 20, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a34', 'The Great Gatsby', 'Classic American novel by F. Scott Fitzgerald', 12.99, 100, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a15', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a35', 'Clean Code', 'Software craftsmanship handbook by Robert C. Martin', 39.99, 75, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a16', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a36', 'Men''s T-Shirt', '100% Cotton, Premium Quality T-Shirt', 24.99, 200, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a18', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a37', 'Women''s Jeans', 'High-waisted skinny jeans, multiple sizes', 59.99, 150, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a19', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a38', 'Gaming Laptop', 'High-performance gaming laptop, RTX 4070, 32GB RAM', 1899.99, 8, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a39', 'Wireless Earbuds', 'Premium noise-cancelling wireless earbuds', 199.99, 50, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', TRUE),
('c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a40', 'To Kill a Mockingbird', 'Harper Lee''s masterpiece novel', 14.99, 90, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a15', TRUE)
ON CONFLICT (id) DO NOTHING;

-- Insert sample orders
INSERT INTO orders (id, user_id, total_amount, status, created_at) VALUES
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a41', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a21', 999.99, 'completed', CURRENT_TIMESTAMP - INTERVAL '5 days'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a42', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a22', 1899.99, 'shipped', CURRENT_TIMESTAMP - INTERVAL '3 days'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a43', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a23', 64.98, 'pending', CURRENT_TIMESTAMP - INTERVAL '1 day'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a44', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a24', 749.99, 'processing', CURRENT_TIMESTAMP - INTERVAL '2 hours'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a45', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a25', 199.99, 'completed', CURRENT_TIMESTAMP - INTERVAL '7 days'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a46', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a27', 2499.99, 'shipped', CURRENT_TIMESTAMP - INTERVAL '4 days'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a47', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a28', 27.98, 'pending', CURRENT_TIMESTAMP - INTERVAL '30 minutes'),
('d3eebc99-9c0b-4ef8-bb6d-6bb9bd380a48', 'b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a21', 1749.98, 'processing', CURRENT_TIMESTAMP - INTERVAL '12 hours')
ON CONFLICT (id) DO NOTHING;

-- Insert sample order items
INSERT INTO order_items (id, order_id, product_id, quantity, unit_price, subtotal) VALUES
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a51', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a41', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a32', 1, 999.99, 999.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a52', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a42', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a38', 1, 1899.99, 1899.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a53', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a43', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a36', 2, 24.99, 49.98),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a54', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a43', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a35', 1, 14.99, 14.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a55', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a44', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a33', 1, 749.99, 749.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a56', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a45', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a39', 1, 199.99, 199.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a57', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a46', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a31', 1, 2499.99, 2499.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a58', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a47', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a34', 1, 12.99, 12.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a59', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a47', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a40', 1, 14.99, 14.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a60', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a48', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a33', 1, 749.99, 749.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a61', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a48', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a39', 1, 199.99, 199.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a62', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a48', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a35', 1, 39.99, 39.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a63', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a48', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a37', 1, 59.99, 59.99),
('e4eebc99-9c0b-4ef8-bb6d-6bb9bd380a64', 'd3eebc99-9c0b-4ef8-bb6d-6bb9bd380a48', 'c2eebc99-9c0b-4ef8-bb6d-6bb9bd380a36', 4, 24.99, 99.96)
ON CONFLICT (id) DO NOTHING;

-- ============================================================================
-- Comprehensive PostgreSQL Data Types Test Table
-- This table includes an enum type and all possible PostgreSQL data types
-- to test if the code handles and parses them correctly
-- ============================================================================

-- Create enum type for testing
CREATE TYPE order_status_enum AS ENUM ('pending', 'processing', 'shipped', 'delivered', 'cancelled', 'refunded');

-- Create comprehensive test table with all PostgreSQL data types
CREATE TABLE IF NOT EXISTS postgres_types_test (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Enum type (USER-DEFINED)
    status_enum order_status_enum NOT NULL DEFAULT 'pending',

    -- Integer types
    col_smallint SMALLINT,
    col_integer INTEGER,
    col_bigint BIGINT,
    col_smallserial SMALLSERIAL,
    col_serial SERIAL,
    col_bigserial BIGSERIAL,

    -- Floating point types
    col_real REAL,
    col_double_precision DOUBLE PRECISION,

    -- Decimal/Numeric types
    col_numeric NUMERIC(10, 2),
    col_decimal DECIMAL(15, 5),
    col_money MONEY,

    -- Boolean types
    col_boolean BOOLEAN,
    col_bool BOOL,

    -- Text types
    col_char CHAR(10),
    col_varchar VARCHAR(255),
    col_text TEXT,
    col_char_varying CHARACTER VARYING(100),

    -- Binary types
    col_bytea BYTEA,

    -- Date/Time types
    col_date DATE,
    col_time TIME WITHOUT TIME ZONE,
    col_time_tz TIME WITH TIME ZONE,
    col_timestamp TIMESTAMP WITHOUT TIME ZONE,
    col_timestamp_tz TIMESTAMP WITH TIME ZONE,
    col_interval INTERVAL,
    col_interval_year INTERVAL,
    col_interval_month INTERVAL,
    col_interval_day INTERVAL,
    col_interval_hour INTERVAL,
    col_interval_minute INTERVAL,
    col_interval_second INTERVAL,

    -- JSON types
    col_json JSON,
    col_jsonb JSONB,

    -- UUID type
    col_uuid UUID,

    -- Network types
    col_inet INET,
    col_cidr CIDR,
    col_macaddr MACADDR,
    col_macaddr8 MACADDR8,

    -- Geometric types
    col_point POINT,
    col_line LINE,
    col_lseg LSEG,
    col_box BOX,
    col_path PATH,
    col_polygon POLYGON,
    col_circle CIRCLE,

    -- Range types
    col_int4range INT4RANGE,
    col_int8range INT8RANGE,
    col_numrange NUMRANGE,
    col_tsrange TSRANGE,
    col_tstzrange TSTZRANGE,
    col_daterange DATERANGE,

    -- Array types (various base types)
    col_text_array TEXT[],
    col_integer_array INTEGER[],
    col_boolean_array BOOLEAN[],
    col_uuid_array UUID[],
    col_timestamp_array TIMESTAMP[],
    col_numeric_array NUMERIC(10,2)[],

    -- Bit string types
    col_bit BIT(8),
    col_bit_varying BIT VARYING(16),
    col_varbit VARBIT(32),

    -- XML type
    col_xml XML,

    -- Created timestamp
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index on enum column
CREATE INDEX IF NOT EXISTS idx_postgres_types_test_status_enum ON postgres_types_test(status_enum);

-- Insert sample data with all data types
INSERT INTO postgres_types_test (
    status_enum,
    col_smallint,
    col_integer,
    col_bigint,
    col_smallserial,
    col_serial,
    col_bigserial,
    col_real,
    col_double_precision,
    col_numeric,
    col_decimal,
    col_money,
    col_boolean,
    col_bool,
    col_char,
    col_varchar,
    col_text,
    col_char_varying,
    col_bytea,
    col_date,
    col_time,
    col_time_tz,
    col_timestamp,
    col_timestamp_tz,
    col_interval,
    col_interval_year,
    col_interval_month,
    col_interval_day,
    col_interval_hour,
    col_interval_minute,
    col_interval_second,
    col_json,
    col_jsonb,
    col_uuid,
    col_inet,
    col_cidr,
    col_macaddr,
    col_macaddr8,
    col_point,
    col_line,
    col_lseg,
    col_box,
    col_path,
    col_polygon,
    col_circle,
    col_int4range,
    col_int8range,
    col_numrange,
    col_tsrange,
    col_tstzrange,
    col_daterange,
    col_text_array,
    col_integer_array,
    col_boolean_array,
    col_uuid_array,
    col_timestamp_array,
    col_numeric_array,
    col_bit,
    col_bit_varying,
    col_varbit,
    col_xml
) VALUES
(
    'pending',                          -- status_enum
    32767,                              -- col_smallint
    2147483647,                         -- col_integer
    9223372036854775807,                -- col_bigint
    DEFAULT,                            -- col_smallserial (auto-generated)
    DEFAULT,                            -- col_serial (auto-generated)
    DEFAULT,                            -- col_bigserial (auto-generated)
    3.14159,                            -- col_real
    3.141592653589793,                  -- col_double_precision
    12345.67,                           -- col_numeric
    987654321.12345,                    -- col_decimal
    1000.50,                            -- col_money
    TRUE,                               -- col_boolean
    FALSE,                              -- col_bool
    'Hello',                            -- col_char
    'Variable length string',            -- col_varchar
    'This is a very long text field that can contain multiple lines and paragraphs of text.', -- col_text
    'Character varying',                 -- col_char_varying
    E'\\x48656c6c6f20576f726c64'::bytea, -- col_bytea (Hello World in hex)
    '2024-01-15',                       -- col_date
    '14:30:00',                         -- col_time
    '14:30:00+05:30',                   -- col_time_tz
    '2024-01-15 14:30:00',              -- col_timestamp
    '2024-01-15 14:30:00+00:00',        -- col_timestamp_tz
    '1 day 2 hours 3 minutes',          -- col_interval
    '1 year',                           -- col_interval_year
    '6 months',                         -- col_interval_month
    '30 days',                          -- col_interval_day
    '12 hours',                         -- col_interval_hour
    '45 minutes',                       -- col_interval_minute
    '30 seconds',                       -- col_interval_second
    '{"key": "value", "number": 42}',   -- col_json
    '{"key": "value", "number": 42, "nested": {"foo": "bar"}}'::jsonb, -- col_jsonb
    '550e8400-e29b-41d4-a716-446655440000', -- col_uuid
    '192.168.1.1',                      -- col_inet
    '192.168.1.0/24',                   -- col_cidr
    '08:00:2b:01:02:03',                -- col_macaddr
    '08:00:2b:01:02:03:04:05',          -- col_macaddr8
    '(10,20)',                          -- col_point
    '{1,2,3}',                          -- col_line
    '[(1,1),(2,2)]',                    -- col_lseg
    '((1,1),(3,3))',                    -- col_box
    '[(0,0),(1,1),(2,0)]',              -- col_path
    '((0,0),(1,1),(1,0),(0,0))',        -- col_polygon
    '<(5,5),3>',                        -- col_circle
    '[1,10)',                           -- col_int4range
    '[100,1000)',                       -- col_int8range
    '[1.5,99.9]',                       -- col_numrange
    '[2024-01-01 00:00:00,2024-12-31 23:59:59)', -- col_tsrange
    '[2024-01-01 00:00:00+00,2024-12-31 23:59:59+00)', -- col_tstzrange
    '[2024-01-01,2024-12-31)',         -- col_daterange
    ARRAY['apple', 'banana', 'cherry'], -- col_text_array
    ARRAY[1, 2, 3, 4, 5],               -- col_integer_array
    ARRAY[TRUE, FALSE, TRUE],           -- col_boolean_array
    ARRAY['550e8400-e29b-41d4-a716-446655440000'::uuid, '660e8400-e29b-41d4-a716-446655440001'::uuid], -- col_uuid_array
    ARRAY['2024-01-01 00:00:00'::timestamp, '2024-01-02 00:00:00'::timestamp], -- col_timestamp_array
    ARRAY[10.5, 20.75, 30.25],          -- col_numeric_array
    B'10101010',                        -- col_bit
    B'111100001111',                    -- col_bit_varying
    B'111111110000000011111111',        -- col_varbit
    '<root><element>value</element></root>'::xml -- col_xml
),
(
    'processing',                       -- status_enum
    -32768,                             -- col_smallint (negative)
    0,                                  -- col_integer (zero)
    -9223372036854775808,               -- col_bigint (negative)
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    -3.14159,                           -- col_real (negative)
    -3.141592653589793,                 -- col_double_precision (negative)
    -12345.67,                          -- col_numeric (negative)
    -987654321.12345,                   -- col_decimal (negative)
    -1000.50,                           -- col_money (negative)
    FALSE,                              -- col_boolean
    TRUE,                               -- col_bool
    'World',                            -- col_char
    'Another string',                   -- col_varchar
    'Another long text field with different content.', -- col_text
    'Another varying',                  -- col_char_varying
    E'\\x426172'::bytea,                -- col_bytea (Bar in hex)
    '2024-06-20',                       -- col_date
    '23:59:59',                         -- col_time
    '23:59:59-08:00',                   -- col_time_tz
    '2024-06-20 23:59:59',              -- col_timestamp
    '2024-06-20 23:59:59+00:00',       -- col_timestamp_tz
    '2 years 3 months',                 -- col_interval
    '2 years',                          -- col_interval_year
    '12 months',                        -- col_interval_month
    '60 days',                          -- col_interval_day
    '24 hours',                         -- col_interval_hour
    '59 minutes',                       -- col_interval_minute
    '59 seconds',                       -- col_interval_second
    '{"array": [1, 2, 3], "null": null}', -- col_json
    '{"array": [1, 2, 3], "null": null, "boolean": true}'::jsonb, -- col_jsonb
    '660e8400-e29b-41d4-a716-446655440001', -- col_uuid
    '10.0.0.1',                         -- col_inet
    '10.0.0.0/8',                       -- col_cidr
    'aa:bb:cc:dd:ee:ff',                -- col_macaddr
    'aa:bb:cc:dd:ee:ff:00:11',          -- col_macaddr8
    '(100,200)',                        -- col_point
    '{4,5,6}',                          -- col_line
    '[(10,10),(20,20)]',                -- col_lseg
    '((10,10),(30,30))',                -- col_box
    '[(0,0),(10,10),(20,0)]',           -- col_path
    '((0,0),(10,10),(10,0),(0,0))',    -- col_polygon
    '<(50,50),10>',                     -- col_circle
    '[100,200]',                        -- col_int4range
    '[1000,2000]',                      -- col_int8range
    '[10.1,99.9)',                      -- col_numrange
    '[2023-01-01 00:00:00,2023-12-31 23:59:59)', -- col_tsrange
    '[2023-01-01 00:00:00+00,2023-12-31 23:59:59+00)', -- col_tstzrange
    '[2023-01-01,2023-12-31]',         -- col_daterange
    ARRAY['dog', 'cat'],                -- col_text_array
    ARRAY[10, 20, 30],                  -- col_integer_array
    ARRAY[FALSE, TRUE, FALSE],          -- col_boolean_array
    ARRAY['770e8400-e29b-41d4-a716-446655440002'::uuid], -- col_uuid_array
    ARRAY['2023-12-31 23:59:59'::timestamp], -- col_timestamp_array
    ARRAY[100.1, 200.2],                -- col_numeric_array
    B'00001111',                        -- col_bit
    B'000000001111',                    -- col_bit_varying
    B'000000001111111100000000',        -- col_varbit
    '<root><child>text</child></root>'::xml -- col_xml
),
(
    'shipped',                          -- status_enum
    NULL,                               -- col_smallint (NULL)
    NULL,                               -- col_integer (NULL)
    NULL,                               -- col_bigint (NULL)
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    NULL,                               -- col_real (NULL)
    NULL,                               -- col_double_precision (NULL)
    NULL,                               -- col_numeric (NULL)
    NULL,                               -- col_decimal (NULL)
    NULL,                               -- col_money (NULL)
    NULL,                               -- col_boolean (NULL)
    NULL,                               -- col_bool (NULL)
    NULL,                               -- col_char (NULL)
    NULL,                               -- col_varchar (NULL)
    NULL,                               -- col_text (NULL)
    NULL,                               -- col_char_varying (NULL)
    NULL,                               -- col_bytea (NULL)
    NULL,                               -- col_date (NULL)
    NULL,                               -- col_time (NULL)
    NULL,                               -- col_time_tz (NULL)
    NULL,                               -- col_timestamp (NULL)
    NULL,                               -- col_timestamp_tz (NULL)
    NULL,                               -- col_interval (NULL)
    NULL,                               -- col_interval_year (NULL)
    NULL,                               -- col_interval_month (NULL)
    NULL,                               -- col_interval_day (NULL)
    NULL,                               -- col_interval_hour (NULL)
    NULL,                               -- col_interval_minute (NULL)
    NULL,                               -- col_interval_second (NULL)
    NULL,                               -- col_json (NULL)
    NULL,                               -- col_jsonb (NULL)
    NULL,                               -- col_uuid (NULL)
    NULL,                               -- col_inet (NULL)
    NULL,                               -- col_cidr (NULL)
    NULL,                               -- col_macaddr (NULL)
    NULL,                               -- col_macaddr8 (NULL)
    NULL,                               -- col_point (NULL)
    NULL,                               -- col_line (NULL)
    NULL,                               -- col_lseg (NULL)
    NULL,                               -- col_box (NULL)
    NULL,                               -- col_path (NULL)
    NULL,                               -- col_polygon (NULL)
    NULL,                               -- col_circle (NULL)
    NULL,                               -- col_int4range (NULL)
    NULL,                               -- col_int8range (NULL)
    NULL,                               -- col_numrange (NULL)
    NULL,                               -- col_tsrange (NULL)
    NULL,                               -- col_tstzrange (NULL)
    NULL,                               -- col_daterange (NULL)
    NULL,                               -- col_text_array (NULL)
    NULL,                               -- col_integer_array (NULL)
    NULL,                               -- col_boolean_array (NULL)
    NULL,                               -- col_uuid_array (NULL)
    NULL,                               -- col_timestamp_array (NULL)
    NULL,                               -- col_numeric_array (NULL)
    NULL,                               -- col_bit (NULL)
    NULL,                               -- col_bit_varying (NULL)
    NULL,                               -- col_varbit (NULL)
    NULL                                -- col_xml (NULL)
),
(
    'delivered',                        -- status_enum
    100,                                -- col_smallint
    1000,                               -- col_integer
    1000000,                            -- col_bigint
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    0.0,                                -- col_real (zero)
    0.0,                                -- col_double_precision (zero)
    0.00,                               -- col_numeric (zero)
    0.00000,                            -- col_decimal (zero)
    0.00,                               -- col_money (zero)
    TRUE,                               -- col_boolean
    TRUE,                               -- col_bool
    'Zero',                             -- col_char
    'Zero values',                      -- col_varchar
    'Testing zero values for numeric types.', -- col_text
    'Zero varying',                     -- col_char_varying
    E'\\x00'::bytea,                    -- col_bytea (single null byte)
    CURRENT_DATE,                       -- col_date (current date)
    CURRENT_TIME,                       -- col_time (current time)
    CURRENT_TIME,                       -- col_time_tz (current time with tz)
    CURRENT_TIMESTAMP,                  -- col_timestamp (current timestamp)
    CURRENT_TIMESTAMP,                  -- col_timestamp_tz (current timestamp with tz)
    '0',                                -- col_interval (zero interval)
    '0 years',                          -- col_interval_year
    '0 months',                         -- col_interval_month
    '0 days',                           -- col_interval_day
    '0 hours',                          -- col_interval_hour
    '0 minutes',                        -- col_interval_minute
    '0 seconds',                        -- col_interval_second
    '{}',                               -- col_json (empty object)
    '[]'::jsonb,                        -- col_jsonb (empty array)
    uuid_generate_v4(),                 -- col_uuid (generated)
    '127.0.0.1',                        -- col_inet (localhost)
    '127.0.0.1/32',                     -- col_cidr
    '00:00:00:00:00:00',                -- col_macaddr
    '00:00:00:00:00:00:00:00',          -- col_macaddr8
    '(0,0)',                            -- col_point (origin)
    '{1,0,0}',                          -- col_line (horizontal line: y = 0)
    '[(0,0),(0,0)]',                    -- col_lseg
    '((0,0),(0,0))',                    -- col_box
    '[(0,0)]',                          -- col_path
    '((0,0),(0,0),(0,0))',              -- col_polygon
    '<(0,0),0>',                        -- col_circle
    '[0,0]',                            -- col_int4range (empty range)
    '[0,0]',                            -- col_int8range (empty range)
    '[0,0]',                            -- col_numrange (empty range)
    '[2024-01-01 00:00:00,2024-01-01 00:00:00)', -- col_tsrange
    '[2024-01-01 00:00:00+00,2024-01-01 00:00:00+00)', -- col_tstzrange
    '[2024-01-01,2024-01-01)',         -- col_daterange
    ARRAY[]::TEXT[],                    -- col_text_array (empty array)
    ARRAY[]::INTEGER[],                 -- col_integer_array (empty array)
    ARRAY[]::BOOLEAN[],                 -- col_boolean_array (empty array)
    ARRAY[]::UUID[],                    -- col_uuid_array (empty array)
    ARRAY[]::TIMESTAMP[],               -- col_timestamp_array (empty array)
    ARRAY[]::NUMERIC(10,2)[],           -- col_numeric_array (empty array)
    B'00000000',                        -- col_bit (all zeros)
    B'0',                               -- col_bit_varying (single zero)
    B'0',                               -- col_varbit (single zero)
    '<root/>'::xml                      -- col_xml (empty root)
),
(
    'cancelled',                        -- status_enum
    42,                                 -- col_smallint
    4242,                               -- col_integer
    424242424242,                       -- col_bigint
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    42.42,                              -- col_real
    4242.4242,                          -- col_double_precision
    42.42,                              -- col_numeric
    4242.42424,                         -- col_decimal
    42.42,                              -- col_money
    TRUE,                               -- col_boolean
    FALSE,                              -- col_bool
    'Test',                             -- col_char
    'Test string',                      -- col_varchar
    'Test text content for various data types.', -- col_text
    'Test varying',                     -- col_char_varying
    E'\\x54657374'::bytea,              -- col_bytea (Test in hex)
    '2024-12-25',                       -- col_date
    '12:00:00',                         -- col_time
    '12:00:00+00:00',                   -- col_time_tz
    '2024-12-25 12:00:00',              -- col_timestamp
    '2024-12-25 12:00:00+00:00',       -- col_timestamp_tz
    '1 year 1 month 1 day 1 hour 1 minute 1 second', -- col_interval
    '1 year',                           -- col_interval_year
    '1 month',                          -- col_interval_month
    '1 day',                            -- col_interval_day
    '1 hour',                           -- col_interval_hour
    '1 minute',                         -- col_interval_minute
    '1 second',                         -- col_interval_second
    '{"test": true, "number": 42}',     -- col_json
    '{"test": true, "number": 42, "array": [1,2,3]}'::jsonb, -- col_jsonb
    '880e8400-e29b-41d4-a716-446655440003', -- col_uuid
    '172.16.0.1',                       -- col_inet
    '172.16.0.0/12',                    -- col_cidr
    '12:34:56:78:90:ab',                -- col_macaddr
    '12:34:56:78:90:ab:cd:ef',          -- col_macaddr8
    '(42,42)',                          -- col_point
    '{1,1,0}',                          -- col_line (diagonal line: x + y = 0)
    '[(1,1),(42,42)]',                  -- col_lseg
    '((1,1),(42,42))',                  -- col_box
    '[(0,0),(42,42),(0,42)]',           -- col_path
    '((0,0),(42,42),(42,0),(0,0))',    -- col_polygon
    '<(42,42),21>',                     -- col_circle
    '[42,100)',                         -- col_int4range
    '[4242,10000)',                     -- col_int8range
    '[42.0,100.0]',                     -- col_numrange
    '[2024-06-01 00:00:00,2024-06-30 23:59:59)', -- col_tsrange
    '[2024-06-01 00:00:00+00,2024-06-30 23:59:59+00)', -- col_tstzrange
    '[2024-06-01,2024-06-30)',         -- col_daterange
    ARRAY['test', 'data', 'types'],     -- col_text_array
    ARRAY[42, 100, 200],                -- col_integer_array
    ARRAY[TRUE, TRUE, FALSE],           -- col_boolean_array
    ARRAY['990e8400-e29b-41d4-a716-446655440004'::uuid, 'aa0e8400-e29b-41d4-a716-446655440005'::uuid], -- col_uuid_array
    ARRAY['2024-06-15 12:00:00'::timestamp, '2024-06-16 12:00:00'::timestamp], -- col_timestamp_array
    ARRAY[42.5, 100.75, 200.25],        -- col_numeric_array
    B'11111111',                        -- col_bit (all ones)
    B'111111111111',                    -- col_bit_varying (all ones)
    B'111111111111111111111111',        -- col_varbit (all ones)
    '<root><test>data</test></root>'::xml -- col_xml
),
(
    'refunded',                         -- status_enum
    999,                                -- col_smallint
    999999,                             -- col_integer
    999999999999,                       -- col_bigint
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    999.999,                            -- col_real
    999999.999999,                      -- col_double_precision
    999.99,                             -- col_numeric
    999999.99999,                       -- col_decimal
    999.99,                             -- col_money
    FALSE,                              -- col_boolean
    TRUE,                               -- col_bool
    'Max',                              -- col_char
    'Maximum values test',              -- col_varchar
    'Testing maximum and edge case values for all data types.', -- col_text
    'Max varying',                      -- col_char_varying
    E'\\x4d6178696d756d'::bytea,        -- col_bytea (Maximum in hex)
    '2099-12-31',                       -- col_date (far future)
    '23:59:59.999999',                  -- col_time (max precision)
    '23:59:59.999999+14:00',            -- col_time_tz (max timezone)
    '2099-12-31 23:59:59.999999',       -- col_timestamp (max precision)
    '2099-12-31 23:59:59.999999+14:00', -- col_timestamp_tz (max precision and tz)
    '999 years 11 months 30 days 23 hours 59 minutes 59 seconds', -- col_interval (large)
    '999 years',                        -- col_interval_year
    '11 months',                        -- col_interval_month
    '30 days',                          -- col_interval_day
    '23 hours',                         -- col_interval_hour
    '59 minutes',                       -- col_interval_minute
    '59 seconds',                       -- col_interval_second
    '{"max": 999999, "large": true, "nested": {"deep": {"value": 42}}}', -- col_json
    '{"max": 999999, "large": true, "nested": {"deep": {"value": 42}}, "array": [1,2,3,4,5]}'::jsonb, -- col_jsonb
    'ff0e8400-e29b-41d4-a716-44665544ffff', -- col_uuid
    '255.255.255.255',                  -- col_inet (broadcast)
    '0.0.0.0/0',                        -- col_cidr (all networks)
    'ff:ff:ff:ff:ff:ff',                -- col_macaddr (broadcast)
    'ff:ff:ff:ff:ff:ff:ff:ff',          -- col_macaddr8 (broadcast)
    '(999,999)',                        -- col_point
    '{999,999,999}',                    -- col_line
    '[(0,0),(999,999)]',                -- col_lseg
    '((0,0),(999,999))',                -- col_box
    '[(0,0),(999,999),(0,999)]',       -- col_path
    '((0,0),(999,999),(999,0),(0,0))', -- col_polygon
    '<(999,999),500>',                  -- col_circle
    '[0,2147483647)',                   -- col_int4range (max int4)
    '[0,9223372036854775807)',          -- col_int8range (max int8)
    '[0,999999.99]',                    -- col_numrange
    '[1970-01-01 00:00:00,2099-12-31 23:59:59)', -- col_tsrange (full range)
    '[1970-01-01 00:00:00+00,2099-12-31 23:59:59+00)', -- col_tstzrange (full range)
    '[1970-01-01,2099-12-31)',         -- col_daterange (full range)
    ARRAY['max', 'values', 'test', 'array', 'with', 'many', 'elements'], -- col_text_array (large)
    ARRAY[1,2,3,4,5,6,7,8,9,10],        -- col_integer_array (large)
    ARRAY[TRUE,FALSE,TRUE,FALSE,TRUE],  -- col_boolean_array (large)
    ARRAY['bb0e8400-e29b-41d4-a716-446655440006'::uuid, 'cc0e8400-e29b-41d4-a716-446655440007'::uuid, 'dd0e8400-e29b-41d4-a716-446655440008'::uuid], -- col_uuid_array (large)
    ARRAY['2024-01-01 00:00:00'::timestamp, '2024-06-15 12:00:00'::timestamp, '2024-12-31 23:59:59'::timestamp], -- col_timestamp_array (large)
    ARRAY[999.99, 888.88, 777.77, 666.66, 555.55], -- col_numeric_array (large)
    B'11111111',                        -- col_bit (max 8-bit)
    B'1111111111111111',                -- col_bit_varying (max 16-bit)
    B'11111111111111111111111111111111', -- col_varbit (max 32-bit)
    '<root><max><nested><deep><value>999</value></deep></nested></max></root>'::xml -- col_xml (deep nesting)
),
(
    'pending',                          -- status_enum
    1,                                  -- col_smallint
    100,                                -- col_integer
    1000000000,                         -- col_bigint
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    1.1,                                -- col_real
    1.111111111111111,                  -- col_double_precision
    1.11,                               -- col_numeric
    1.11111,                            -- col_decimal
    1.11,                               -- col_money
    TRUE,                               -- col_boolean
    TRUE,                               -- col_bool
    'One',                              -- col_char
    'Minimal values',                   -- col_varchar
    'Testing minimal non-zero values.', -- col_text
    'Min varying',                     -- col_char_varying
    E'\\x01'::bytea,                    -- col_bytea (single byte)
    '2024-01-01',                       -- col_date
    '00:00:00',                         -- col_time
    '00:00:00+00:00',                   -- col_time_tz
    '2024-01-01 00:00:00',              -- col_timestamp
    '2024-01-01 00:00:00+00:00',       -- col_timestamp_tz
    '1 second',                         -- col_interval
    '1 year',                           -- col_interval_year
    '1 month',                          -- col_interval_month
    '1 day',                            -- col_interval_day
    '1 hour',                           -- col_interval_hour
    '1 minute',                         -- col_interval_minute
    '1 second',                         -- col_interval_second
    '{"min": 1}',                       -- col_json
    '{"min": 1}'::jsonb,                -- col_jsonb
    '00000000-0000-0000-0000-000000000001', -- col_uuid
    '0.0.0.1',                          -- col_inet
    '0.0.0.0/32',                       -- col_cidr
    '00:00:00:00:00:01',                -- col_macaddr
    '00:00:00:00:00:00:00:01',          -- col_macaddr8
    '(1,1)',                            -- col_point
    '{1,1,0}',                          -- col_line (diagonal line: x + y = 0)
    '[(0,0),(1,1)]',                    -- col_lseg
    '((0,0),(1,1))',                    -- col_box
    '[(0,0),(1,1)]',                    -- col_path
    '((0,0),(1,1),(1,0),(0,0))',       -- col_polygon
    '<(1,1),1>',                        -- col_circle
    '[1,2)',                            -- col_int4range
    '[1,2)',                            -- col_int8range
    '[1.0,2.0]',                        -- col_numrange
    '[2024-01-01 00:00:00,2024-01-02 00:00:00)', -- col_tsrange
    '[2024-01-01 00:00:00+00,2024-01-02 00:00:00+00)', -- col_tstzrange
    '[2024-01-01,2024-01-02)',         -- col_daterange
    ARRAY['one'],                       -- col_text_array
    ARRAY[1],                           -- col_integer_array
    ARRAY[TRUE],                        -- col_boolean_array
    ARRAY['00000000-0000-0000-0000-000000000001'::uuid], -- col_uuid_array
    ARRAY['2024-01-01 00:00:00'::timestamp], -- col_timestamp_array
    ARRAY[1.1],                         -- col_numeric_array
    B'00000001',                        -- col_bit
    B'1',                               -- col_bit_varying
    B'1',                               -- col_varbit
    '<root><min>1</min></root>'::xml    -- col_xml
),
(
    'processing',                       -- status_enum
    50,                                 -- col_smallint
    5000,                               -- col_integer
    5000000000,                         -- col_bigint
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    50.5,                               -- col_real
    50.555555555555555,                 -- col_double_precision
    50.55,                              -- col_numeric
    50.55555,                           -- col_decimal
    50.55,                              -- col_money
    TRUE,                               -- col_boolean
    FALSE,                              -- col_bool
    'Fifty',                            -- col_char
    'Medium values',                    -- col_varchar
    'Testing medium range values for all data types.', -- col_text
    'Med varying',                      -- col_char_varying
    E'\\x4669667479'::bytea,            -- col_bytea (Fifty in hex)
    '2024-07-15',                       -- col_date
    '12:30:00',                         -- col_time
    '12:30:00+05:00',                   -- col_time_tz
    '2024-07-15 12:30:00',              -- col_timestamp
    '2024-07-15 12:30:00+05:00',        -- col_timestamp_tz
    '50 days 12 hours 30 minutes',      -- col_interval
    '50 years',                         -- col_interval_year
    '6 months',                         -- col_interval_month
    '15 days',                          -- col_interval_day
    '12 hours',                         -- col_interval_hour
    '30 minutes',                       -- col_interval_minute
    '30 seconds',                       -- col_interval_second
    '{"medium": 50, "value": "test"}',  -- col_json
    '{"medium": 50, "value": "test", "nested": {"key": "val"}}'::jsonb, -- col_jsonb
    '550e8400-e29b-41d4-a716-446655440050', -- col_uuid
    '172.16.50.1',                      -- col_inet
    '172.16.50.0/24',                   -- col_cidr
    '50:50:50:50:50:50',                -- col_macaddr
    '50:50:50:50:50:50:50:50',          -- col_macaddr8
    '(50,50)',                          -- col_point
    '{1,2,0}',                          -- col_line (line: x + 2y = 0)
    '[(0,0),(50,50)]',                  -- col_lseg
    '((0,0),(50,50))',                  -- col_box
    '[(0,0),(50,50),(25,25)]',         -- col_path
    '((0,0),(50,50),(50,0),(0,0))',    -- col_polygon
    '<(50,50),25>',                     -- col_circle
    '[50,100)',                         -- col_int4range
    '[5000,10000)',                     -- col_int8range
    '[50.0,100.0]',                     -- col_numrange
    '[2024-07-01 00:00:00,2024-07-31 23:59:59)', -- col_tsrange
    '[2024-07-01 00:00:00+00,2024-07-31 23:59:59+00)', -- col_tstzrange
    '[2024-07-01,2024-07-31)',         -- col_daterange
    ARRAY['medium', 'values'],          -- col_text_array
    ARRAY[50, 100],                     -- col_integer_array
    ARRAY[TRUE, FALSE],                 -- col_boolean_array
    ARRAY['550e8400-e29b-41d4-a716-446655440050'::uuid, '550e8400-e29b-41d4-a716-446655440051'::uuid], -- col_uuid_array
    ARRAY['2024-07-15 12:00:00'::timestamp, '2024-07-16 12:00:00'::timestamp], -- col_timestamp_array
    ARRAY[50.5, 100.5],                 -- col_numeric_array
    B'00110011',                        -- col_bit
    B'001100110011',                    -- col_bit_varying
    B'001100110011001100110011',        -- col_varbit
    '<root><medium>50</medium></root>'::xml -- col_xml
),
(
    'shipped',                          -- status_enum
    250,                                -- col_smallint
    25000,                              -- col_integer
    25000000000,                        -- col_bigint
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    250.25,                             -- col_real
    250.252525252525252,                -- col_double_precision
    250.25,                             -- col_numeric
    250.25252,                          -- col_decimal
    250.25,                             -- col_money
    FALSE,                              -- col_boolean
    TRUE,                               -- col_bool
    'Two50',                            -- col_char
    'Quarter values',                   -- col_varchar
    'Testing quarter range values for comprehensive coverage.', -- col_text
    'Qtr varying',                     -- col_char_varying
    E'\\x54776f3530'::bytea,            -- col_bytea (Two50 in hex)
    '2024-10-01',                       -- col_date
    '18:45:00',                         -- col_time
    '18:45:00-05:00',                   -- col_time_tz
    '2024-10-01 18:45:00',              -- col_timestamp
    '2024-10-01 18:45:00-05:00',       -- col_timestamp_tz
    '250 days 18 hours 45 minutes',     -- col_interval
    '250 years',                        -- col_interval_year
    '8 months',                         -- col_interval_month
    '15 days',                          -- col_interval_day
    '18 hours',                         -- col_interval_hour
    '45 minutes',                       -- col_interval_minute
    '45 seconds',                       -- col_interval_second
    '{"quarter": 250, "data": {"nested": true}}', -- col_json
    '{"quarter": 250, "data": {"nested": true}, "list": [1,2,3]}'::jsonb, -- col_jsonb
    '550e8400-e29b-41d4-a716-446655442500', -- col_uuid
    '192.168.250.1',                    -- col_inet
    '192.168.250.0/24',                 -- col_cidr
    '25:25:25:25:25:25',                -- col_macaddr
    '25:25:25:25:25:25:25:25',          -- col_macaddr8
    '(250,250)',                        -- col_point
    '{2,1,0}',                          -- col_line (line: 2x + y = 0)
    '[(0,0),(250,250)]',                -- col_lseg
    '((0,0),(250,250))',                -- col_box
    '[(0,0),(250,250),(125,125)]',      -- col_path
    '((0,0),(250,250),(250,0),(0,0))',  -- col_polygon
    '<(250,250),125>',                  -- col_circle
    '[250,500)',                        -- col_int4range
    '[25000,50000)',                    -- col_int8range
    '[250.0,500.0]',                    -- col_numrange
    '[2024-10-01 00:00:00,2024-10-31 23:59:59)', -- col_tsrange
    '[2024-10-01 00:00:00+00,2024-10-31 23:59:59+00)', -- col_tstzrange
    '[2024-10-01,2024-10-31)',         -- col_daterange
    ARRAY['quarter', 'test', 'data'],   -- col_text_array
    ARRAY[250, 500, 750],               -- col_integer_array
    ARRAY[FALSE, TRUE, FALSE],          -- col_boolean_array
    ARRAY['550e8400-e29b-41d4-a716-446655442500'::uuid, '550e8400-e29b-41d4-a716-446655442501'::uuid, '550e8400-e29b-41d4-a716-446655442502'::uuid], -- col_uuid_array
    ARRAY['2024-10-01 00:00:00'::timestamp, '2024-10-15 12:00:00'::timestamp, '2024-10-31 23:59:59'::timestamp], -- col_timestamp_array
    ARRAY[250.25, 500.50, 750.75],     -- col_numeric_array
    B'11110000',                        -- col_bit
    B'111100001111',                    -- col_bit_varying
    B'111100001111000011110000',        -- col_varbit
    '<root><quarter>250</quarter><nested><value>test</value></nested></root>'::xml -- col_xml
),
(
    'delivered',                        -- status_enum
    500,                                -- col_smallint
    50000,                              -- col_integer
    50000000000,                        -- col_bigint
    DEFAULT,                            -- col_smallserial
    DEFAULT,                            -- col_serial
    DEFAULT,                            -- col_bigserial
    500.5,                              -- col_real
    500.555555555555555,                -- col_double_precision
    500.50,                             -- col_numeric
    500.55555,                          -- col_decimal
    500.50,                             -- col_money
    TRUE,                               -- col_boolean
    TRUE,                               -- col_bool
    'Five00',                           -- col_char
    'Half values',                      -- col_varchar
    'Testing half range values to ensure comprehensive data type coverage.', -- col_text
    'Half varying',                    -- col_char_varying
    E'\\x466976653030'::bytea,          -- col_bytea (Five00 in hex)
    '2024-12-31',                       -- col_date
    '23:59:59',                         -- col_time
    '23:59:59+00:00',                   -- col_time_tz
    '2024-12-31 23:59:59',              -- col_timestamp
    '2024-12-31 23:59:59+00:00',       -- col_timestamp_tz
    '500 days 23 hours 59 minutes',    -- col_interval
    '500 years',                        -- col_interval_year
    '11 months',                        -- col_interval_month
    '30 days',                          -- col_interval_day
    '23 hours',                         -- col_interval_hour
    '59 minutes',                       -- col_interval_minute
    '59 seconds',                       -- col_interval_second
    '{"half": 500, "complex": {"nested": {"deep": true}}}', -- col_json
    '{"half": 500, "complex": {"nested": {"deep": true}}, "array": [1,2,3,4,5]}'::jsonb, -- col_jsonb
    '550e8400-e29b-41d4-a716-446655445000', -- col_uuid
    '10.0.50.1',                        -- col_inet
    '10.0.50.0/24',                     -- col_cidr
    '50:50:50:50:50:50',                -- col_macaddr
    '50:50:50:50:50:50:50:50',          -- col_macaddr8
    '(500,500)',                        -- col_point
    '{3,2,0}',                          -- col_line (line: 3x + 2y = 0)
    '[(0,0),(500,500)]',                -- col_lseg
    '((0,0),(500,500))',                -- col_box
    '[(0,0),(500,500),(250,250)]',      -- col_path
    '((0,0),(500,500),(500,0),(0,0))',  -- col_polygon
    '<(500,500),250>',                  -- col_circle
    '[500,1000)',                       -- col_int4range
    '[50000,100000)',                   -- col_int8range
    '[500.0,1000.0]',                   -- col_numrange
    '[2024-12-01 00:00:00,2024-12-31 23:59:59)', -- col_tsrange
    '[2024-12-01 00:00:00+00,2024-12-31 23:59:59+00)', -- col_tstzrange
    '[2024-12-01,2024-12-31)',         -- col_daterange
    ARRAY['half', 'values', 'test'],    -- col_text_array
    ARRAY[500, 1000, 1500],             -- col_integer_array
    ARRAY[TRUE, TRUE, TRUE],            -- col_boolean_array
    ARRAY['550e8400-e29b-41d4-a716-446655445000'::uuid, '550e8400-e29b-41d4-a716-446655445001'::uuid, '550e8400-e29b-41d4-a716-446655445002'::uuid, '550e8400-e29b-41d4-a716-446655445003'::uuid], -- col_uuid_array
    ARRAY['2024-12-01 00:00:00'::timestamp, '2024-12-15 12:00:00'::timestamp, '2024-12-31 23:59:59'::timestamp], -- col_timestamp_array
    ARRAY[500.5, 1000.5, 1500.5],       -- col_numeric_array
    B'11111111',                        -- col_bit
    B'1111111111111111',                -- col_bit_varying
    B'111111111111111111111111',        -- col_varbit
    '<root><half>500</half><complex><nested><deep>true</deep></nested></complex></root>'::xml -- col_xml
);

-- Verify data insertion
SELECT 'Database initialized successfully!' as status;
SELECT COUNT(*) as user_count FROM users;
SELECT COUNT(*) as product_count FROM products;
SELECT COUNT(*) as order_count FROM orders;
SELECT COUNT(*) as order_item_count FROM order_items;
SELECT COUNT(*) as postgres_types_test_count FROM postgres_types_test;

-- ============================================================================
-- Schema 1: E-commerce Schema
-- ============================================================================
CREATE SCHEMA IF NOT EXISTS schema1;

-- Create tables in schema1
CREATE TABLE IF NOT EXISTS schema1.customers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(50),
    address TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS schema1.products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    stock INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS schema1.orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    status VARCHAR(50) DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES schema1.customers(id) ON DELETE CASCADE
);

-- Insert data into schema1
INSERT INTO schema1.customers (id, name, email, phone, address) VALUES
('c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'John Doe', 'john.doe@example.com', '555-0101', '123 Main St, City, State'),
('c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'Jane Smith', 'jane.smith@example.com', '555-0102', '456 Oak Ave, City, State'),
('c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'Bob Johnson', 'bob.johnson@example.com', '555-0103', '789 Pine Rd, City, State')
ON CONFLICT (id) DO NOTHING;

INSERT INTO schema1.products (id, name, description, price, stock) VALUES
('a1eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'Laptop', 'High-performance laptop', 1299.99, 25),
('a1eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'Mouse', 'Wireless mouse', 29.99, 100),
('a1eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'Keyboard', 'Mechanical keyboard', 89.99, 50),
('a1eebc99-9c0b-4ef8-bb6d-6bb9bd380a04', 'Monitor', '27-inch 4K monitor', 399.99, 30),
('a1eebc99-9c0b-4ef8-bb6d-6bb9bd380a05', 'Headphones', 'Noise-cancelling headphones', 199.99, 40)
ON CONFLICT (id) DO NOTHING;

INSERT INTO schema1.orders (id, customer_id, total, status) VALUES
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 1299.99, 'completed'),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 119.98, 'processing'),
('b1eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'c1eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 399.99, 'pending')
ON CONFLICT (id) DO NOTHING;

-- ============================================================================
-- Schema 2: Inventory Management Schema
-- ============================================================================
CREATE SCHEMA IF NOT EXISTS schema2;

-- Create tables in schema2
CREATE TABLE IF NOT EXISTS schema2.warehouses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    location VARCHAR(255) NOT NULL,
    capacity INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS schema2.items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sku VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    unit_price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS schema2.inventory (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    warehouse_id UUID NOT NULL,
    item_id UUID NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (warehouse_id) REFERENCES schema2.warehouses(id) ON DELETE CASCADE,
    FOREIGN KEY (item_id) REFERENCES schema2.items(id) ON DELETE CASCADE,
    UNIQUE(warehouse_id, item_id)
);

CREATE TABLE IF NOT EXISTS schema2.transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    warehouse_id UUID NOT NULL,
    item_id UUID NOT NULL,
    transaction_type VARCHAR(50) NOT NULL, -- 'in' or 'out'
    quantity INTEGER NOT NULL,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (warehouse_id) REFERENCES schema2.warehouses(id) ON DELETE CASCADE,
    FOREIGN KEY (item_id) REFERENCES schema2.items(id) ON DELETE CASCADE
);

-- Insert data into schema2
INSERT INTO schema2.warehouses (id, name, location, capacity) VALUES
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'Main Warehouse', 'New York, NY', 10000),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'West Coast Distribution', 'Los Angeles, CA', 8000),
('d2eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'East Coast Distribution', 'Boston, MA', 6000)
ON CONFLICT (id) DO NOTHING;

INSERT INTO schema2.items (id, sku, name, description, unit_price) VALUES
('e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'SKU-001', 'Widget A', 'Standard widget', 10.50),
('e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'SKU-002', 'Widget B', 'Premium widget', 25.75),
('e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'SKU-003', 'Gadget X', 'Electronic gadget', 99.99),
('e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a04', 'SKU-004', 'Gadget Y', 'Advanced gadget', 149.99),
('e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a05', 'SKU-005', 'Tool Z', 'Professional tool', 79.50)
ON CONFLICT (id) DO NOTHING;

INSERT INTO schema2.inventory (id, warehouse_id, item_id, quantity) VALUES
('f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 500),
('f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 300),
('f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 150),
('f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a04', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a04', 100),
('f2eebc99-9c0b-4ef8-bb6d-6bb9bd380a05', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a05', 200)
ON CONFLICT (id) DO NOTHING;

INSERT INTO schema2.transactions (id, warehouse_id, item_id, transaction_type, quantity, notes) VALUES
('12eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'in', 100, 'Initial stock'),
('12eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a01', 'out', 50, 'Order fulfillment'),
('12eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a02', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'in', 150, 'New shipment'),
('12eebc99-9c0b-4ef8-bb6d-6bb9bd380a04', 'd2eebc99-9c0b-4ef8-bb6d-6bb9bd380a03', 'e2eebc99-9c0b-4ef8-bb6d-6bb9bd380a05', 'in', 200, 'Restock')
ON CONFLICT (id) DO NOTHING;

-- Verify schema data insertion
SELECT 'Schema1 initialized successfully!' as status;
SELECT COUNT(*) as schema1_customers FROM schema1.customers;
SELECT COUNT(*) as schema1_products FROM schema1.products;
SELECT COUNT(*) as schema1_orders FROM schema1.orders;

SELECT 'Schema2 initialized successfully!' as status;
SELECT COUNT(*) as schema2_warehouses FROM schema2.warehouses;
SELECT COUNT(*) as schema2_items FROM schema2.items;
SELECT COUNT(*) as schema2_inventory FROM schema2.inventory;
SELECT COUNT(*) as schema2_transactions FROM schema2.transactions;

