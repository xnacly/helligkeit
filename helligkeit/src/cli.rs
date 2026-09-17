use std::{fmt::Display, str::FromStr};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true)]
    /// Match devices by substring instead of exact name
    pub like: bool,

    #[command(subcommand)]
    pub command: Option<Action>,

    /// disable stdout/stderr writes
    #[arg(short, long, global = true)]
    pub silent: bool,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Action {
    /// Device info
    Info {
        /// Device to operate on, defaults to the primary backlight
        device: Option<String>,
    },
    /// Get current brightness of device
    Get {
        /// Device to operate on, defaults to the primary backlight
        device: Option<String>,
    },
    /// Set device brightness, either absolute (300), as percentage of the
    /// devices maximum (50%) or relative to the current value (+10, -5%)
    Set {
        #[arg(allow_hyphen_values = true)]
        value: Adjust,
        /// Device to operate on, defaults to the primary backlight
        device: Option<String>,
    },

    /// List all controllable and supported devices
    List,
}

/// A brightness value, either in device units or as percentage of the devices
/// max brightness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Absolute(usize),
    Percent(usize),
}

impl Value {
    /// convert to device units, percentages are rounded to the nearest step
    fn absolute(self, max: usize) -> usize {
        match self {
            Value::Absolute(n) => n,
            Value::Percent(p) => (max * p + 50) / 100,
        }
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, percent) = match s.strip_suffix('%') {
            Some(num) => (num, true),
            None => (s, false),
        };

        let n: usize = num
            .parse()
            .map_err(|_| format!("'{s}' is not a valid brightness value"))?;

        if percent {
            if n > 100 {
                return Err(format!("'{s}' exceeds 100%"));
            }
            Ok(Value::Percent(n))
        } else {
            Ok(Value::Absolute(n))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Absolute(n) => write!(f, "{n}"),
            Value::Percent(p) => write!(f, "{p}%"),
        }
    }
}

/// A brightness adjustment, either absolute or relative to the current value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjust {
    Set(Value),
    Increase(Value),
    Decrease(Value),
}

impl Adjust {
    /// whether resolving this adjustment requires the current brightness
    pub fn is_relative(&self) -> bool {
        !matches!(self, Adjust::Set(_))
    }

    /// resolve to device units, clamped to `0..=max`
    pub fn resolve(&self, current: usize, max: usize) -> usize {
        match self {
            Adjust::Set(v) => v.absolute(max).min(max),
            Adjust::Increase(v) => current.saturating_add(v.absolute(max)).min(max),
            Adjust::Decrease(v) => current.saturating_sub(v.absolute(max)),
        }
    }
}

impl FromStr for Adjust {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix('+') {
            rest.parse().map(Adjust::Increase)
        } else if let Some(rest) = s.strip_prefix('-') {
            rest.parse().map(Adjust::Decrease)
        } else {
            s.parse().map(Adjust::Set)
        }
    }
}

impl Display for Adjust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Adjust::Set(v) => write!(f, "{v}"),
            Adjust::Increase(v) => write!(f, "+{v}"),
            Adjust::Decrease(v) => write!(f, "-{v}"),
        }
    }
}

impl Action {
    /// device the action was given, if any
    pub fn device(&self) -> Option<&str> {
        match self {
            Action::Info { device } | Action::Get { device } | Action::Set { device, .. } => {
                device.as_deref()
            }
            Action::List => None,
        }
    }
}
