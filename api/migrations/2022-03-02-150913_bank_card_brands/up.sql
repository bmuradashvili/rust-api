-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bank_card_brands
(
    `id`   INT(11) PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(60),

    UNIQUE KEY unique_brand_name (`name`)
);

INSERT INTO bank_card_brands (`name`)
VALUES ('Visa'),
       ('Master Card'),
       ('American Express'),
       ('Discover'),
       ('Paypal');