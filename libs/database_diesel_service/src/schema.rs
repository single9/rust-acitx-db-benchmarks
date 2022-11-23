diesel::table! {
    todo (id) {
        id -> Uuid,
        title -> Text,
        checked -> Bool,
        create_time -> Timestamptz,
        modify_time -> Timestamptz,
    }
}
