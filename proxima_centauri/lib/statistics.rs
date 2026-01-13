use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use tokio::time::Instant;

#[derive(Default)]
struct StatsTable {
    ingress_bytes: u128,
    egress_bytes: u128,
    concurrency: u16,
}

enum LimitValue<T> {
    Unrestricted,
    Restricted(T),
}
pub(crate) struct Limits {
    concurrency: LimitValue<u16>,
    traffic: LimitValue<u128>,
}
impl Default for Limits {
    fn default() -> Self {
        Limits {
            concurrency: LimitValue::Unrestricted,
            traffic: LimitValue::Unrestricted,
        }
    }
}

impl Limits {
    pub(crate) fn with_low_concurrency() -> Self {
        Limits {
            concurrency: LimitValue::Restricted(2),
            traffic: LimitValue::Unrestricted,
        }
    }
}
pub(crate) struct UserContext {
    limits: Limits,
    stats_table: StatsTable,
    last_update_at: Instant,
}
impl UserContext {
    pub(crate) fn new(limits: Limits) -> Self {
        UserContext {
            limits,
            stats_table: StatsTable::default(),
            last_update_at: Instant::now(),
        }
    }
    pub(crate) fn add_ingress_traffic(&mut self, traffic_value: u128) {
        self.stats_table.ingress_bytes += traffic_value;
        self.last_update_at = Instant::now();
    }
    pub(crate) fn add_egress_traffic(&mut self, traffic_value: u128) {
        self.stats_table.egress_bytes += traffic_value;
        self.last_update_at = Instant::now();
    }

    pub(crate) fn inc_concurrency(&mut self) {
        self.stats_table.concurrency += 1;
        self.last_update_at = Instant::now();
    }
    pub(crate) fn dec_concurrency(&mut self) {
        self.stats_table.concurrency -= 1;
        self.last_update_at = Instant::now();
    }
}
pub(crate) struct UsersStatistic {
    inner: HashMap<String, UserContext>,
}

impl UsersStatistic {
    pub(crate) fn new() -> Self {
        UsersStatistic {
            inner: HashMap::new(),
        }
    }
    pub(crate) fn create_user(&mut self, user: &str, limits: Limits) -> Option<UserContext> {
        self.inner
            .insert(user.to_string(), UserContext::new(limits))
    }

    pub(crate) fn add_ingress_traffic(&mut self, user: &str, traffic_value: u128) {
        self.inner
            .entry(user.to_string())
            .and_modify(|ctx| ctx.add_ingress_traffic(traffic_value));
    }
    pub(crate) fn add_egress_traffic(&mut self, user: &str, traffic_value: u128) {
        self.inner
            .entry(user.to_string())
            .and_modify(|ctx| ctx.add_egress_traffic(traffic_value));
    }

    pub(crate) fn inc_concurrency(&mut self, user: &str) {
        self.inner
            .entry(user.to_string())
            .and_modify(|ctx| ctx.inc_concurrency());
    }
    pub(crate) fn dec_concurrency(&mut self, user: &str) {
        self.inner
            .entry(user.to_string())
            .and_modify(|ctx| ctx.dec_concurrency());
    }
}

impl Display for UsersStatistic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (user, ctx) in self.inner.iter() {
            writeln!(
                f,
                "User `{}` stats. ingress: {}, egress: {}",
                user, ctx.stats_table.ingress_bytes, ctx.stats_table.egress_bytes
            )
            .expect("TODO: panic message");
        }
        Ok(())
    }
}

impl Debug for UsersStatistic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (user, ctx) in self.inner.iter() {
            writeln!(
                f,
                "User `{}` stats. ingress: {}, egress: {}",
                user, ctx.stats_table.ingress_bytes, ctx.stats_table.egress_bytes
            )
            .expect("TODO: panic message");
        }
        Ok(())
    }
}
