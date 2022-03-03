-- Your SQL goes here
CREATE TABLE IF NOT EXISTS car_transmission_types
(
    `id`   INT(11) PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(60),

    INDEX (`name`)
)