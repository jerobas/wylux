use uuid::{Uuid, Version};
use crate::config::schema::ResolvedConfig;

/// PortaQEMU namespace UUID (v5 namespace)
const PORTAQEMU_NAMESPACE: Uuid = Uuid::from_u128(0x6ba7b8109dad11d180b400c04fd430c8);

/// Generate stable GUID for a VM profile.
pub fn generate_profile_guid(config: &ResolvedConfig) -> Uuid {
    Uuid::new_v5(&PORTAQEMU_NAMESPACE, config.vm.name.as_bytes())
}
