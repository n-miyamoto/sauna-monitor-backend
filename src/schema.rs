// @generated automatically by Diesel CLI.

diesel::table! {
    sensor_data (id) {
        id -> Int4,
        time_stamp -> Timestamp,
        data0 -> Nullable<Float4>,
        data1 -> Nullable<Float4>,
        data2 -> Nullable<Float4>,
    }
}
