// @generated automatically by Diesel CLI.

diesel::table! {
    contest_participates (id) {
        id -> Int4,
        user_id -> Int4,
        contest_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    contests (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        start_at -> Timestamp,
        end_at -> Timestamp,
        penalty -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        author_id -> Int4,
        contest_id -> Int4,
        name -> Varchar,
        statement -> Varchar,
        constraints -> Varchar,
        input -> Varchar,
        output -> Varchar,
        score -> Int4,
        time_limit -> Int4,
        memory_limit -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        encrypted_password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(contest_participates -> contests (contest_id));
diesel::joinable!(contest_participates -> users (user_id));
diesel::joinable!(tasks -> contests (contest_id));
diesel::joinable!(tasks -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    contest_participates,
    contests,
    tasks,
    users,
);
