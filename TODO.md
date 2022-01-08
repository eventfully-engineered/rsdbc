# TODOs

1. Ditch driver logic and move more towards a connection factory
2. work on errors
3. logger including a sql logger and a slow query logger
4. sqlite transaction definition options
5. impl Drop to release resources???
6. Add support for Tracing (https://github.com/tokio-rs/tracing)
7. connection pooling
   - look at https://docs.rs/mobc/latest/mobc/
   - https://github.com/r2dbc/r2dbc-pool


It probably doesnt belong here but adding as a reminder to look at the following for inspiration around schema/migrations/queries and generated code https://docs.sqlc.dev/en/stable/index.html. I think we could build something on top of this which is more raw data access along with southbound migrations

Also look at a simple orm like the following that can be built on top
- https://github.com/DapperLib/Dapper
- https://docs.spring.io/spring-framework/docs/current/javadoc-api/org/springframework/jdbc/core/JdbcTemplate.html
- https://jdbi.org/


not sure if this should go here or doatavious but want convenience methods for
- one: one() returns the only row in the result set. If zero or multiple rows are encountered, it will throw IllegalStateException.
- find_one: returns an Optional<T> of the only row in the result set, or Optional.empty() if no rows are returned.
- first: returns the first row in the result set. If zero rows are encountered, IllegalStateException is thrown.
- find_first: returns an Optional<T> of the first row, if any.

row mapper
