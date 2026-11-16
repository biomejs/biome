pub(crate) use crate::LicenseList;
use crate::license::generated::Licence;

pub mod expression;
pub mod generated;

impl LicenseList {
    /// Returns the license whose SPDX identifier is exactly `license_id`.
    ///
    /// The comparison is case-sensitive, and deprecated identifiers are found too.
    pub(crate) fn find(&self, license_id: &str) -> Option<&'static Licence> {
        self.license_list
            .binary_search_by_key(&license_id, |license| license.license_id)
            .ok()
            .map(|index| self.license_list[index])
    }

    pub fn is_valid(&self, license_id: &str) -> bool {
        self.find(license_id).is_some()
    }

    pub fn is_deprecated(&self, license_id: &str) -> bool {
        self.find(license_id)
            .is_some_and(|license| license.is_deprecated_license_id)
    }

    pub fn is_osi_approved(&self, license_id: &str) -> bool {
        self.find(license_id)
            .is_some_and(|license| license.is_osi_approved)
    }

    pub fn is_fsf_libre(&self, license_id: &str) -> bool {
        self.find(license_id)
            .is_some_and(|license| license.is_fsf_libre)
    }
}

#[cfg(test)]
mod tests {
    use super::generated::LICENSES;

    #[test]
    fn licenses_are_sorted_by_id() {
        for pair in LICENSES.windows(2) {
            assert!(
                pair[0].license_id < pair[1].license_id,
                "{:?} must come before {:?}",
                pair[1].license_id,
                pair[0].license_id
            );
        }
    }
}
