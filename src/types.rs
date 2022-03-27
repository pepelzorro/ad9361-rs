//! Rust types for AD9361

use crate::bindings;

/// TX RF Port Selection
///
/// tx_rf_port_input_select
pub enum TxRfPortSelection {
    TXA = 0,
    TXB = 1,
}
impl From<u32> for TxRfPortSelection {
    fn from(a: u32) -> Self {
        match a {
            0 => TxRfPortSelection::TXA,
            _ => TxRfPortSelection::TXB,
        }
    }
}
impl From<TxRfPortSelection> for u32 {
    fn from(a: TxRfPortSelection) -> Self {
        a as u32
    }
}

/// RX RF Port Selection
///
/// rx_rf_port_input_select
#[allow(non_camel_case_types)]
pub enum RxRfPortSelection {
    /// (RX1A_N &  RX1A_P) and (RX2A_N & RX2A_P) enabled; balanced
    A_BALANCED = 0,
    /// (RX1B_N &  RX1B_P) and (RX2B_N & RX2B_P) enabled; balanced
    B_BALANCED = 1,
    /// (RX1C_N &  RX1C_P) and (RX2C_N & RX2C_P) enabled; balanced
    C_BALANCED = 2,
    /// RX1A_N and RX2A_N enabled; unbalanced
    A_N = 3,
    /// RX1A_P and RX2A_P enabled; unbalanced
    A_P = 4,
    /// RX1B_N and RX2B_N enabled; unbalanced
    B_N = 5,
    /// RX1B_P and RX2B_P enabled; unbalanced
    B_P = 6,
    /// RX1C_N and RX2C_N enabled; unbalanced
    C_N = 7,
    /// RX1C_P and RX2C_P enabled; unbalanced
    C_P = 8,
    /// TX_MONITOR1
    TX_MON1 = 9,
    /// TX_MONITOR2
    TX_MON2 = 10,
    /// TX_MONITOR1 & TX_MONITOR2
    TX_MON1_2 = 11,
}
impl From<u32> for RxRfPortSelection {
    fn from(a: u32) -> Self {
        match a {
            1 => RxRfPortSelection::B_BALANCED,
            2 => RxRfPortSelection::C_BALANCED,
            3 => RxRfPortSelection::A_N,
            4 => RxRfPortSelection::A_P,
            5 => RxRfPortSelection::B_N,
            6 => RxRfPortSelection::B_P,
            7 => RxRfPortSelection::C_N,
            8 => RxRfPortSelection::C_P,
            9 => RxRfPortSelection::TX_MON1,
            10 => RxRfPortSelection::TX_MON2,
            11 => RxRfPortSelection::TX_MON1_2,
            _ => RxRfPortSelection::A_BALANCED,
        }
    }
}
impl From<RxRfPortSelection> for u32 {
    fn from(a: RxRfPortSelection) -> Self {
        a as u32
    }
}

/// Enable State Machine (ENSM) state.
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub enum EnsmState {
    /// Clocks/BB PLL disabled OR Clocks enabled
    SleepOrWait = 0,
    /// Synthesizers enabled
    Alert = 5,
    /// Tx signal chain enabled
    Tx = 6,
    /// Tx digital block flush time
    TxFlush = 7,
    /// Rx signal chain enabled
    Rx = 8,
    /// Rx digital block flush time
    RxFlush = 9,
    /// Tx and Rx signal chains enabled
    Fdd = 10,
    /// Flush all digital signal path blocks
    FddFlush = 11,
    Unknown = 0xFF,
}
impl From<EnsmState> for u8 {
    fn from(state: EnsmState) -> u8 {
        state as u8
    }
}
impl From<u8> for EnsmState {
    fn from(v: u8) -> EnsmState {
        match v {
            0 => EnsmState::SleepOrWait,
            5 => EnsmState::Alert,
            6 => EnsmState::Tx,
            7 => EnsmState::TxFlush,
            8 => EnsmState::Rx,
            9 => EnsmState::RxFlush,
            10 => EnsmState::Fdd,
            11 => EnsmState::FddFlush,
            _ => EnsmState::Unknown,
        }
    }
}

