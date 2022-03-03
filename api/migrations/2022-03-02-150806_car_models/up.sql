-- Your SQL goes here
CREATE TABLE IF NOT EXISTS car_models
(
    `id`                   INT(11) PRIMARY KEY AUTO_INCREMENT,
    `brand_id`             INT(11),
    `transmission_type_id` INT(11),
    `model_name`           VARCHAR(60),
    `engine_cylinders`     INT(4),
    `engine_displacement`  DECIMAL(6, 2),
    `created_at`           DATETIME,
    `updated_at`           DATETIME,

    INDEX (brand_id, transmission_type_id),

    FOREIGN KEY (brand_id)
        REFERENCES car_brands (id)
        ON UPDATE CASCADE ON DELETE RESTRICT,

    FOREIGN KEY (transmission_type_id)
        REFERENCES car_transmission_types (id)
        ON UPDATE CASCADE ON DELETE RESTRICT
)