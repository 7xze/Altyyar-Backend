
# تيار — Tayyar

**منصة المصممين العرب — The Arab Designers Social Network**

<p align="center">
  <img alt="Tayyar" src="./docs/hero-nodes.gif" width="80%" />
</p>

<p align="center">
  <strong>منصة اجتماعية مبنية على ActivityPub — مخصصة للمصممين العرب</strong><br />
  <strong>A social platform built on ActivityPub — tailored for Arab designers</strong>
</p>

---

## 📋 جدول المحتويات | Table of Contents

- [عن تيار | About Tayyar](#عن-تيار--about-tayyar)
- [المميزات الكاملة | Complete Features](#المميزات-الكاملة--complete-features)
- [طريقة العمل | How It Works](#طريقة-العمل--how-it-works)
- [شجرة الملفات | File Tree](#شجرة-الملفات--file-tree)
- [التقنيات المستخدمة | Tech Stack](#التقنيات-المستخدمة--tech-stack)
- [متطلبات التشغيل | Requirements](#متطلبات-التشغيل--requirements)
- [طريقة النشر | Deployment](#طريقة-النشر--deployment)
- [التطبيق الأمامي | Frontend App](#التطبيق-الأمامي--frontend-app)
- [المساهمة | Contributing](#المساهمة--contributing)
- [الترخيص وحقوق الملكية | License & Copyright](#الترخيص-وحقوق-الملكية--license--copyright)

---

## عن تيار | About Tayyar

**تيار** هو باكند (backend) اجتماعي متخصص للمصممين العرب، مبني على أساس **Mastodon** مفتوح المصدر. 
يتيح تيار للمصممين التواصل، مشاركة أعمالهم، اكتشاف مصممين آخرين، وبناء مجتمع إبداعي عربي.

**Tayyar** is a specialized social backend for Arab designers, built on the open-source **Mastodon** platform.
It enables designers to connect, share their work, discover other designers, and build a creative Arab community.

### الرؤية | Vision

أن نكون المنصة الأولى للمصممين العرب، حيث يلتقي الإبداع بالتقنية مفتوحة المصدر.

### الرسالة | Mission

توفير منصة اجتماعية عربية متكاملة، مفتوحة المصدر، تحترم خصوصية المستخدمين، وتمكن المصممين من عرض أعمالهم وبناء شبكة علاقات مهنية.

---

## المميزات الكاملة | Complete Features

### 🌐 المميزات الأساسية | Core Features

| الميزة | Feature | الوصف |
|--------|---------|-------|
| حسابات مخصصة | Custom Profiles | بروفايل كامل مع صورة شخصية، غلاف، سيرة ذاتية، روابط |
| المنشورات | Posts | نشر أعمال تصميم مع صور، فيديو، روابط |
| التفاعل | Interaction | إعجاب، إعادة نشر، تعليق، مشاركة |
| التوقيت الزمني | Timeline | تغذية زمنية حقيقية حسب الترتيب الزمني |
| الهاشتاغات | Hashtags | وسوم لاكتشاف التصاميم والمصممين |
| القوائم | Lists | تنظيم المصممين المتابعين في قوائم مخصصة |
| الإشارات | Mentions | @mention للمصممين الآخرين |
| البحث | Search | بحث متقدم عن مصممين، هاشتاغات، أعمال |

### 🎨 مميزات المصممين | Designer-Specific Features

| الميزة | Feature | الوصف |
|--------|---------|-------|
| معرض الأعمال | Portfolio | عرض التصاميم في معرض مرئي |
| الوسوم المتخصصة | Design Tags | هاشتاغات متخصصة: #تصميم_جرافيك #UI #UX |
| تصنيف المحتوى | Content Categories | فئات: شعارات، بوسترات، واجهات، رسوميات |
| علامة المصمم المحترف | Verified Designer | توثيق حسابات المصممين المعتمدين |
| تحميل عالي الدقة | HD Upload | رفع صور عالية الدقة مناسبة لأعمال التصميم |
| روابط الأعمال | Portfolio Links | إضافة روابط لأعمال سابقة |

### 🔒 الخصوصية والأمان | Privacy & Security

| الميزة | Feature | الوصف |
|--------|---------|-------|
| منشورات خاصة | Private Posts | نشر أعمال خاصة للمتابعين فقط |
| حسابات مقفلة | Locked Accounts | الموافقة على طلبات المتابعة يدوياً |
| فلترة المحتوى | Content Filtering | تصفية الكلمات والمحتوى غير المرغوب |
| كتم وحظر | Mute & Block | كتم أو حظر مستخدمين |
| تقارير الإساءة | Abuse Reports | نظام إبلاغ وإدارة محتوى مسيء |
| المصادقة الثنائية | 2FA | حماية الحساب بمصادقة ثنائية |
| التحكم في الإشعارات | Notification Control | تحكم دقيق بالإشعارات |

### 🔗 الشبكة الموزعة | Federated Network

| الميزة | Feature | الوصف |
|--------|---------|-------|
| الفيديريشن | Federation | اتصال مع خوادم ActivityPub الأخرى |
| متابعة عبر الخوادم | Cross-server Following | متابعة مصممين من خوادم مختلفة |
| التفاعل الموحد | Unified Interaction | إعجاب وتعليق عبر الشبكة بالكامل |
| هجرة الحساب | Account Migration | نقل الحساب بين الخوادم |

### 🛠 أدوات الإدارة | Admin Tools

| الميزة | Feature | الوصف |
|--------|---------|-------|
| لوحة التحكم | Admin Dashboard | لوحة إدارة متكاملة |
| إدارة المستخدمين | User Management | إدارة الحسابات والصلاحيات |
| إدارة الوسائط | Media Management | إدارة المرفقات والصور |
| تقارير وإحصائيات | Reports & Analytics | تحليلات المنصة |
| إعدادات الخادم | Server Settings | ضبط إعدادات الخادم |
| أدوات الإشراف | Moderation Tools | نظام إشراف كامل |
| حظر الخوادم | Domain Blocks | حظر نطاقات محددة |

### 🚀 أداء وقابلية التوسع | Performance & Scalability

| الميزة | Feature | الوصف |
|--------|---------|-------|
| البث المباشر | Streaming API | تحديثات فورية في الوقت الحقيقي |
| المهام الخلفية | Background Jobs | معالجة غير متزامنة عبر Sidekiq |
| التخزين المؤقت | Caching | Redis للتخزين المؤقت عالي الأداء |
| التوزيع | Horizontal Scaling | إمكانية توزيع الأحمال على عدة خوادم |

---

## طريقة العمل | How It Works

### العمارة التقنية | Technical Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Frontend (Android App)                   │
│              تطبيق تيار — Tayyar Android App              │
└─────────────────────┬───────────────────────────────────┘
                      │ REST API / WebSocket Streaming
                      ▼
┌─────────────────────────────────────────────────────────┐
│              Backend — Tayyar (Ruby on Rails)            │
│                                                          │
│  ┌────────────┐  ┌────────────┐  ┌────────────────┐    │
│  │  REST API  │  │  Streaming │  │  Sidekiq Jobs  │    │
│  │ (Rails)    │  │ (Node.js)  │  │ (Redis-backed) │    │
│  └────┬───────┘  └─────┬──────┘  └───────┬────────┘    │
│       │                │                  │              │
│  ┌────▼────────────────▼──────────────────▼──────────┐  │
│  │               PostgreSQL Database                   │  │
│  │  Accounts, Posts, Media, Follows, Interactions     │  │
│  └───────────────────────────────────────────────────┘  │
│                                                          │
│  ┌───────────────────────────────────────────────────┐  │
│  │            ActivityPub Federation                   │  │
│  │  تواصل مع خوادم ActivityPub الأخرى                   │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### سير العمل الأساسي | Basic Workflow

```
مستخدم جاهز → ينشئ حساب → يملأ البروفايل ← يبدأ بمتابعة مصممين
                                           ← ينشر أعماله
                                           ← يتفاعل مع المنشورات
                                           ← يكتشف مصممين جدد
                                           ← يتواصل عبر الفيديريشن
```

1. **التسجيل**: ينشئ المصمم حساباً على تيار
2. **الملف الشخصي**: يملأ معلوماته، يرفع صورته الشخصية وأعماله
3. **النشر**: ينشر تصاميمه مع وصف وهاشتاغات
4. **التفاعل**: يتفاعل مع مصممين آخرين (إعجاب، تعليق، مشاركة)
5. **الاكتشاف**: يكتشف مصممين جدد من خلال التغذية الزمنية والهاشتاغات
6. **الفيديريشن**: يمكنه متابعة مصممين من خوادم ActivityPub أخرى

### دورة حياة المنشور | Post Lifecycle

```
إنشاء منشور → معالجة الوسائط (FFmpeg) → فهرسة → توزيع التغذية الزمنية → إشعار المتابعين
                                                      → فيديريشن للخوادم الأخرى
                                                      → معالجة الصور المصغرة
```

---

## شجرة الملفات | File Tree

```
altyyar-backend/
│
├── app/                              # تطبيق Rails الرئيسي
│   ├── controllers/                  # REST API controllers
│   │   ├── api/                      # API endpoints
│   │   │   ├── v1/                   # API v1
│   │   │   │   ├── accounts/         # حسابات
│   │   │   │   ├── statuses/         # منشورات
│   │   │   │   ├── timelines/        # تغذية زمنية
│   │   │   │   └── admin/           # إدارة
│   │   │   └── v2/                   # API v2
│   │   ├── admin/                    # لوحة التحكم
│   │   ├── concerns/                 # Shared controller logic
│   │   └── settings/                 # إعدادات المستخدم
│   │
│   ├── models/                       # نماذج البيانات
│   │   ├── account.rb                # حساب المستخدم
│   │   ├── status.rb                 # المنشور
│   │   ├── media_attachment.rb       # المرفقات (صور/فيديو)
│   │   ├── follow.rb                 # المتابعة
│   │   ├── favourite.rb              # الإعجاب
│   │   └── ... (100+ models)
│   │
│   ├── services/                     # خدمات التطبيق
│   │   ├── post_status_service.rb    # نشر منشور
│   │   ├── follow_service.rb         # متابعة
│   │   ├── favourite_service.rb      # إعجاب
│   │   ├── notify_service.rb         # إشعارات
│   │   ├── activitypub/              # ActivityPub
│   │   └── admin/                    # خدمات الإدارة
│   │
│   ├── serializers/                  # تسلسل البيانات
│   │   ├── rest/                     # REST JSON serializers
│   │   └── activitypub/              # ActivityPub serializers
│   │
│   ├── workers/                      # خلفية (Sidekiq)
│   │   ├── activitypub/              # توزيع الفيديريشن
│   │   ├── scheduler/                # مهام مجدولة
│   │   └── web/                      # إشعارات ويب
│   │
│   ├── lib/                          # مكتبات
│   ├── validators/                   # مدققات
│   ├── policies/                     # صلاحيات (Pundit)
│   └── presenters/                   # مقدمات
│
├── config/                           # إعدادات
│   ├── routes.rb                     # مسارات API
│   ├── settings.yml                  # إعدادات المنصة
│   ├── locales/                      # ملفات ترجمة
│   └── environments/                 # بيئات التشغيل
│
├── db/                               # قاعدة بيانات
│   ├── migrate/                      # تهجيرات
│   └── schema.rb                     # هيكل DB
│
├── streaming/                        # خادم البث (Node.js)
│   ├── index.js                      # نقطة الدخول
│   ├── redis.js                      # اتصال Redis
│   └── utils.js                      # أدوات
│
├── spec/                             # اختبارات
│   ├── controllers/                  # اختبارات متحكمات
│   ├── models/                       # اختبارات نماذج
│   ├── services/                     # اختبارات خدمات
│   ├── requests/                     # اختبارات API
│   └── workers/                      # اختبارات خلفية
│
├── frontend/                         # واجهة ويب (React)
│   ├── javascript/                   # كود React/Redux
│   │   ├── actions/                  # Redux actions
│   │   ├── reducers/                 # Redux reducers
│   │   ├── components/              # React components
│   │   └── features/                 # Feature modules
│   └── styles/                       # CSS/Sass
│
├── lib/                              # مكتبات Ruby
├── public/                           # ملفات عامة
├── vendor/                           # تبعيات
├── docker-compose.yml                # Docker compose
├── Dockerfile                        # Docker image
├── Gemfile                           # Ruby dependencies
├── package.json                      # Node.js dependencies
├── Procfile                          # Production processes
└── Vagrantfile                       # Dev environment
```

---

## التقنيات المستخدمة | Tech Stack

| التقنية | Technology | الاستخدام |
|---------|------------|-----------|
| **Ruby on Rails** 8.1 | Ruby on Rails | REST API, واجهات ويب |
| **PostgreSQL** 14+ | PostgreSQL | قاعدة البيانات الأساسية |
| **Redis** 7+ | Redis | تخزين مؤقت، طوابير |
| **Sidekiq** 7 | Sidekiq | المهام الخلفية غير المتزامنة |
| **Node.js** 22+ | Node.js | خادم البث المباشر |
| **React** 18 | React | واجهة المستخدم الديناميكية |
| **Redux** | Redux | إدارة حالة UI |
| **ActivityPub** | ActivityPub | بروتوكول الفيديريشن |
| **FFmpeg** 5.1+ | FFmpeg | معالجة الوسائط المتعددة |

---

## متطلبات التشغيل | Requirements

| المتطلب | Requirement | الإصدار |
|----------|-------------|---------|
| Ruby | Ruby | 3.3+ |
| PostgreSQL | PostgreSQL | 14+ |
| Redis | Redis | 7+ |
| Node.js | Node.js | 22+ |
| FFmpeg | FFmpeg | 5.1+ |
| Docker (optional) | Docker | 24+ |

---

## طريقة النشر | Deployment

### باستخدام Docker (مستحسن)

```bash
# سحب وتشغيل
docker compose up -d

# أو بناء من المصدر
docker compose build
docker compose up -d
```

### تثبيت يدوي | Manual Setup

```bash
# Clone
git clone https://github.com/7xze/Altyyar-Backend.git
cd Altyyar-Backend

# Install Ruby dependencies
bundle install

# Install JavaScript dependencies
corepack enable
yarn install

# Setup database
rails db:setup

# Run server
rails server -b 0.0.0.0 -p 3000
```

### متغيرات البيئة الأساسية | Key Environment Variables

```env
# Domain Settings
LOCAL_DOMAIN=tayyar.app
WEB_DOMAIN=tayyar.app

# Database
DB_HOST=localhost
DB_PORT=5432
DB_NAME=tayyar_production
DB_USER=tayyar_user
DB_PASS=strong_password

# Redis
REDIS_HOST=localhost
REDIS_PORT=6379

# Secrets (generate with `rails secret`)
SECRET_KEY_BASE=your_512_bit_secret
OTP_SECRET=your_otp_secret

# VAPID Keys (for push notifications)
VAPID_PRIVATE_KEY=your_vapid_private
VAPID_PUBLIC_KEY=your_vapid_public

# Email (SMTP)
SMTP_SERVER=smtp.example.com
SMTP_PORT=587
SMTP_LOGIN=your_email
SMTP_PASSWORD=your_password
SMTP_FROM_ADDRESS=tayyar@example.com
```

---

## التطبيق الأمامي | Frontend App

التطبيق الأمامي لتطبيق تيار مخصص لمنصة Android، يمكنك الوصول إليه عبر:

**GitHub**: [Altyyar-Android-app](https://github.com/7xze/Altyyar-Android-app-main)

The Tayyar Android frontend app can be found at the repository above.

---

## المساهمة | Contributing

نرحب بمساهماتكم! يرجى قراءة إرشادات المساهمة وقواعد السلوك.

We welcome contributions! Please read our contributing guidelines and code of conduct.

للمساهمة:
1. Fork المستودع
2. إنشاء فرع للميزة (`git checkout -b feature/amazing-feature`)
3. Commit التغييرات (`git commit -m 'Add amazing feature'`)
4. Push إلى الفرع (`git push origin feature/amazing-feature`)
5. فتح Pull Request

---

## الترخيص وحقوق الملكية | License & Copyright

```
Copyright (c) 2025 7X Corporation. All rights reserved.

"تيار" (Tayyar) is based on Mastodon, free open-source software
licensed under GNU Affero General Public License v3.0.

Copyright (c) 2016-2025 Eugen Rochko & other Mastodon contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.
```

### 📞 التواصل | Contact

| | |
|---|---|
| **الشركة** | 7X Corporation |
| **المنصة** | تيار — Tayyar |
| **الفئة المستهدفة** | المصممين العرب |
| **جيت هاب** | [7xze](https://github.com/7xze) |

---

<p align="center">
  <strong>تيار — من المصممين، للمصممين</strong><br />
  <strong>Tayyar — By Designers, For Designers</strong><br />
  <sub>Build for the Arab design community ❤️</sub>
</p>
