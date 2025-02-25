use proc_macro2::{Literal, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use std::io;
use ureq::get;
use xtask::*;
use xtask::{project_root, Mode};
use xtask_codegen::update;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LicenseList {
    license_list_version: String,
    licenses: Vec<Licence>,
    release_date: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Licence {
    reference: String,
    is_deprecated_license_id: bool,
    details_url: String,
    reference_number: u16,
    name: String,
    license_id: String,
    see_also: Vec<String>,
    is_osi_approved: Option<bool>,
    is_fsf_libre: Option<bool>,
}

const URL: &str =
    "https://raw.githubusercontent.com/spdx/license-list-data/master/json/licenses.json";
pub(crate) fn generate_license(mode: Mode) -> Result<()> {
    let request = get(URL);
    let result = request.call()?;
    let license_list = result.into_json::<LicenseList>()?;
    let config_root = project_root().join("crates/biome_package/src/license");

    let tokens = create_data(license_list).expect("To write data into file");

    update(
        &config_root.join("generated.rs"),
        &xtask::reformat(tokens.to_string())?,
        &mode,
    )?;

    Ok(())
}

fn create_data(license_list: LicenseList) -> io::Result<TokenStream> {
    let mut list = vec![];

    let version = Literal::string(&license_list.license_list_version);
    let release_date = Literal::string(&license_list.release_date);

    for item in license_list.licenses {
        let reference = Literal::string(&item.reference);
        let details_url = Literal::string(&item.details_url);
        let name = Literal::string(&item.name);
        let license_id = Literal::string(&item.license_id);
        let is_deprecated_license_id = item.is_deprecated_license_id;
        let is_osi_approved = item.is_osi_approved.unwrap_or_default();
        let is_fsf_libre = item.is_fsf_libre.unwrap_or_default();
        let see_also: Vec<_> = item
            .see_also
            .iter()
            .map(|see_also| {
                let see_also = Literal::string(see_also);
                quote!(#see_also)
            })
            .collect();
        list.push(quote! {

             &Licence {
                reference: #reference,
                is_deprecated_license_id: #is_deprecated_license_id,
                details_url: #details_url,
                reference_number: 0,
                name: #name,
                license_id: #license_id,
                see_also: &[
                    #( #see_also ),*
                ],
                is_osi_approved: #is_osi_approved,
                is_fsf_libre: #is_fsf_libre,
            }
        });
    }

    let tokens = quote! {

        #[derive(Debug)]
        pub struct LicenseList {
            #[expect(unused)]
            pub(crate) license_list_version: &'static str,
            pub(crate) license_list: &'static [&'static Licence],
            #[expect(unused)]
            pub(crate) release_date: &'static str,
        }

        #[derive(Debug)]
        pub struct Licence {
            #[expect(unused)]
            pub(crate) reference: &'static str,
            pub(crate) is_deprecated_license_id: bool,
            #[expect(unused)]
            pub(crate) details_url: &'static str,
            #[expect(unused)]
            pub(crate) reference_number: u16,
            #[expect(unused)]
            pub(crate) name: &'static str,
            pub(crate) license_id: &'static str,
            #[expect(unused)]
            pub(crate) see_also: &'static [&'static str],
            #[expect(unused)]
            pub(crate) is_osi_approved: bool,
            #[expect(unused)]
            pub(crate) is_fsf_libre: bool,
        }


        pub const LICENSE_LIST: LicenseList = LicenseList {
            license_list_version: #version,
            license_list: LICENSES,
            release_date: #release_date,
        };

        pub const LICENSES: &[&Licence] = &[
            #( #list ),*
        ];


    };

    Ok(tokens)
}
