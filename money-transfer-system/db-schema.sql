
-- accounts table schema
CREATE TABLE accounts (
    number VARCHAR(50) PRIMARY KEY, -- Account number (unique identifier)
    balance real NOT NULL CHECK (balance >= 0) -- Account balance (non-negative)
);


-- Insert some sample data
INSERT INTO accounts (number, balance) VALUES
('ACC001', 1000.00),
('ACC002', 500.00),
('ACC003', 750.00);
