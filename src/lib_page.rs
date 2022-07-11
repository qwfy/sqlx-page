use sqlx;

macro_rules! push_where {
    ($self:expr, $builder:expr, $($cursor:expr),+) => {{
        $builder.push(" ");
        $builder.push("(");

        // (col1, col2, ...)
        let mut sep = $builder.separated(", ");
        for col in &$self.columns {
            sep.push(col);
        }

        // operator, < or >
        $builder.push(" ");
        $builder.push(&$self.compare);
        $builder.push(" ");

        // ($_, $_, ...)
        let mut sep = $builder.separated(", ");
        $(
            sep.push_bind($cursor);
        )+

        $builder.push(")");
    }}
}


/// The struct used to perform pagination
pub struct Page {
    columns: Vec<String>,
    compare: &'static str,
    sort: &'static str,
    size: u32,
}

impl Page {
    pub fn new(smaller: bool, size: u32, columns: Vec<String>) -> Self {
        // When smaller: 7 6 5 4 3 => order desc, col < 7
        // When larger : 3 4 5 6 7 => order asc,  col > 3
        let compare = if smaller { "<" } else { ">" };
        let sort = if smaller { "desc" } else { "asc" };

        Page {
            columns,
            compare,
            sort,
            size,
        }
    }


    pub fn push_where1<'args, T1>(&self, builder: &mut sqlx::QueryBuilder<'args, sqlx::Postgres>, cursors: Option<T1>)
        where
            T1: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send
    {
        match cursors {
            None => {
                builder.push(" ");
                builder.push("true");
            }
            Some(t1) => {
                push_where!(self, builder, t1);
            }
        }
    }

    pub fn push_where2<'args, T1, T2>(&self, builder: &mut sqlx::QueryBuilder<'args, sqlx::Postgres>, cursors: Option<(T1, T2)>)
        where
            T1: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T2: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
    {
        match cursors {
            None => {
                builder.push(" ");
                builder.push("true");
            }
            Some((t1, t2)) => {
                push_where!(self, builder, t1, t2);
            }
        }
    }

    pub fn push_where3<'args, T1, T2, T3>(&self, builder: &mut sqlx::QueryBuilder<'args, sqlx::Postgres>, cursors: Option<(T1, T2, T3)>)
        where
            T1: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T2: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T3: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
    {
        match cursors {
            None => {
                builder.push(" ");
                builder.push("true");
            }
            Some((t1, t2, t3)) => {
                push_where!(self, builder, t1, t2, t3);
            }
        }
    }

    pub fn push_where4<'args, T1, T2, T3, T4>(&self, builder: &mut sqlx::QueryBuilder<'args, sqlx::Postgres>, cursors: Option<(T1, T2, T3, T4)>)
        where
            T1: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T2: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T3: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T4: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
    {
        match cursors {
            None => {
                builder.push(" ");
                builder.push("true");
            }
            Some((t1, t2, t3, t4)) => {
                push_where!(self, builder, t1, t2, t3, t4);
            }
        }
    }

    pub fn push_where5<'args, T1, T2, T3, T4, T5>(&self, builder: &mut sqlx::QueryBuilder<'args, sqlx::Postgres>, cursors: Option<(T1, T2, T3, T4, T5)>)
        where
            T1: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T2: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T3: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T4: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
            T5: 'args + sqlx::Encode<'args, sqlx::Postgres> + sqlx::Type <sqlx::Postgres> + Send,
    {
        match cursors {
            None => {
                builder.push(" ");
                builder.push("true");
            }
            Some((t1, t2, t3, t4, t5)) => {
                push_where!(self, builder, t1, t2, t3, t4, t5);
            }
        }
    }

    pub fn push_order_by(&self, builder: &mut sqlx::QueryBuilder <sqlx::Postgres>) {
        builder.push(" ");
        builder.push("order by");

        // col1 desc, col2 desc, ...
        builder.push(" ");
        let mut sep = builder.separated(", ");
        for col in &self.columns {
            sep.push(col);
            sep.push_unseparated(" ");
            sep.push_unseparated(&self.sort);
        }
    }

    pub fn push_limit(&self, builder: &mut sqlx::QueryBuilder <sqlx::Postgres>) {
        builder.push(" ");
        builder.push("limit");

        builder.push(" ");
        builder.push_bind(self.size);
    }
}