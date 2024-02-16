use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
/// Information about a particular physical network connection.
pub struct RemoteSystemsData {
    /// A TimeFilter for this entry.
    pub time_mark: u32,
    /// Represents an arbitrary local integer value used to identify a remote system.
    pub remote_index: u32,
    /// Identify the chassis associated with the remote system.
    pub chassis_id_subtype: String,
    /// Identify the chassis component associated with the remote system.
    pub chassis_id: String,
    /// The type of port identifier encoding used in the associated 'port-id' object.
    pub port_id_subtype: String,
    /// Port component associated with the remote system.
    pub port_id: String,
    /// Description of the given port associated with the remote system.
    pub port_desc: String,
    /// System name of the remote system.
    pub system_name: String,
    /// System description of the remote system.
    pub system_description: String,
    /// Capabilities that are supported on the remote system.
    pub system_capabilities_supported: String,
    /// System capabilities that are enabled on the remote system.
    pub system_capabilities_enabled: String,
    /// Management address information about a particular chassis component.
    pub management_address: Vec<ManagementAddress>,
    /// Information about an unrecognized TLV received from a physical network connection. Entries may be created and deleted in this table by the agent, if a physical topology discovery process is active.
    pub remote_unknown_tlv: Vec<RemoteUnknownTlv>,
    /// Information about the unrecognized organizationally defined information advertised by the remote system.
    pub remote_org_defined_info: Vec<RemoteOrgDefinedInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
/// Management address information about a particular chassis component.
pub struct ManagementAddress {
    /// Management address identifier encoding.
    pub address_subtype: String,
    /// Management address component associated with the remote system.
    pub address: String,
    /// Interface numbering method used for defining the interface number, associated with the remote system.
    pub if_subtype: String,
    /// Interface number regarding the management address component associated with the remote system.
    pub if_id: u32,
}

#[derive(Serialize, Deserialize, Clone)]
/// Information about an unrecognized TLV received from a physical network connection. Entries may be created and deleted in this table by the agent, if a physical topology discovery process is active.
pub struct RemoteUnknownTlv {
    /// Type of TLV.
    pub tlv_type: u32,
    /// Value extracted from TLV.
    pub tlv_info: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
/// Information about the unrecognized organizationally defined information advertised by the remote system.
pub struct RemoteOrgDefinedInfo {
    /// The Organizationally Unique Identifier (OUI) or Company ID (CID).
    pub info_identifier: u32,
    /// The subtype of the organizationally defined information received from the remote system.
    pub info_subtype: u32,
    /// Arbitrary local integer value.
    pub info_index: u32,
    /// The organizationally defined information of the remote system.
    pub remote_info: Vec<u8>,
}
