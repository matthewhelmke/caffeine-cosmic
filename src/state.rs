use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use zbus::zvariant::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, Type)]
pub enum TimerSelection {
    #[default]
    Infinity,
    OneHour,
    TwoHours,
    Manual,
}

impl TimerSelection {
    pub fn label(&self) -> &'static str {
        match self {
            TimerSelection::Infinity => "Infinity",
            TimerSelection::OneHour => "1 Hour",
            TimerSelection::TwoHours => "2 Hours",
            TimerSelection::Manual => "Manual",
        }
    }

    pub fn duration_secs(&self, manual_mins: Option<u64>) -> Option<u64> {
        match self {
            TimerSelection::Infinity => None,
            TimerSelection::OneHour => Some(3600),
            TimerSelection::TwoHours => Some(7200),
            TimerSelection::Manual => manual_mins.map(|m| m * 60),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, Type)]
pub struct CaffeineState {
    pub active: bool,
    pub selection: TimerSelection,
    pub expiry_ts: i64, // -1 for None, else timestamp
}

impl CaffeineState {
    pub fn inactive() -> Self {
        Self {
            active: false,
            selection: TimerSelection::default(),
            expiry_ts: -1,
        }
    }

    pub fn active(selection: TimerSelection, expiry_ts: Option<u64>) -> Self {
        Self {
            active: true,
            selection,
            expiry_ts: expiry_ts.map(|t| t as i64).unwrap_or(-1),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn remaining_secs(&self) -> Option<u64> {
        if !self.active || self.expiry_ts == -1 {
            return None;
        }
        let ts = self.expiry_ts as u64;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(std::time::Duration::from_secs(0))
            .as_secs();
        if ts > now {
            Some(ts - now)
        } else {
            Some(0)
        }
    }
}
