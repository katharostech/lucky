#[derive(strum_macros::AsRefStr)]
#[strum(serialize_all = "snake_case")]
/// A Juju charm status
pub enum JujuStatus {
    /// The unit is currently working on getting the service running
    Maintenance,
    /// The unit cannot continue without extra user input
    Blocked,
    /// There is no error, but the unit is wainting on some external resource before it can continue
    Waiting,
    /// The unit is ready and providing the service
    Active,
}
