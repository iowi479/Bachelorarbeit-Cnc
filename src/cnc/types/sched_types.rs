use serde::{Deserialize, Serialize};

/// A GateControlEntry consists of an operation name, followed by up to 2
/// parameters associated with the operation. The first parameter is a
/// gateStatesValue; the second parameter is a timeIntervalValue
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GateControlEntry {
    pub operation_name: GateControlOperation,

    pub time_interval_value: u32,

    /// gateStatesValue is the gate states for this entry for the Port.
    /// The gates are immediately set to the states in gateStatesValue
    /// when this entry executes. The bits of the octet represent the
    /// gate states for the corresponding traffic classes; the
    /// most-significant bit corresponds to traffic class 7, the
    /// least-significant bit to traffic class 0. A bit value of 0
    /// indicates closed; a bit value of 1 indicates open.
    pub gate_state_value: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GateControlOperation {
    /// Operation to set the gate states.
    SetGateStates,

    /// Operation to set and hold MAC.
    SetAndHoldMAC,

    /// Operation to set and release MAC.
    SetAndReleaseMAC,
}

/// A list containing a set of max SDU parameters, one for each
/// traffic class. All writable objects in this table must be
/// persistent over power up restart/reboot.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueueMaxSduEntry {
    /// Traffic class
    traffic_class: u8,

    /// The value of the queueMaxSDU parameter for the traffic class. A
    /// value of 0 is interpreted as the max SDU size supported by the
    /// underlying MAC. The value must be retained across
    /// reinitializations of the management system.
    queue_max_sdu: u32,

    /// A counter of transmission overrun events, where a PDU is still
    /// being transmitted by a MAC at the time when the transmission
    /// gate for the queue closed.
    transmission_overrun: u64,
}

pub type PtpTimeScale = u32;
pub type RationalGrouping = (i32, i32);

///A table that contains the per-port manageable parameters for
/// traffic scheduling. For a given Port, an entry in the table exists.
/// All writable objects in this table must be persistent over power up
/// restart/reboot.
pub type SchedParameters = Vec<GateParameterTableEntry>;

/// A table that contains the per-port manageable parameters for
/// traffic scheduling. For a given Port, an entry in the table exists.
/// All writable objects in this table must be persistent over power up
/// restart/reboot.
#[allow(dead_code)]
pub struct GateParameterTableEntry {
    /// A list containing a set of max SDU parameters, one for each
    /// traffic class. All writable objects in this table must be
    /// persistent over power up restart/reboot.
    queue_max_sdu_table: Vec<QueueMaxSduEntry>,

    /// The GateEnabled parameter determines whether traffic scheduling
    /// is active (true) or inactive (false). The value must be retained
    /// across reinitializations of the management system.
    gate_enable: bool,

    /// AdminGateStates is the administrative value of the initial gate
    /// states for the Port. The bits of the octet represent the gate
    /// states for the corresponding traffic classes; the most-significant
    /// bit corresponds to traffic class 7, the least-significant bit to
    /// traffic class 0. A bit value of 0 indicates closed; a bit value of
    /// 1 indicates open. The value must be retained across
    /// reinitializations of the management system.
    admin_gate_states: u8,

    /// OperGateStates is the operational value of the current gate
    /// states for the Port. The bits of the octet represent the gate
    /// states for the corresponding traffic classes; the most-significant
    /// bit corresponds to traffic class 7, the least-significant bit to
    /// traffic class 0. A bit value of 0 indicates closed; a bit value of
    /// 1 indicates open.
    oper_gate_states: u8,

    /// AdminControlList is the administrative value of the gate control
    /// list for the Port. The value must be retained across
    /// reinitializations of the management system.
    admin_control_list: Vec<GateControlEntry>,

    /// OperControlList is the operational value of the gate control list
    /// for the Port.
    oper_control_list: Vec<GateControlEntry>,

    /// AdminCycleTime specifies the administrative value of the gating
    /// cycle time for the Port. AdminCycleTime is a rational number of
    /// seconds, defined by an integer numerator and an integer
    /// denominator. The value must be retained across reinitializations
    /// of the management system.
    admin_cycle_time: RationalGrouping,

    /// OperCycleTime specifies the operational value of the gating cycle
    /// time for the Port. OperCycleTime is a rational number of seconds,
    /// defined by an integer numerator and an integer denominator.
    oper_cycle_time: RationalGrouping,

    /// An unsigned integer number of nanoseconds, defining the maximum
    /// amount of time by which the gating cycle for the Port is permitted
    /// to be extended when a new cycle configuration is being installed.
    /// This is the administrative value. The value must be retained
    /// across reinitializations of the management system.
    admin_cycle_time_extension: u32,

    /// An unsigned integer number of nanoseconds, defining the maximum
    /// amount of time by which the gating cycle for the Port is permitted
    /// to be extended when a new cycle configuration is being installed.
    /// This is the operational value.
    oper_cycle_time_extension: u32,

    /// The administrative value of the base time at which gating cycles
    /// begin, expressed as an IEEE 1588 precision time protocol (PTP)
    /// timescale. The value must be retained across reinitializations of
    /// the management system.
    admin_base_time: PtpTimeScale,

    /// The operational value of the base time at which gating cycles
    /// begin, expressed as an IEEE 1588 precision time protocol (PTP)
    /// timescale.
    oper_base_time: PtpTimeScale,

    /// The ConfigChange parameter signals the start of a configuration
    /// change when it is set to TRUE, indicating that the administrative
    /// parameters for the Port are ready to be copied into their
    /// corresponding operational parameters. This should only be done
    /// when the various administrative parameters are all set to
    /// appropriate values.
    config_change: bool,

    /// The time at which the next config change is scheduled to occur.
    config_time_change_time: PtpTimeScale,

    /// The granularity of the cycle time clock, represented as an
    /// unsigned number of tenths of nanoseconds. The value must be
    /// retained across reinitializations of the management system.
    tick_granularity: u32,

    /// The current time as maintained by the local system.
    current_time: PtpTimeScale,

    /// The value of the ConfigPending state machine variable. The value
    /// is TRUE if a configuration change is in progress but has not yet
    /// completed.
    config_pending: bool,

    /// A counter of the number of times that a re-configuration of the
    /// traffic schedule has been requested with the old schedule still
    /// running and the requested base time was in the past.
    config_change_error: u64,

    /// The maximum value supported by this Port for the
    /// AdminControlListLength and OperControlListLength parameters. It is
    /// available for use by schedule computation software to determine
    /// the port’s control list capacity prior to computation. The object
    /// may optionally be read-only.
    supported_list_max: u32,

    /// The maximum value supported by this Port of the AdminCycleTime
    /// and OperCycleTime parameters. The object may optionally be
    /// read-only.
    supported_cycle_max: RationalGrouping,

    /// The maximum value supported by this Port of the TimeIntervalValue
    /// parameter. The object may optionally be read-only.
    supported_interval_max: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfigurableGateParameterTableEntry {
    // YANG -> The value must be retained across reinitializations of the management system.
    // queue_max_sdu_table: Vec<QueueMaxSduEntry>,
    pub gate_enable: bool,
    pub admin_gate_states: u8,
    pub admin_control_list: Vec<GateControlEntry>,
    pub admin_cycle_time: RationalGrouping,
    pub admin_cycle_time_extension: u32,
    pub admin_base_time: PtpTimeScale,

    // must not be retained... This applies the config?
    pub config_change: bool,
}
