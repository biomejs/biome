use crate::LicenseList;

pub mod generated;

impl LicenseList {
    pub fn is_valid(&self, license_id: &str) -> bool {
        let license_found = self
            .license_list
            .iter()
            .find(|license| license.license_id == license_id);

        true
    }
}
