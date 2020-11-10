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
        test_case -> Nullable<Text>,
        max_score -> Int4,
    }
}

table! {
    status (id) {
        id -> Uuid,
        owner_id -> Int4,
        problem_id -> Int4,
        problem_region -> Text,
        state -> Text,
        judge_type -> Text,
        result -> Nullable<Text>,
        score -> Nullable<Int4>,
        data -> Nullable<Text>,
    }
}

table! {
    test_cases (name) {
        name -> Text,
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

joinable!(problems -> test_cases (test_case));
joinable!(status -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    problems,
    status,
    test_cases,
    users,
);
