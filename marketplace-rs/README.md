# Tayyar Marketplace — سوق تيار

خدمة السوق لمنصة تيار، مبنية بلغة **Rust** بإطار **Axum**.

## العمارة | Architecture

```
┌─────────────────────────────────────────┐
│          Tayyar Marketplace API          │
│              Rust / Axum                │
│                                          │
│  ┌──────────┐  ┌────────┐  ┌────────┐  │
│  │ Services │  │  Cart  │  │ Orders │  │
│  └────┬─────┘  └───┬────┘  └───┬────┘  │
│       │            │           │        │
│  ┌────▼────────────▼───────────▼────┐  │
│  │       PostgreSQL Database        │  │
│  └─────────────────────────────────┘  │
│                                          │
│  ┌─────────────────────────────────┐  │
│  │   Moyasar Payment Gateway      │  │
│  │   (Mada, Visa, Apple Pay)      │  │
│  └─────────────────────────────────┘  │
└─────────────────────────────────────────┘
         │
         ▼ Mastodon OAuth Validation
    ┌────────────────┐
    │  Rails API     │
    └────────────────┘
```

## التقنيات | Tech Stack

- **Rust** 1.83+ مع Axum 0.7
- **PostgreSQL** 14+ (مشاركة مع Rails أو منفصلة)
- **SQLx** (استعلامات آمنة مع التحقق في وقت الترجمة)
- **Moyasar** لمعالجة المدفوعات (يدعم Mada, Visa, Mastercard, Apple Pay, STC Pay)

## API Endpoints

### Services (الخدمات)
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/marketplace/services` | No | List active services (optional `?category=DESIGN`) |
| GET | `/api/marketplace/services/search?q=...` | No | Search services |
| GET | `/api/marketplace/services/{id}` | No | Get service details |
| POST | `/api/marketplace/services` | Yes | Create service |
| PUT | `/api/marketplace/services/{id}` | Yes | Update service |
| DELETE | `/api/marketplace/services/{id}` | Yes | Soft-delete service |
| GET | `/api/marketplace/seller/{id}/services` | No | Get seller's services |
| GET | `/api/marketplace/seller/services` | Yes | Get my services |

### Cart (السلة)
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| GET | `/api/marketplace/cart` | Yes | Get cart |
| POST | `/api/marketplace/cart` | Yes | Add to cart `{"service_id":"..."}` |
| PUT | `/api/marketplace/cart/{service_id}` | Yes | Update quantity `{"quantity":2}` |
| DELETE | `/api/marketplace/cart/{service_id}` | Yes | Remove item |
| DELETE | `/api/marketplace/cart` | Yes | Clear cart |

### Orders (الطلبات)
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/marketplace/orders` | Yes | Create order from cart |
| GET | `/api/marketplace/orders` | Yes | Get my orders |
| GET | `/api/marketplace/orders/{id}` | Yes | Get order details |

### Payments (المدفوعات)
| Method | Endpoint | Auth | Description |
|--------|----------|------|-------------|
| POST | `/api/marketplace/payments/intent` | Yes | Create payment intent |
| GET | `/api/marketplace/payments/order/{id}` | Yes | Get payment status |
| POST | `/api/webhooks/moyasar` | No | Moyasar webhook |

## Authentication

يستخدم المصادقة عبر **Mastodon OAuth Token**. أضف الهيدر:
```
Authorization: Bearer <your_mastodon_token>
```

يتم التحقق من التوكن عبر API ماستدون وتخزين النتيجة في cache لمدة 5 دقائق.

## Database Tables

- `marketplace_services` — قائمة الخدمات
- `cart_items` — عناصر السلة
- `orders` — الطلبات
- `order_items` — عناصر الطلب
- `payments` — المدفوعات ومعاملات Moyasar

## التثبيت والتشغيل

```bash
# 1. إنشاء قاعدة البيانات
createdb tayyar_marketplace

# 2. نسخ الإعدادات
cp .env.example .env
# عدل .env بالقيم الصحيحة

# 3. تشغيل
cargo run --release

# أو باستخدام Docker
docker build -t tayyar-marketplace .
docker run -p 8080:8080 --env-file .env tayyar-marketplace
```

## المتغيرات البيئية

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection string |
| `MASTODON_API_BASE` | Mastodon Rails API base URL |
| `MOYASAR_SECRET_KEY` | Moyasar secret API key |
| `MOYASAR_PUBLISHABLE_KEY` | Moyasar publishable key |
| `WEBHOOK_SECRET` | Secret for webhook signature validation |
| `RUST_LOG` | Logging level |

## License

Copyright (c) 2025 7X Corporation. AGPLv3.
