-- Your SQL goes here
CREATE TABLE IF NOT EXISTS car_brands
(
    `id`         INT(11) PRIMARY KEY AUTO_INCREMENT,
    `name`       VARCHAR(60),
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY unique_name (`name`)
);

INSERT INTO car_brands (`name`)
VALUES ('Koenigsegg'),
       ('Bugatti'),
       ('Ferrari'),
       ('Porsche'),
       ('Lamborghini'),
       ('McLaren'),
       ('BMW'),
       ('Audi'),
       ('Mercedes-Benz'),
       ('Tesla'),
       ('Dodge'),
       ('Lexus'),
       ('Toyota'),
       ('Nissan');