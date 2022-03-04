table! {
    bank_card_brands (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Varchar>,
    }
}

table! {
    bank_card_types (id) {
        id -> Nullable<Integer>,
        card_type -> Nullable<Varchar>,
    }
}

table! {
    bank_cards (id) {
        id -> Nullable<Integer>,
        bank_id -> Nullable<Integer>,
        card_type_id -> Nullable<Integer>,
        card_brand_id -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    banks (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    car_brands (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    car_models (id) {
        id -> Nullable<Integer>,
        brand_id -> Nullable<Integer>,
        transmission_type_id -> Nullable<Integer>,
        model_name -> Nullable<Varchar>,
        engine_cylinders -> Nullable<Integer>,
        engine_displacement -> Nullable<Decimal>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    car_transmission_types (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Varchar>,
    }
}

table! {
    user_bank_cards (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        bank_card_id -> Nullable<Integer>,
        name_on_card -> Nullable<Varchar>,
        expiration -> Nullable<Date>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    user_cars (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        car_model_id -> Nullable<Integer>,
        year -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        birth_date -> Nullable<Date>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    car_info (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        brand -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        transmission_type -> Nullable<Varchar>,
        engine_cylinders -> Nullable<Integer>,
        engine_displacement -> Nullable<Decimal>,
        year -> Nullable<Integer>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    bank_card_info (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        bank -> Nullable<Varchar>,
        card_type -> Nullable<Varchar>,
        card_company -> Nullable<Varchar>,
        name_on_card -> Nullable<Varchar>,
        expiration -> Nullable<Date>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

joinable!(bank_cards -> bank_card_brands (card_brand_id));
joinable!(bank_cards -> bank_card_types (card_type_id));
joinable!(bank_cards -> banks (bank_id));
joinable!(car_models -> car_brands (brand_id));
joinable!(car_models -> car_transmission_types (transmission_type_id));
joinable!(user_bank_cards -> bank_cards (bank_card_id));
joinable!(user_bank_cards -> users (user_id));
joinable!(user_cars -> car_models (car_model_id));
joinable!(user_cars -> users (user_id));

allow_tables_to_appear_in_same_query!(
    bank_card_brands,
    bank_card_types,
    bank_cards,
    banks,
    car_brands,
    car_models,
    car_transmission_types,
    user_bank_cards,
    user_cars,
    users,
);
