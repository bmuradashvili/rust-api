-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bank_cards
(
    `id`            INT(11) PRIMARY KEY AUTO_INCREMENT,
    `bank_id`       INT(11),
    `card_type_id`  INT(11),
    `card_brand_id` INT(11),
    `created_at`    DATETIME DEFAULT CURRENT_TIMESTAMP,
    `updated_at`    DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    UNIQUE KEY unique_name (`bank_id`, `card_type_id`, `card_brand_id`),

    FOREIGN KEY (`bank_id`)
        REFERENCES banks (`id`)
        ON UPDATE CASCADE ON DELETE RESTRICT,

    FOREIGN KEY (`card_type_id`)
        REFERENCES bank_card_types (`id`)
        ON UPDATE CASCADE ON DELETE RESTRICT,

    FOREIGN KEY (`card_brand_id`)
        REFERENCES bank_card_brands (`id`)
        ON UPDATE CASCADE ON DELETE RESTRICT
);