# user

## DDL for `user`

```sql
CREATE TABLE users (
    id VARCHAR(255) PRIMARY KEY DEFAULT uuid_generate_v4 (),
    first_name VARCHAR(60) NOT NULL,
    last_name VARCHAR(60) NOT NULL,
    date_of_birth Date NOT NULL
);

INSERT INTO users (id, first_name, last_name, date_of_birth) VALUES ('1', '1', '1', '2022-01-01');
INSERT INTO users (id, first_name, last_name, date_of_birth) VALUES ('2', '2', '2', '2022-11-25');
INSERT INTO users (id, first_name, last_name, date_of_birth) VALUES ('3', '3', '3', '2022-11-26');
```

## References

- <https://dev.to/steadylearner/how-to-use-grpc-with-rust-tonic-and-postgres-database-with-examples-3dl7>
