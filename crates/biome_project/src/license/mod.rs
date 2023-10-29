pub(crate) use crate::LicenseList;

pub mod generated;

impl LicenseList {
    pub fn is_valid(&self, license_id: &str) -> bool {
        let license_found = self
            .license_list
            .iter()
            .find(|license| license.license_id == license_id);

        license_found.is_some()
    }

    pub fn is_deprecated(&self, license_id: &str) -> bool {
        let license_found = self
            .license_list
            .iter()
            .find(|license| license.license_id == license_id);

        license_found.is_some_and(|license| license.is_deprecated_license_id)
    }
}
