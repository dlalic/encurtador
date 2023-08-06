// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Uuid,
        url -> Text,
        shortened -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    visits (url_id) {
        url_id -> Uuid,
        count -> Int8,
    }
}

diesel::joinable!(visits -> urls (url_id));

diesel::allow_tables_to_appear_in_same_query!(urls, visits,);
