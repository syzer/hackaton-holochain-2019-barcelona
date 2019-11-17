#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;
extern crate rand; // add rand package to generate random numbers

use rand::Rng;

use hdk::{
    error::ZomeApiResult,
};

use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.38-alpha14/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[zome]
mod hello_zome {

  #[init]
  fn init() {
    Ok(())
  }

  #[validate_agent]
  pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
    Ok(())
  }

  #[zome_fn("hc_public")]
  fn hello_holo(name: String) -> ZomeApiResult<String> {
    Ok(format!("Hello {}!", name).into())
  }

  #[zome_fn("hc_public")]
  fn generate_rand() -> ZomeApiResult<String> {
    let mut rng = rand::thread_rng();
    let random_float = rng.gen::<f64>();
    Ok(format!("your random number: {}", random_float).into())
  }

  // #[entry_def]
  // fn person_entry_def() -> ValidatingEntryType {
  //   entry!(
  //     name: "person",
  //     description: "Person to say hello to",
  //     sharing: Sharing::Private,
  //     validation_package: || {
  //       hdk::ValidationPackageDefinition::Entry
  //     },
  //     validation: | _validation_data: hdk::EntryValidationData<Person>| {
  //       Ok(())
  //     }
  //   )
  // }
}
