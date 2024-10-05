use keyring::{credential, default, Entry, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("KEYCHAINS");

    let persistence = default::default_credential_builder().persistence();
    if matches!(persistence, credential::CredentialPersistence::UntilDelete) {
        println!("The default credential builder persists credentials on disk!")
    } else {
        println!("The default credential builder doesn't persist credentials on disk!")
    }

    let entry = Entry::new("my-service", "jmsdnns")?;

    let attrs = entry.get_attributes();
    println!("ATTRS ::\n{:?}", attrs);

    let creds = entry.get_credential();
    println!("CREDS ::\n{:?}", creds);

    let hm = HashMap::from([("cats", "meow")]);
    let _ = entry.update_attributes(&hm);
    let attrs = entry.get_attributes();
    println!("ATTRS ::\n{:?}", attrs);

    entry.set_password("topS3cr3tP4$$w0rd")?;
    let password = entry.get_password()?;
    println!("My password is '{}'", password);

    //entry.delete_credential()?;
    Ok(())
}
