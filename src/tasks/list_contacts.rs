use crate::models::contacts;
use loco_rs::prelude::*;
pub struct ListContacts;
#[async_trait]
impl Task for ListContacts {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "list_contacts".to_string(),
            detail: "List of all contacts".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        let contacts = contacts::Entity::find().all(&app_context.db).await?;

        for (count, contact) in contacts.iter().enumerate() {
            println!("{}. {}", count + 1, contact.email);
        }

        Ok(())
    }
}
