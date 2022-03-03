-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bank_card_brands
(
    `id`   INT(11) PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(60),

    INDEX (name)
)