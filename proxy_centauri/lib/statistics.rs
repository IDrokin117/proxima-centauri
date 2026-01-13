use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

struct StatsTable {
    ingress_bytes: u128,
    egress_bytes: u128,
}

impl StatsTable {
    fn new() -> Self {
        StatsTable {
            ingress_bytes: 0,
            egress_bytes: 0,
        }
    }
}
pub(crate) struct Statistics {
    inner: HashMap<String, StatsTable>,
}

impl Statistics {
    pub(crate) fn new() -> Self {
        Statistics {
            inner: HashMap::new(),
        }
    }

    pub(crate) fn add_ingress_traffic(&mut self, user: &str, traffic_value: u64) {
        match self.inner.entry(user.to_string()) {
            Entry::Occupied(mut st) => {
                st.get_mut().ingress_bytes += traffic_value as u128;
            }
            Entry::Vacant(vst) => {
                let mut stats_table = StatsTable::new();
                stats_table.ingress_bytes += traffic_value as u128;
                vst.insert(stats_table);
            }
        }
    }
    pub(crate) fn add_egress_traffic(&mut self, user: &str, traffic_value: u64) {
        match self.inner.entry(user.to_string()) {
            Entry::Occupied(mut st) => {
                st.get_mut().egress_bytes += traffic_value as u128;
            }
            Entry::Vacant(vst) => {
                let mut stats_table = StatsTable::new();
                stats_table.egress_bytes += traffic_value as u128;
                vst.insert(stats_table);
            }
        }
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (user, stats_table) in self.inner.iter() {
            writeln!(
                f,
                "User `{}` stats. ingress: {}, egress: {}",
                user, stats_table.ingress_bytes, stats_table.egress_bytes
            )
            .expect("TODO: panic message");
        }
        Ok(())
    }
}

impl Debug for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (user, stats_table) in self.inner.iter() {
            writeln!(
                f,
                "User `{}` stats. ingress: {}, egress: {}",
                user, stats_table.ingress_bytes, stats_table.egress_bytes
            )
            .expect("TODO: panic message");
        }
        Ok(())
    }
}
