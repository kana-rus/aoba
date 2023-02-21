pub enum DBTypes {
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
    DOUBLEPRECISION,// -> f64

    DATE,// -> self::times::Date
    TIME,// -> self::times::Time
    TIMESTAMP,// -> self::times::TimeStamp
    TIMESTAMPZ,// -> self::times::TimeStampZ
    INTERVAL,// -> self::times::Interval

    // -> impl JSON
    JSON,
    JSONB,
}
