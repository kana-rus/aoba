pub enum DBTypes {
    // -> String
    CHAR(usize),
    VARCHAR(usize),
    TEXT,
    UUID,

    BOOLEAN,// -> bool

    SMALLINT,// -> i16
    INT,// -> i32
    SERIAL,// -> i32
    
    FLOAT(usize),// -> f64
    REAL,// -> f32
    // NUMERIC(usize, usize),// ->

    DATE,// -> self::times::Date
    TIME,// -> self::times::Time
    TIMESTAMP,// -> self::times::TimeStamp
    TIMESTAMPZ,// -> self::times::TimeStampZ
    INTERVAL,// -> self::times::Interval

    // -> impl JSON
    JSON,
    JSONB,
}

fn from_row<'r, T: sqlx::FromRow<'r, sqlx::postgres::PgRow>>() {}
fn test() {
    // from_row::<(u8,)>();
    // from_row::<(u16,)>();
    // from_row::<(u32,)>();
    // from_row::<(u64,)>();
    // from_row::<(usize,)>();
    from_row::<(i8,)>();
    from_row::<(i16,)>();
    from_row::<(i32,)>();
    from_row::<(i64,)>();
    // from_row::<(isize,)>();
    from_row::<(String,)>();
    from_row::<(bool,)>();
    // from_row::<f32>();
    // from_row::<f64>();
}
