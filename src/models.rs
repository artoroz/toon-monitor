#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Toon {
    #[serde(rename = "dev_3")]
    pub dev3: Dev3,
    #[serde(rename = "dev_3.1")]
    pub dev31: Dev31,
    #[serde(rename = "dev_3.2")]
    pub dev32: Dev32,
    #[serde(rename = "dev_3.3")]
    pub dev33: Dev33,
    #[serde(rename = "dev_3.4")]
    pub dev34: Dev34,
    #[serde(rename = "dev_3.5")]
    pub dev35: Dev35,
    #[serde(rename = "dev_3.6")]
    pub dev36: Dev36,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Dev3 {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "internalAddress")]
    pub internal_address: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "IsConnected")]
    pub is_connected: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "CurrentSensorStatus")]
    pub current_sensor_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Dev31 {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "internalAddress")]
    pub internal_address: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "CurrentGasFlow")]
    pub current_gas_flow: String,
    #[serde(rename = "CurrentGasQuantity")]
    pub current_gas_quantity: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "CurrentSensorStatus")]
    pub current_sensor_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Dev32 {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "internalAddress")]
    pub internal_address: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "CurrentElectricityFlow")]
    pub current_electricity_flow: String,
    #[serde(rename = "CurrentElectricityQuantity")]
    pub current_electricity_quantity: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "CurrentSensorStatus")]
    pub current_sensor_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Dev33 {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "internalAddress")]
    pub internal_address: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "CurrentElectricityFlow")]
    pub current_electricity_flow: String,
    #[serde(rename = "CurrentElectricityQuantity")]
    pub current_electricity_quantity: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "CurrentSensorStatus")]
    pub current_sensor_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Dev34 {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "internalAddress")]
    pub internal_address: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "CurrentElectricityFlow")]
    pub current_electricity_flow: String,
    #[serde(rename = "CurrentElectricityQuantity")]
    pub current_electricity_quantity: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "CurrentSensorStatus")]
    pub current_sensor_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Dev35 {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "internalAddress")]
    pub internal_address: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "CurrentElectricityFlow")]
    pub current_electricity_flow: String,
    #[serde(rename = "CurrentElectricityQuantity")]
    pub current_electricity_quantity: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "CurrentSensorStatus")]
    pub current_sensor_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Dev36 {
    pub uuid: String,
    pub name: String,
    #[serde(rename = "internalAddress")]
    pub internal_address: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "CurrentElectricityFlow")]
    pub current_electricity_flow: String,
    #[serde(rename = "CurrentElectricityQuantity")]
    pub current_electricity_quantity: String,
    #[serde(rename = "DeviceName")]
    pub device_name: String,
    #[serde(rename = "CurrentSensorStatus")]
    pub current_sensor_status: String,
}