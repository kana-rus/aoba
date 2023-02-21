pub enum Mixin {
    id,
    times,
}

pub enum DBType {
    // -> String
    VARCHAR(usize),
    TEXT,

    BOOL,// -> bool

    SMALLINT,// -> i16
    INT,// -> i32
    BIGINT,// -> i64
    
    SERIAL,// -> i32
    BIGSERIAL,// -> i64
    
    REAL,// -> f32
    DOUBLE_PRECISION,// -> f64

    DATE,// -> self::times::Date
    TIME,// -> self::times::Time
    TIMESTAMP,// -> self::times::TimeStamp
    // TIMESTAMPZ,// -> self::times::TimeStampZ
    INTERVAL,// -> self::times::Interval

    // -> impl JSON
    JSON,
    JSONB,
}

pub enum ColumnConstrain {
    CHECK(&'static str),
    NOT_NULL,
    UNIQUE,
    PRIMARY_KEY,
    REFERENCES(&'static str),
    DEFAULT(&'static str),
}
pub enum TableConstrain {
    CHECK(&'static str),
    UNIQUE(&'static str),
    PRIMARY_KEY(&'static str),
    FOREIGN_KEY(&'static str), REFERENCES(&'static str),
}
