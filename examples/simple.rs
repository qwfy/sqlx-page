use sqlx::QueryBuilder;
use sqlx_page::Pagination;

fn main() {
    let pagination = Pagination::new(
        // Direction to select rows.
        // If true, will select rows in the "smaller" direction,
        // e.g. if your want to sort by the time,
        // `true` means select rows from the current cursor to the past
        true,
        // Size of the page
        100,
        // Use these columns for sorting.
        // The combination of the columns should uniquely identify a row
        vec![String::from("row_id")]
    );

    // Create a `sqlx::QueryBuilder` as usual
    let mut builder = QueryBuilder::new(r#"
        select row_id, user_name
        from users
        where true and
        "#);

    // ... Maybe other search conditions

    // Push and bind the pagination condition: `(row_id < $_)`.
    // `push_where2..5` are also defined, they are used if your cursor consists of multiple columns
    pagination.push_where1(&mut builder, Some(11));

    // Push the order by clause: `order by row_id desc`
    pagination.push_order_by(&mut builder);

    // Push the limit clause: `limit 100`
    pagination.push_limit(&mut builder);

    // ... Use the builder as usual
}