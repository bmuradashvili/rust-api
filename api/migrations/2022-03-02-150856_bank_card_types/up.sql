-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bank_card_types
(
    `id`        INT(11) PRIMARY KEY AUTO_INCREMENT,
    `card_type` VARCHAR(60),

    INDEX (`card_type`)
)