/// Internal / External LO selection
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub enum InternalExternalLO {
    Internal = 0,
    External = 1,
}
impl From<InternalExternalLO> for u8 {
    fn from(lo: InternalExternalLO) -> u8 {
        lo as u8
    }
}

/// Tx Local Oscillator power down
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub enum LOPowerStatus {
    On = 0,
    Off = 1,
}
impl From<LOPowerStatus> for u8 {
    fn from(p: LOPowerStatus) -> u8 {
        p as u8
    }
}
impl From<u8> for LOPowerStatus {
    fn from(v: u8) -> Self {
        match v {
            // This is the opposite sense to the enum values in order to correct
            // for an apparrent error in the C driver
            1 => Self::On,
            0 => Self::Off,
            _ => unreachable!(),
        }
    }
}

/// Built-In Self Test (BIST) mode
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub enum BistMode {
    Disable = 0,
    InjectTx = 1,
    InjectRx = 2,
}
impl Default for bindings::ad9361_bist_mode {
    fn default() -> Self {
        Self::BIST_DISABLE
    }
}
impl From<BistMode> for bindings::ad9361_bist_mode {
    fn from(mode: BistMode) -> bindings::ad9361_bist_mode {
        match mode {
            BistMode::Disable => bindings::ad9361_bist_mode::BIST_DISABLE,
            BistMode::InjectTx => bindings::ad9361_bist_mode::BIST_INJ_TX,
            BistMode::InjectRx => bindings::ad9361_bist_mode::BIST_INJ_RX,
        }
    }
}
impl From<bindings::ad9361_bist_mode> for BistMode {
    fn from(mode: bindings::ad9361_bist_mode) -> BistMode {
        match mode {
            bindings::ad9361_bist_mode::BIST_DISABLE => BistMode::Disable,
            bindings::ad9361_bist_mode::BIST_INJ_TX => BistMode::InjectTx,
            bindings::ad9361_bist_mode::BIST_INJ_RX => BistMode::InjectRx,
        }
    }
}

/// Loopback mode. When enabled, loopback (AD9361 internal) TX->RX
pub enum LoopbackMode {
    Disabled = 0,
    Enabled = 1,
}
impl From<LoopbackMode> for i32 {
    fn from(mode: LoopbackMode) -> i32 {
        mode as i32
    }
}
impl From<i32> for LoopbackMode {
    fn from(v: i32) -> LoopbackMode {
        match v {
            1 => LoopbackMode::Enabled,
            _ => LoopbackMode::Disabled,
        }
    }
}

/// RF Gain Control Mode
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub enum RfGainControlMode {
    Manual = 0,
    FastAttackAgc = 1,
    SlowAttackAgc = 2,
    HybridAgc = 3,
}
impl From<RfGainControlMode> for u8 {
    fn from(mode: RfGainControlMode) -> u8 {
        mode as u8
    }
}
impl From<u8> for RfGainControlMode {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Manual,
            1 => Self::FastAttackAgc,
            2 => Self::SlowAttackAgc,
            3 => Self::HybridAgc,
            _ => unreachable!(),
        }
    }
}

// ---- Internal Types ----------------------

#[repr(transparent)]
pub(crate) struct TemperatureX1000(i32);
impl From<i32> for TemperatureX1000 {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
impl From<TemperatureX1000> for f32 {
    fn from(t: TemperatureX1000) -> f32 {
        (t.0 as f32) / 1000.
    }
}

#[repr(transparent)]
pub(crate) struct InBool(bool);
impl From<u8> for InBool {
    fn from(v: u8) -> Self {
        match v {
            0 => Self(false),
            _ => Self(true),
        }
    }
}
impl From<InBool> for bool {
    fn from(b: InBool) -> bool {
        b.0
    }
}

// ---- implementations bindings -> rust ----------------------

impl From<bindings::rf_rssi> for f32 {
    fn from(rssi: bindings::rf_rssi) -> f32 {
        rssi.symbol as f32 / -100.0 // -0.25dB / LSB, already multiplied by 25
    }
}
