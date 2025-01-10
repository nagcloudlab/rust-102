
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



-- transaction_history table schema
CREATE TABLE transaction_history (
    id SERIAL PRIMARY KEY,
    from_account VARCHAR(50),
    to_account VARCHAR(50),
    amount real,
    timestamp TIMESTAMP DEFAULT NOW()
);


ALTER TABLE transaction_history
ALTER COLUMN from_account SET NOT NULL,
ALTER COLUMN to_account SET NOT NULL,
ALTER COLUMN amount SET NOT NULL,
ALTER COLUMN timestamp SET NOT NULL;




