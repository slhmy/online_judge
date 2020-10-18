table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Nullable<Varchar>,
        mobile -> Nullable<Varchar>,
        job_number -> Nullable<Varchar>,
        role -> Varchar,
        salt -> Varchar,
        register_time -> Timestamp,
        hash -> Bytea,
    }
}
