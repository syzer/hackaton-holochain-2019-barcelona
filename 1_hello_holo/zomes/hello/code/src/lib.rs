#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;
extern crate rand; // add rand package to generate random numbers

use rand::Rng;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};

use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError,
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;



#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Message {
  content: String
}


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
    Ok(format!("yyour random number: {}", random_float).into())
  }

  #[entry_def]
  fn message_entry_def() -> ValidatingEntryType {
    entry!(
      name: "message",
      description: "Message to propagate",
      sharing: Sharing::Public,
      validation_package: || {
        hdk::ValidationPackageDefinition::Entry
      },
      validation: | _validation_data: hdk::EntryValidationData<Message>| {
        Ok(())
      }
    )
  }

  #[zome_fn("hc_public")]
  pub fn create_message(message: Message) -> ZomeApiResult<Address> {
    let entry = Entry::App("message".into(), message.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
    // link to anchor
  }

  #[zome_fn("hc_public")]
  fn retrieve_message(address: Address) -> ZomeApiResult<Message> {
    hdk::utils::get_as_type(address)
  }
}
