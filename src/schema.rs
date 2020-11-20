table! {
    contest_register_lists (contest_region, user_id) {
        contest_region -> Text,
        user_id -> Int4,
        is_unrated -> Bool,
    }
}

table! {
    contests (region) {
        region -> Text,
        name -> Text,
        state -> Text,
        start_time -> Timestamp,
        end_time -> Timestamp,
        seal_before_end -> Nullable<Int4>,
        register_end_time -> Timestamp,
        final_rank -> Nullable<Text>,
    }
}

table! {
    problems (region, id) {
        id -> Int4,
        region -> Text,
        title -> Text,
        description -> Nullable<Text>,
        input_explain -> Nullable<Text>,
        output_explain -> Nullable<Text>,
        input_examples -> Array<Text>,
        output_examples -> Array<Text>,
        hint -> Nullable<Text>,
        tags -> Array<Text>,
        sources -> Array<Text>,
        difficulty -> Text,
        submit_times -> Int4,
        accept_times -> Int4,
        default_max_cpu_time -> Int4,
        default_max_memory -> Int4,
        test_case -> Nullable<Text>,
        max_score -> Int4,
        opaque_output -> Bool,
    }
}

table! {
    regions (name) {
        name -> Text,
        need_pass -> Bool,
        salt -> Nullable<Varchar>,
        hash -> Nullable<Bytea>,
        self_type -> Text,
        judge_type -> Nullable<Text>,
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
        score -> Nullable<Float8>,
        setting_data -> Text,
        result_data -> Nullable<Text>,
        err_reason -> Nullable<Text>,
        submit_time -> Timestamp,
        start_pend_time -> Nullable<Timestamp>,
        finish_time -> Nullable<Timestamp>,
        language -> Text,
    }
}

table! {
    test_cases (name) {
        name -> Text,
        is_spj -> Bool,
        count -> Int4,
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

joinable!(contest_register_lists -> contests (contest_region));
joinable!(contest_register_lists -> users (user_id));
joinable!(problems -> regions (region));
joinable!(problems -> test_cases (test_case));
joinable!(status -> regions (problem_region));
joinable!(status -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    contest_register_lists,
    contests,
    problems,
    regions,
    status,
    test_cases,
    users,
);
