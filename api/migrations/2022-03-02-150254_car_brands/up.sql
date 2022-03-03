-- Your SQL goes here
CREATE TABLE IF NOT EXISTS car_brands
(
    `id`         INT(11) PRIMARY KEY AUTO_INCREMENT,
    `name`       VARCHAR(60),
    `created_at` DATETIME,
    `updated_at` DATETIME,

    INDEX (`name`)
)