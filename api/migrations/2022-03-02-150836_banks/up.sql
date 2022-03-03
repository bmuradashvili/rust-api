-- Your SQL goes here
CREATE TABLE IF NOT EXISTS banks
(
    `id`         INT(11) PRIMARY KEY AUTO_INCREMENT,
    `name`       VARCHAR(60),
    `country`    VARCHAR(60),
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY unique_name (`name`)
);

INSERT INTO banks (`name`, `country`)
VALUES ('TBC', 'Georgia'),
       ('Bank of Georgia', 'Georgia'),
       ('Liberty Bank', 'Georgia');