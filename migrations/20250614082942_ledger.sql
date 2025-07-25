-- Add migration script here
USE sunminimart;

CREATE TABLE IF NOT EXISTS receipts
(
    id         INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS receipt_items
(
    receipt_id INT UNSIGNED PRIMARY KEY,
    barcode    VARCHAR(64)            NOT NULL,
    cost       DECIMAL(6, 2) UNSIGNED NOT NULL,
    price      DECIMAL(6, 2) UNSIGNED NOT NULL,
    quantity   SMALLINT UNSIGNED      NOT NULL,
    FOREIGN KEY (receipt_id) REFERENCES receipts (id) ON DELETE CASCADE ON UPDATE CASCADE
);
