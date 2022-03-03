-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bank_card_types
(
    `id`        INT(11) PRIMARY KEY AUTO_INCREMENT,
    `card_type` VARCHAR(60),

    UNIQUE KEY unique_card_type (`card_type`)
);

INSERT INTO bank_card_types (`card_type`)
VALUES ('Credit'),
       ('Debit'),
       ('Virtual'),
       ('Prepaid');