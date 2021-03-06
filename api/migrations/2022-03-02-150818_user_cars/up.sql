-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_cars
(
    `id`           INT(11) PRIMARY KEY AUTO_INCREMENT,
    `user_id`      INT(11),
    `car_model_id` INT(11),
    `year`         INT(4),
    `created_at`   DATETIME DEFAULT CURRENT_TIMESTAMP,
    `updated_at`   DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (`user_id`)
        REFERENCES users (`id`)
        ON UPDATE CASCADE ON DELETE RESTRICT,

    FOREIGN KEY (`car_model_id`)
        REFERENCES car_models (`id`)
        ON UPDATE CASCADE ON DELETE RESTRICT
);