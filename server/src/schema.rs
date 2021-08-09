table! {
    activities (id) {
        id -> Int4,
        title -> Text,
        activity_type -> Text,
        date -> Timestamp,
        time -> Interval,
        distance -> Float8,
        elevation -> Float8,
    }
}

table! {
    valid_activity_types (activity_type) {
        activity_type -> Text,
    }
}

joinable!(activities -> valid_activity_types (activity_type));

allow_tables_to_appear_in_same_query!(activities, valid_activity_types,);
