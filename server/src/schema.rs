table! {
    activities (id) {
        id -> Int4,
        title -> Text,
        user_id -> Int4,
        activity_type_id -> Int4,
        date -> Timestamp,
        time -> Interval,
        distance -> Float8,
        elevation -> Float8,
    }
}

table! {
    activity_type (id) {
        id -> Int4,
        name -> Text,
        scale -> Float8,
        elevation_scale -> Float8,
    }
}

table! {
    users (id) {
        id -> Int4,
        user_name -> Text,
    }
}

joinable!(activities -> activity_type (activity_type_id));
joinable!(activities -> users (user_id));

allow_tables_to_appear_in_same_query!(
    activities,
    activity_type,
    users,
);
