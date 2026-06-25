-- Marketplace Services
CREATE TABLE IF NOT EXISTS marketplace_services (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title       VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    price       BIGINT NOT NULL, -- price in halalas/cents (SAR 50.00 = 5000)
    images      TEXT[] NOT NULL DEFAULT '{}',
    seller_id   VARCHAR(255) NOT NULL,
    seller_name VARCHAR(255) NOT NULL,
    category    VARCHAR(50) NOT NULL CHECK (category IN ('DESIGN', 'PHOTOGRAPHY', 'EDITING')),
    is_active   BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_services_seller ON marketplace_services(seller_id);
CREATE INDEX idx_services_category ON marketplace_services(category);
CREATE INDEX idx_services_active ON marketplace_services(is_active);
CREATE INDEX idx_services_search ON marketplace_services USING gin(to_tsvector('simple', title || ' ' || description));

-- Cart Items
CREATE TABLE IF NOT EXISTS cart_items (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     VARCHAR(255) NOT NULL,
    service_id  UUID NOT NULL REFERENCES marketplace_services(id) ON DELETE CASCADE,
    quantity    INT NOT NULL DEFAULT 1 CHECK (quantity > 0),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, service_id)
);

CREATE INDEX idx_cart_user ON cart_items(user_id);

-- Orders
CREATE TABLE IF NOT EXISTS orders (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    buyer_id        VARCHAR(255) NOT NULL,
    total_price     BIGINT NOT NULL,
    status          VARCHAR(50) NOT NULL DEFAULT 'PENDING'
                    CHECK (status IN ('PENDING', 'CONFIRMED', 'IN_PROGRESS', 'COMPLETED', 'CANCELLED')),
    notes           TEXT NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_orders_buyer ON orders(buyer_id);
CREATE INDEX idx_orders_status ON orders(status);

-- Order Items
CREATE TABLE IF NOT EXISTS order_items (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id    UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    service_id  UUID NOT NULL REFERENCES marketplace_services(id),
    title       VARCHAR(255) NOT NULL,
    price       BIGINT NOT NULL,
    quantity    INT NOT NULL DEFAULT 1
);

CREATE INDEX idx_order_items_order ON order_items(order_id);

-- Payments
CREATE TABLE IF NOT EXISTS payments (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id        UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    amount          BIGINT NOT NULL,
    currency        VARCHAR(3) NOT NULL DEFAULT 'SAR',
    status          VARCHAR(50) NOT NULL DEFAULT 'PENDING'
                    CHECK (status IN ('PENDING', 'PAID', 'FAILED', 'REFUNDED')),
    payment_method  VARCHAR(50) NOT NULL DEFAULT 'moyasar',
    gateway_id      VARCHAR(255) NOT NULL DEFAULT '',
    gateway_response JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_order ON payments(order_id);
CREATE INDEX idx_payments_gateway ON payments(gateway_id);
