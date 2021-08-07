table! {
    activities (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
        date -> Timestamp,
        time -> Interval,
        distance -> Numeric,
        elevation -> Nullable<Numeric>,
        title -> Nullable<Text>,
    }
}

table! {
    valid_activity_types (type_) {
        #[sql_name = "type"]
        type_ -> Text,
    }
}

joinable!(activities -> valid_activity_types (type));

allow_tables_to_appear_in_same_query!(
    activities,
    valid_activity_types,
);
