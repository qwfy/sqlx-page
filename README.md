# SQLx-Page

This library provides cursor based pagination for SQLx.

It is build around `sqlx::QueryBuilder`, so it has the same security level as `sqlx::QueryBuilder`.

Currently only Postgres is supported. Without too much work, other databases can be added.

See the `examples` directory for an example.