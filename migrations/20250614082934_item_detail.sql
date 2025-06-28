-- Add migration script here
USE sunminimart;

CREATE TABLE IF NOT EXISTS items (
    barcode VARCHAR(64) PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    cost DECIMAL(6, 2) UNSIGNED NOT NULL,
    price DECIMAL(6, 2) UNSIGNED NOT NULL,
    quantity SMALLINT NOT NULL,
    image BLOB
);

CREATE TABLE IF NOT EXISTS expire_dates (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    ref_barcode VARCHAR(64) NOT NULL,
    expire_date DATE NOT NULL,
    FOREIGN KEY (ref_barcode) REFERENCES items(barcode) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE IF NOT EXISTS bulk_items (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    barcode VARCHAR(64) UNIQUE,
    ref_barcode VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    price DECIMAL(6, 2) UNSIGNED NOT NULL,
    quantity SMALLINT NOT NULL,
    image BLOB,
    FOREIGN KEY (ref_barcode) REFERENCES items(barcode) ON DELETE CASCADE ON UPDATE CASCADE
);

