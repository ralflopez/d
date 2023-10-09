INSERT INTO organizations (id, name, display_name) VALUES (1, 'test-org', 'test organization');

INSERT INTO warehouses (name, organization_id) VALUES ('default',1);

INSERT INTO users (id, display_name,  username,  password,  organization_id) VALUES (1, 'admin@test-org', 'admin@test-org', 'password', 1);

INSERT INTO user_permissions (user_id, permission_id) VALUES (1, 2);

-- Products
INSERT INTO products (id, sku, brand, name, display_name, description, organization_id, price) VALUES (1, 'sku-1', 'brand x', 'name', 'brand x name', 'description', 1, 100);
INSERT INTO products (id, sku, brand, name, display_name, description, organization_id, price) VALUES (2, 'sku-2', 'brand y', 'name', 'brand y name', 'description', 1, 530);

-- Incoming
INSERT INTO inventory_logs (id, quantity, product_id, action, price, organization_id, warehouse_id) VALUES (
  1,
  10,
  1,
  'INCOMING',
  50,
  1,
  1
);
INSERT INTO inventory_logs (id, quantity, product_id, action, price, organization_id, warehouse_id) VALUES (
  2,
  10,
  2,
  'INCOMING',
  300,
  1,
  1
);

INSERT INTO inventory_transactions (
  id,
  organization_id,
  action
) VALUES (
  1,
  1,
  'DEPOSIT'
);

INSERT INTO inventory_transaction_items (
  id,
  inventory_transaction_id,
  inventory_log_id
) VALUES (
  1,
  1,
  1
);
INSERT INTO inventory_transaction_items (
  id,
  inventory_transaction_id,
  inventory_log_id
) VALUES (
  2,
  1,
  2
);

-- Select
SELECT it.id as id, il.price as purchase_price, il.warehouse_id, it.timestamp as timestamp, it.action as action, p.sku as sku, p.display_name as display_name 
FROM inventory_transaction_items iti
INNER JOIN inventory_logs il ON iti.inventory_log_id = il.id
INNER JOIN inventory_transactions it ON iti.inventory_transaction_id = it.id
INNER JOIN products p ON il.product_id = p.id 
OFFSET 0 LIMIT 100;

-- select stock level
SELECT 
  product_id,
  p.sku,
  p.brand,
  p.name,
  p.description,
  p.price,
  SUM(CASE WHEN action = 'INCOMING' THEN quantity ELSE -quantity END) as quantity
FROM inventory_logs il
JOIN products p
ON p.id = il.product_id
WHERE il.organization_id = 1
GROUP BY product_id, p.sku, p.brand, p.name, p.description, p.price;