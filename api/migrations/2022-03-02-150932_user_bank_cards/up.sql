-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_bank_cards
(
    `id`           INT(11) PRIMARY KEY AUTO_INCREMENT,
    `user_id`      INT(11),
    `bank_card_id` INT(11),
    `name_on_card` VARCHAR(60),
    `expiration`   DATE,
    `created_at`   DATETIME DEFAULT CURRENT_TIMESTAMP,
    `updated_at`   DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (`user_id`)
        REFERENCES users (id)
        ON UPDATE CASCADE ON DELETE RESTRICT,

    FOREIGN KEY (`bank_card_id`)
        REFERENCES bank_cards (id)
        ON UPDATE CASCADE ON DELETE RESTRICT
);