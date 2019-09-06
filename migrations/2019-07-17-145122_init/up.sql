CREATE TABLE `dish` (
    `dish_id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `dish_link` varchar(191),
    PRIMARY KEY (`dish_id`),
    UNIQUE KEY  `dish_id` (`dish_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `language` (
    `language_name` varchar(191) NOT NULL,
    PRIMARY KEY (`language_name`),
    UNIQUE KEY `language_name` (`language_name`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `details` (
    `dish_id` bigint unsigned NOT NULL,
    `language_name` varchar(191) NOT NULL,
    `dish_name` varchar(191) NOT NULL,
    `dish_instruction` text,
    PRIMARY KEY (`dish_id`, `language_name`),
    CONSTRAINT `fk_dish_details` FOREIGN KEY (`dish_id`) REFERENCES `dish` (`dish_id`) ON DELETE CASCADE,
    CONSTRAINT `fk_detail_language` FOREIGN KEY (`language_name`) REFERENCES `language` (`language_name`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


CREATE TABLE `ingredient` (
    `ingredient_id` bigint unsigned NOT NULL AUTO_INCREMENT,
    PRIMARY KEY (`ingredient_id`),
    UNIQUE KEY `ingredient_id` (`ingredient_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `ingredient_translation` ( 
    `ingredient_id` bigint unsigned NOT NULL,
    `ingredient_name` varchar(191) NOT NULL,
    `language_name` varchar(191) NOT NULL,
    PRIMARY KEY (`ingredient_id`, `language_name`),
    CONSTRAINT `fk_ingredient_name` FOREIGN KEY (`ingredient_id`) REFERENCES `ingredient` (`ingredient_id`) ON DELETE CASCADE,
    CONSTRAINT `fk_ingredient_language` FOREIGN KEY (`language_name`) REFERENCES `language` (`language_name`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `dish_ingredient` (
    `dish_id` bigint unsigned NOT NULL,
    `ingredient_id` bigint unsigned NOT NULL,
    `ingredient_amount` bigint unsigned NOT NULL,
    `ingredient_unit` varchar(191) NOT NULL,
    PRIMARY KEY (`dish_id`,`ingredient_id`),
    CONSTRAINT `fk_dish_ingredient` FOREIGN KEY (`dish_id`) REFERENCES `dish` (`dish_id`) ON DELETE CASCADE,
    CONSTRAINT `fk_ingredient` FOREIGN KEY (ingredient_id) REFERENCES `ingredient` (`ingredient_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `chef` (
    `chef_id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `chef_name` varchar(191) NOT NULL,
    PRIMARY KEY (`chef_name`),
    UNIQUE KEY `chef_id` (`chef_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `dish_chef` (
    `dish_id` bigint unsigned NOT NULL,
    `chef_id` bigint unsigned NOT NULL,
    PRIMARY KEY (`dish_id`, `chef_id`),
    CONSTRAINT `fk_dish_chef` FOREIGN KEY (`dish_id`) REFERENCES `dish` (`dish_id`) ON DELETE CASCADE,
    CONSTRAINT `fk_chef` FOREIGN KEY (`chef_id`) REFERENCES `chef` (`chef_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `location` (
    `location_id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `location_name` varchar(191) NOT NULL,
    `location_address` varchar(191) NOT NULL,
    PRIMARY KEY (`location_id`),
    UNIQUE KEY `location_id` (`location_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `event` (
    `event_id` bigint unsigned NOT NULL AUTO_INCREMENT,
    `event_name` varchar(191) NOT NULL,
    `event_date` datetime NOT NULL,
    `location_id` bigint unsigned NOT NULL,
    PRIMARY KEY (`event_id`),
    UNIQUE KEY `event_id` (`event_id`),
    CONSTRAINT `fk_location` FOREIGN KEY (`location_id`) REFERENCES `location` (`location_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `category` (
    `category_id` bigint unsigned NOT NULL AUTO_INCREMENT,
    PRIMARY KEY (`category_id`),
    UNIQUE KEY `category_id` (`category_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `category_translation` (
    `category_id` bigint unsigned NOT NULL,
    `category_name`varchar(191) NOT NULL,
    `language_name` varchar(191) NOT NULL,
    PRIMARY KEY (`category_id`,`language_name`),
    CONSTRAINT `fk_category_name` FOREIGN KEY (`category_id`) REFERENCES `category` (`category_id`) ON DELETE CASCADE,
    CONSTRAINT `fk_category_langauge` FOREIGN KEY (`language_name`) REFERENCES `language` (`language_name`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;


CREATE TABLE `dish_category` (
    `dish_id` bigint unsigned NOT NULL,
    `category_id` bigint unsigned NOT NULL,
    PRIMARY KEY (`dish_id`,`category_id`),
    CONSTRAINT `fk_dish_category` FOREIGN KEY (`dish_id`) REFERENCES `dish` (`dish_id`) ON DELETE CASCADE,
    CONSTRAINT `fk_category` FOREIGN KEY (`category_id`) REFERENCES `category` (`category_id`)
) ENGINE=InnoDB CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
