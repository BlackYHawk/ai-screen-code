use crate::models::{BankCard, Order, Subscription, SubscriptionStatus, User};
use chrono::Utc;
use rusqlite::{Connection, Result, params};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        Self::new_with_path(&db_path)
    }

    /// Create a database with a custom path (for testing)
    pub fn new_with_path(db_path: &PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        db.init_tables()?;

        Ok(db)
    }

    fn get_db_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("ai-screen-code");
        path.push("data");
        path.push("app.db");
        path
    }

    fn init_tables(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // Enable foreign key constraints
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                nickname TEXT,
                avatar TEXT,
                provider VARCHAR(50),
                provider_id VARCHAR(255),
                provider_token TEXT,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS bank_cards (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                card_number_last4 TEXT NOT NULL,
                bank_name TEXT NOT NULL,
                card_holder_name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_bank_cards_user_id ON bank_cards(user_id)",
            [],
        )?;

        // 订阅表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS subscriptions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                plan TEXT NOT NULL,
                status TEXT NOT NULL,
                start_date TEXT NOT NULL,
                end_date TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id ON subscriptions(user_id)",
            [],
        )?;

        // 复合索引：加速用户有效订阅查询
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_subscriptions_user_status_end
             ON subscriptions(user_id, status, end_date)",
            [],
        )?;

        // 订单表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS orders (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                plan TEXT NOT NULL,
                amount INTEGER NOT NULL,
                payment_method TEXT NOT NULL,
                status TEXT NOT NULL,
                trade_no TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id)",
            [],
        )?;

        // 索引：加速订单状态查询
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status)",
            [],
        )?;

        // 复合索引：加速用户订单查询
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_orders_user_created
             ON orders(user_id, created_at DESC)",
            [],
        )?;

        // 验证码表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS verification_codes (
                id TEXT PRIMARY KEY,
                email VARCHAR(255) NOT NULL,
                code VARCHAR(10) NOT NULL,
                type VARCHAR(20) NOT NULL,
                expires_at TEXT NOT NULL,
                used BOOLEAN DEFAULT FALSE,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_verification_codes_email ON verification_codes(email)",
            [],
        )?;

        // Create default guest user
        conn.execute(
            "INSERT OR IGNORE INTO users (id, username, email, password_hash, nickname, avatar, created_at)
             VALUES ('guest', 'guest', 'guest@example.com', '$2b$10$dummy', 'Guest', NULL, datetime('now'))",
            [],
        )?;

        tracing::info!("Database tables initialized");

        Ok(())
    }

    /// 创建用户
    pub fn create_user(&self, user: &User) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, nickname, avatar, provider, provider_id, provider_token, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                user.id,
                user.username,
                user.email,
                user.password_hash,
                user.nickname,
                user.avatar,
                user.provider,
                user.provider_id,
                user.provider_token,
                user.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// 根据邮箱查找用户
    pub fn find_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, nickname, avatar, provider, provider_id, provider_token, created_at
             FROM users WHERE email = ?1",
        )?;

        let mut rows = stmt.query(params![email])?;

        if let Some(row) = rows.next()? {
            Ok(Some(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                nickname: row.get(4)?,
                avatar: row.get(5)?,
                provider: row.get(6)?,
                provider_id: row.get(7)?,
                provider_token: row.get(8)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// 根据ID查找用户
    pub fn find_user_by_id(&self, id: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, nickname, avatar, provider, provider_id, provider_token, created_at
             FROM users WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                nickname: row.get(4)?,
                avatar: row.get(5)?,
                provider: row.get(6)?,
                provider_id: row.get(7)?,
                provider_token: row.get(8)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// 根据用户名查找用户
    pub fn find_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, nickname, avatar, provider, provider_id, provider_token, created_at
             FROM users WHERE username = ?1",
        )?;

        let mut rows = stmt.query(params![username])?;

        if let Some(row) = rows.next()? {
            Ok(Some(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                nickname: row.get(4)?,
                avatar: row.get(5)?,
                provider: row.get(6)?,
                provider_id: row.get(7)?,
                provider_token: row.get(8)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新用户信息
    pub fn update_user(&self, user: &User) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE users SET nickname = ?1, avatar = ?2 WHERE id = ?3",
            params![user.nickname, user.avatar, user.id],
        )?;
        Ok(())
    }

    /// 添加银行卡
    pub fn add_bank_card(&self, card: &BankCard) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO bank_cards (id, user_id, card_number_last4, bank_name, card_holder_name, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                card.id,
                card.user_id,
                card.card_number_last4,
                card.bank_name,
                card.card_holder_name,
                card.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// 获取用户的所有银行卡
    pub fn get_user_bank_cards(&self, user_id: &str) -> Result<Vec<BankCard>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, card_number_last4, bank_name, card_holder_name, created_at
             FROM bank_cards WHERE user_id = ?1 ORDER BY created_at DESC",
        )?;

        let rows = stmt.query_map(params![user_id], |row| {
            Ok(BankCard {
                id: row.get(0)?,
                user_id: row.get(1)?,
                card_number_last4: row.get(2)?,
                bank_name: row.get(3)?,
                card_holder_name: row.get(4)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let mut cards = Vec::new();
        for card in rows {
            cards.push(card?);
        }
        Ok(cards)
    }

    /// 删除银行卡
    pub fn delete_bank_card(&self, card_id: &str, user_id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute(
            "DELETE FROM bank_cards WHERE id = ?1 AND user_id = ?2",
            params![card_id, user_id],
        )?;
        Ok(affected > 0)
    }

    // ============ 订阅相关方法 ============

    /// 创建订阅
    pub fn create_subscription(&self, sub: &Subscription) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO subscriptions (id, user_id, plan, status, start_date, end_date, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                sub.id,
                sub.user_id,
                sub.plan,
                sub.status.to_string(),
                sub.start_date.to_rfc3339(),
                sub.end_date.to_rfc3339(),
                sub.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// 获取用户的有效订阅
    pub fn get_active_subscription(&self, user_id: &str) -> Result<Option<Subscription>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, plan, status, start_date, end_date, created_at
             FROM subscriptions WHERE user_id = ?1 AND status = 'active' AND end_date > ?2
             ORDER BY end_date DESC LIMIT 1",
        )?;

        let now = Utc::now().to_rfc3339();
        let mut rows = stmt.query(params![user_id, now])?;

        if let Some(row) = rows.next()? {
            let status_str: String = row.get(3)?;
            let status = match status_str.as_str() {
                "active" => SubscriptionStatus::Active,
                "expired" => SubscriptionStatus::Expired,
                "cancelled" => SubscriptionStatus::Cancelled,
                _ => SubscriptionStatus::Expired,
            };
            Ok(Some(Subscription {
                id: row.get(0)?,
                user_id: row.get(1)?,
                plan: row.get(2)?,
                status,
                start_date: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                end_date: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新订阅状态
    pub fn update_subscription_status(
        &self,
        sub_id: &str,
        status: SubscriptionStatus,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE subscriptions SET status = ?1 WHERE id = ?2",
            params![status.to_string(), sub_id],
        )?;
        Ok(())
    }

    // ============ 订单相关方法 ============

    /// 创建订单
    pub fn create_order(&self, order: &Order) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO orders (id, user_id, plan, amount, payment_method, status, trade_no, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                order.id,
                order.user_id,
                order.plan,
                order.amount,
                order.payment_method.to_string(),
                order.status.to_string(),
                order.trade_no,
                order.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// 根据ID查找订单
    pub fn find_order_by_id(&self, order_id: &str) -> Result<Option<Order>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, plan, amount, payment_method, status, trade_no, created_at
             FROM orders WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![order_id])?;

        if let Some(row) = rows.next()? {
            let payment_method_str: String = row.get(4)?;
            let status_str: String = row.get(5)?;
            Ok(Some(Order {
                id: row.get(0)?,
                user_id: row.get(1)?,
                plan: row.get(2)?,
                amount: row.get(3)?,
                payment_method: payment_method_str
                    .parse()
                    .unwrap_or(crate::models::PaymentMethod::Alipay),
                status: match status_str.as_str() {
                    "pending" => crate::models::OrderStatus::Pending,
                    "paid" => crate::models::OrderStatus::Paid,
                    "cancelled" => crate::models::OrderStatus::Cancelled,
                    _ => crate::models::OrderStatus::Pending,
                },
                trade_no: row.get(6)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新订单状态
    pub fn update_order_status(
        &self,
        order_id: &str,
        status: crate::models::OrderStatus,
        trade_no: Option<String>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE orders SET status = ?1, trade_no = ?2 WHERE id = ?3",
            params![status.to_string(), trade_no, order_id],
        )?;
        Ok(())
    }

    /// 获取用户的所有订单
    pub fn get_user_orders(&self, user_id: &str) -> Result<Vec<Order>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, plan, amount, payment_method, status, trade_no, created_at
             FROM orders WHERE user_id = ?1 ORDER BY created_at DESC",
        )?;

        let rows = stmt.query_map(params![user_id], |row| {
            let payment_method_str: String = row.get(4)?;
            let status_str: String = row.get(5)?;
            Ok(Order {
                id: row.get(0)?,
                user_id: row.get(1)?,
                plan: row.get(2)?,
                amount: row.get(3)?,
                payment_method: payment_method_str
                    .parse()
                    .unwrap_or(crate::models::PaymentMethod::Alipay),
                status: match status_str.as_str() {
                    "pending" => crate::models::OrderStatus::Pending,
                    "paid" => crate::models::OrderStatus::Paid,
                    "cancelled" => crate::models::OrderStatus::Cancelled,
                    _ => crate::models::OrderStatus::Pending,
                },
                trade_no: row.get(6)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let mut orders = Vec::new();
        for order in rows {
            orders.push(order?);
        }
        Ok(orders)
    }

    // ============ 第三方登录相关方法 ============

    /// 根据provider和provider_id查找用户
    pub fn find_user_by_provider(&self, provider: &str, provider_id: &str) -> Result<Option<User>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, email, password_hash, nickname, avatar, provider, provider_id, provider_token, created_at
             FROM users WHERE provider = ?1 AND provider_id = ?2",
        )?;

        let mut rows = stmt.query(params![provider, provider_id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                nickname: row.get(4)?,
                avatar: row.get(5)?,
                provider: row.get(6)?,
                provider_id: row.get(7)?,
                provider_token: row.get(8)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// 更新用户密码
    pub fn update_user_password(&self, user_id: &str, password_hash: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE users SET password_hash = ?1 WHERE id = ?2",
            params![password_hash, user_id],
        )?;
        Ok(())
    }

    /// 更新用户绑定第三方账号
    pub fn update_user_provider(
        &self,
        user_id: &str,
        provider: &str,
        provider_id: &str,
        provider_token: Option<&str>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE users SET provider = ?1, provider_id = ?2, provider_token = ?3 WHERE id = ?4",
            params![provider, provider_id, provider_token, user_id],
        )?;
        Ok(())
    }

    // ============ 验证码相关方法 ============

    /// 创建验证码
    pub fn create_verification_code(&self, code: &crate::models::VerificationCode) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO verification_codes (id, email, code, type, expires_at, used, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                code.id,
                code.email,
                code.code,
                code.code_type,
                code.expires_at.to_rfc3339(),
                code.used,
                code.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    /// 查找有效的验证码
    pub fn find_valid_verification_code(
        &self,
        email: &str,
        code: &str,
        code_type: &str,
    ) -> Result<Option<crate::models::VerificationCode>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, email, code, type, expires_at, used, created_at
             FROM verification_codes WHERE email = ?1 AND code = ?2 AND type = ?3 AND used = FALSE AND expires_at > ?4
             ORDER BY created_at DESC LIMIT 1",
        )?;

        let now = Utc::now().to_rfc3339();
        let mut rows = stmt.query(params![email, code, code_type, now])?;

        if let Some(row) = rows.next()? {
            Ok(Some(crate::models::VerificationCode {
                id: row.get(0)?,
                email: row.get(1)?,
                code: row.get(2)?,
                code_type: row.get(3)?,
                expires_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                used: row.get(5)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// 标记验证码已使用
    pub fn mark_verification_code_used(&self, code_id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE verification_codes SET used = TRUE WHERE id = ?1",
            params![code_id],
        )?;
        Ok(())
    }

    /// 检查是否可以发送验证码（限流）
    pub fn can_send_verification_code(&self, email: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT created_at FROM verification_codes WHERE email = ?1 ORDER BY created_at DESC LIMIT 1",
        )?;

        let mut rows = stmt.query(params![email])?;

        if let Some(row) = rows.next()? {
            let created_at: String = row.get(0)?;
            let created = chrono::DateTime::parse_from_rfc3339(&created_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now());
            // 60秒内只能发送一次
            let seconds_since_last = (Utc::now() - created).num_seconds();
            Ok(seconds_since_last >= 60)
        } else {
            Ok(true)
        }
    }
}
