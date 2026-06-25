tayyar-marketplace/
├── Cargo.toml              # Rust dependencies
├── Dockerfile              # Multi-stage Docker build
├── .env.example            # Environment variables template
├── README.md               # Documentation
│
├── migrations/
│   └── 001_initial.sql     # Database schema
│
└── src/
    ├── main.rs             # Server entry point, router config
    ├── config.rs           # Environment configuration
    ├── db.rs               # Database pool & migrations
    ├── error.rs            # Error handling & response types
    │
    ├── middleware/
    │   └── mod.rs          # Auth: JWT/Mastodon token validation
    │
    ├── models/
    │   ├── mod.rs
    │   ├── service.rs      # MarketplaceService entity
    │   ├── cart.rs         # CartItem entity
    │   ├── order.rs        # Order + OrderItem entities
    │   └── payment.rs      # Payment + webhook entities
    │
    ├── repository/
    │   ├── mod.rs
    │   ├── services.rs     # Service CRUD queries
    │   ├── cart.rs         # Cart operations
    │   ├── orders.rs       # Order creation & queries
    │   └── payments.rs     # Payment & webhook handling
    │
    └── handlers/
        ├── mod.rs
        ├── services.rs     # Service HTTP handlers
        ├── cart.rs         # Cart HTTP handlers
        ├── orders.rs       # Order HTTP handlers
        └── payments.rs     # Payment + webhook handlers
