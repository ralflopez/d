DROP TABLE IF EXISTS inventory_logs;
DROP TABLE IF EXISTS inventory_transactions;
DROP TABLE IF EXISTS product_categories;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS warehouses;
DROP TABLE IF EXISTS user_permissions;
DROP TABLE IF EXISTS permissions;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS organizations;
DROP TABLE IF EXISTS flyway_schema_history;
DROP TYPE inventory_log_action;
DROP TYPE inventory_transaction_action;

CREATE TABLE IF NOT EXISTS organizations (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(255) NOT NULL UNIQUE,
  display_name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  display_name VARCHAR(255) NOT NULL,
  username VARCHAR(255) NOT NULL,
  password TEXT NOT NULL,
  organization_id BIGINT,

  UNIQUE(username, organization_id),

  CONSTRAINT fk_users_organizations
    FOREIGN KEY(organization_id)
	  REFERENCES organizations(id)
	  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS permissions (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,

  UNIQUE(name)
);

CREATE TABLE IF NOT EXISTS user_permissions (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  user_id BIGINT NOT NULL,
  permission_id BIGINT NOT NULL,

  UNIQUE(user_id, permission_id),

  CONSTRAINT fk_user_permissions_users
    FOREIGN KEY(user_id)
    REFERENCES users(id)
    ON DELETE CASCADE,
  CONSTRAINT fk_user_permissions_permissions
    FOREIGN KEY(permission_id)
    REFERENCES permissions(id)
    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS warehouses (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(255) DEFAULT 'default',
  organization_id BIGINT NOT NULL,

  UNIQUE(name, organization_id),

  CONSTRAINT fk_warehouses_organizations
    FOREIGN KEY(organization_id)
	  REFERENCES organizations(id)
	  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS products (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  sku VARCHAR(255) NOT NULL,
  brand VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  display_name TEXT NOT NULL,
  description VARCHAR(255) NOT NULL,
  organization_id BIGINT NOT NULL,
  price NUMERIC NOT NULL DEFAULT 0,

  UNIQUE(sku, organization_id),

  CONSTRAINT fk_products_organizations
    FOREIGN KEY(organization_id)
	  REFERENCES organizations(id)
	  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS categories (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(255) NOT NULL UNIQUE,
  organization_id BIGINT NOT NULL,

  CONSTRAINT fk_category_organizations
    FOREIGN KEY(organization_id)
	  REFERENCES organizations(id)
	  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS product_categories (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  category_id BIGINT NOT NULL,
  product_id BIGINT NOT NULL,

  CONSTRAINT fk_product_categories_categories
    FOREIGN KEY(category_id)
	  REFERENCES categories(id)
	  ON DELETE CASCADE,

  CONSTRAINT fk_product_categories_products
    FOREIGN KEY(product_id)
	  REFERENCES products(id)
	  ON DELETE CASCADE
);

CREATE TYPE inventory_transaction_action AS ENUM (
  'SALES', 'DEPOSIT', 'SALES_ROLLBACK', 'DEPOSIT_ROLLBACK'
);

CREATE TABLE IF NOT EXISTS inventory_transactions (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  timestamp TIMESTAMPTZ DEFAULT NOW() NOT NULL,
  organization_id BIGINT NOT NULL,
  action inventory_transaction_action NOT NULL,

  CONSTRAINT fk_inventory_transactions_organizations
      FOREIGN KEY(organization_id)
  	  REFERENCES organizations(id)
  	  ON DELETE CASCADE
);

CREATE TYPE inventory_log_action AS ENUM (
  'INCOMING', 'OUTGOING'
);

CREATE TABLE IF NOT EXISTS inventory_logs (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  quantity INTEGER NOT NULL,
  product_id BIGINT NOT NULL,
  action inventory_log_action NOT NULL,
  timestamp TIMESTAMPTZ DEFAULT NOW() NOT NULL,
  price NUMERIC NOT NULL,
  organization_id BIGINT NOT NULL,
  warehouse_id BIGINT NOT NULL,
  inventory_transaction_id BIGINT,

  CONSTRAINT fk_inventory_log_organizations
    FOREIGN KEY(organization_id)
	  REFERENCES organizations(id)
	  ON DELETE CASCADE,

  CONSTRAINT fk_inventory_log_products
    FOREIGN KEY(product_id)
	  REFERENCES products(id)
	  ON DELETE CASCADE,

  CONSTRAINT fk_inventory_log_warehouses
    FOREIGN KEY(warehouse_id)
    REFERENCES warehouses(id)
    ON DELETE SET NULL,

  CONSTRAINT fk_inventory_logs_inventory_transactions
    FOREIGN KEY(inventory_transaction_id)
    REFERENCES inventory_transactions(id)
);

INSERT INTO permissions (id, name) VALUES (1, 'superuser');
INSERT INTO permissions (id, name) VALUES (2, 'organization:*');

INSERT INTO users (id, display_name,  username,  password) VALUES (0, 'superuser', 'superuser', '$2b$12$e1RNRrjdu7b6jeg0AMN.9u3TgvfeqjSdc8uqGdIkdmRs6jh7JU0hi');
INSERT INTO user_permissions (id, user_id, permission_id) VALUES (0, 0, 1);