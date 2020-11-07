table! {
    problems (region, id) {
        id -> Int4,
        region -> Text,
        title -> Text,
        description -> Nullable<Text>,
        input_explain -> Nullable<Text>,
        output_explain -> Nullable<Text>,
        input_examples -> Nullable<Array<Text>>,
        output_examples -> Nullable<Array<Text>>,
        hint -> Nullable<Text>,
        tags -> Nullable<Array<Text>>,
        sources -> Nullable<Array<Text>>,
        difficulty -> Text,
        submit_times -> Int4,
        accept_times -> Int4,
        default_max_cpu_time -> Int4,
        default_max_memory -> Int4,
        is_spj -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Nullable<Text>,
        mobile -> Nullable<Text>,
        job_number -> Nullable<Text>,
        role -> Text,
        salt -> Varchar,
        register_time -> Timestamp,
        hash -> Bytea,
        school -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    problems,
    users,
);
