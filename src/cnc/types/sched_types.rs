use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct GateControlEntry {
    pub operation_name: GateControlOperation,
    pub time_interval_value: u32,
    pub gate_state_value: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GateControlOperation {
    SetGateStates,
    SetAndHoldMAC,
    SetAndReleaseMAC,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueueMaxSduEntry {
    traffic_class: u8, // TODO u8 type? supported traffic classes up to 8
    queue_max_sdu: u32,
    transmission_overrun: u64,
}

pub type PtpTimeScale = u32;
pub type RationalGrouping = (i32, i32);

pub type SchedParameters = Vec<GateParameterTableEntry>;

pub struct GateParameterTableEntry {
    queue_max_sdu_table: Vec<QueueMaxSduEntry>,
    gate_enable: bool,
    admin_gate_states: u8, // all 8 gates coded into bit representation
    oper_gate_states: u8,  // all 8 gates coded into bit representation
    admin_control_list: Vec<GateControlEntry>,
    oper_control_list: Vec<GateControlEntry>,
    admin_cycle_time: RationalGrouping,
    oper_cycle_time: RationalGrouping,
    admin_cycle_time_extension: u32,
    oper_cycle_time_extension: u32,
    admin_base_time: PtpTimeScale,
    oper_base_time: PtpTimeScale,
    config_change: bool,
    config_time_change_time: PtpTimeScale,
    tick_granularity: u32,
    current_time: PtpTimeScale,
    config_pending: bool,
    config_change_error: u64,
    supported_list_max: u32,
    supported_cycle_max: RationalGrouping,
    supported_interval_max: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigurableGateParameterTableEntry {
    // YANG -> The value must be retained across reinitializations of the management system.
    // queue_max_sdu_table: Vec<QueueMaxSduEntry>,
    pub gate_enable: bool,
    pub admin_gate_states: u8, // all 8 gates coded into bit representation
    pub admin_control_list: Vec<GateControlEntry>,
    pub admin_cycle_time: RationalGrouping,
    pub admin_cycle_time_extension: u32,
    pub admin_base_time: PtpTimeScale,

    // must not be retained... This applies the config?
    pub config_change: bool,
}
