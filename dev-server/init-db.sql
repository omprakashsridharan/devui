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

-- Verify data insertion
SELECT 'Database initialized successfully!' as status;
SELECT COUNT(*) as user_count FROM users;
SELECT COUNT(*) as product_count FROM products;
SELECT COUNT(*) as order_count FROM orders;
SELECT COUNT(*) as order_item_count FROM order_items;

