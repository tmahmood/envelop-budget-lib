-- Your SQL goes here
CREATE TABLE budget_accounts
(
    id           INTEGER   NOT NULL PRIMARY KEY,
    filed_as     VARCHAR   NOT NULL UNIQUE,
    date_created TIMESTAMP NOT NULL
);

CREATE TABLE transactions
(
    id                  INTEGER          NOT NULL PRIMARY KEY,
    note                VARCHAR          NOT NULL,
    payee               VARCHAR          NOT NULL,
    date_created        TIMESTAMP        NOT NULL,
    amount              DOUBLE PRECISION NOT NULL,
    category_id         INTEGER          NOT NULL,
    income              BOOLEAN          NOT NULL DEFAULT 0,
    transaction_type_id INTEGER          NOT NULL DEFAULT 1,
    FOREIGN KEY (category_id) REFERENCES categories (id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (transaction_type_id) REFERENCES transaction_types (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE categories
(
    id                INTEGER          NOT NULL PRIMARY KEY,
    name              VARCHAR          NOT NULL,
    allocated         DOUBLE PRECISION NOT NULL,
    budget_account_id INTEGER          NOT NULL,
    UNIQUE (name, budget_account_id),
    FOREIGN KEY (budget_account_id) REFERENCES budget_accounts (id) ON UPDATE CASCADE ON DELETE CASCADE
);

create table transaction_types
(
    id   INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    UNIQUE (name)
);

insert into transaction_types (name)
values ("Income"),
       ("Expense"),
       ("TransferIn"),
       ("TransferOut")
;