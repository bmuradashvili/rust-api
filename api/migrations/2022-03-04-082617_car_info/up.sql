-- Your SQL goes here
CREATE VIEW car_info AS
SELECT uc.id                  AS id,
       uc.user_id             AS user_id,
       cb.name                AS brand,
       cm.model_name          AS model,
       ctt.name               AS transmission_type,
       cm.engine_cylinders    AS engine_cylinders,
       cm.engine_displacement AS engine_displacement,
       uc.year                AS `year`,
       uc.created_at          AS created_at,
       uc.updated_at          AS updated_at
FROM user_cars uc
         LEFT JOIN car_models cm on uc.car_model_id = cm.id
         LEFT JOIN car_brands cb on cm.brand_id = cb.id
         LEFT JOIN car_transmission_types ctt on cm.transmission_type_id = ctt.id;