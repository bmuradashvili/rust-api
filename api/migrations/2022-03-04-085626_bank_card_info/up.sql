-- Your SQL goes here
CREATE VIEW bank_card_info AS
SELECT ubc.id           AS id,
       ubc.user_id      AS user_id,
       b.name           AS bank,
       bct.card_type    AS card_type,
       bcb.name         AS card_company,
       ubc.name_on_card AS name_on_card,
       ubc.expiration   AS expiration,
       ubc.created_at   AS created_at,
       ubc.updated_at   AS updated_at
FROM user_bank_cards ubc
         Left JOIN bank_cards bc on ubc.bank_card_id = bc.id
         LEFT JOIN bank_card_brands bcb on bcb.id = bc.card_brand_id
         LEFT JOIN bank_card_types bct on bc.card_type_id = bct.id
         LEFT JOIN banks b on bc.bank_id = b.id;