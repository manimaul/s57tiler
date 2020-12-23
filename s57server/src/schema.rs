table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    styles (id) {
        id -> Int8,
        name -> Text,
        style -> Jsonb,
    }
}

allow_tables_to_appear_in_same_query!(
    spatial_ref_sys,
    styles,
);
