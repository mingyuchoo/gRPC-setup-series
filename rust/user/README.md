# user

## DDL for `user`

```sql
-- Enable the uuid-ossp extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name VARCHAR(60) NOT NULL,
    last_name VARCHAR(60) NOT NULL,
    date_of_birth DATE NOT NULL
);

-- Insert data into the users table
INSERT INTO users (first_name, last_name, date_of_birth) VALUES ('Carly', 'Hickman', '2025-01-01');
INSERT INTO users (first_name, last_name, date_of_birth) VALUES ('Jakobe', 'Savage', '2025-01-25');
INSERT INTO users (first_name, last_name, date_of_birth) VALUES ('Louise', 'Spence', '2025-01-26');
```

## References

- <https://dev.to/steadylearner/how-to-use-grpc-with-rust-tonic-and-postgres-database-with-examples-3dl7>
