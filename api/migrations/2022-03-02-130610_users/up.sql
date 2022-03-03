-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users
(
    `id`           INT(11) PRIMARY KEY AUTO_INCREMENT,
    `email`        VARCHAR(100),
    `password`     VARCHAR(60),
    `first_name`   VARCHAR(60),
    `last_name`    VARCHAR(60),
    `phone_number` VARCHAR(60),
    `birth_date`   DATE,
    `created_at`   DATETIME DEFAULT CURRENT_TIMESTAMP,
    `updated_at`   DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY unique_email (`email`)
);