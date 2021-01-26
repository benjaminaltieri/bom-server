extern crate anyhow;
extern crate clap;
extern crate reqwest;
extern crate url;

use clap::Clap;
use serde_json::to_string_pretty;
use url::Url;
use uuid::Uuid;

use bom_server::client;
use bom_server::parts_list::{PartsListFilter, PartsListUpdate};

#[derive(Clap, PartialEq, Debug)]
#[clap(rename_all = "screaming_snake")]
pub enum FilterOption {
    All,
    TopLevel,
    Assembly,
    Component,
    Subassembly,
    Orphan
}

/// Convert from structopt cli filter repr to internal filter type
impl From<FilterOption> for PartsListFilter { 
    fn from(filter: FilterOption) -> PartsListFilter {
        match filter {
            FilterOption::All => PartsListFilter::All,
            FilterOption::TopLevel => PartsListFilter::TopLevel,
            FilterOption::Assembly => PartsListFilter::Assembly,
            FilterOption::Component => PartsListFilter::Component,
            FilterOption::Subassembly => PartsListFilter::Subassembly,
            FilterOption::Orphan => PartsListFilter::Orphan
        }
    }
}

#[derive(Clap, PartialEq, Debug)]
#[clap(rename_all = "screaming_snake")]
pub enum ActionOption {
    Add,
    Remove,
    Replace,
}

/// Convert from structopt cli filter repr to internal filter type
impl From<ActionOption> for PartsListUpdate { 
    fn from(filter: ActionOption) -> PartsListUpdate {
        match filter {
            ActionOption::Add => PartsListUpdate::Add,
            ActionOption::Remove => PartsListUpdate::Remove,
            ActionOption::Replace => PartsListUpdate::Replace,
        }
    }
}

/// A simple client to test BOM-Server
#[derive(Clap)]
#[clap(version = "0.1.0")]
pub struct Opts {
    /// A level of verbosity, can be used multiple times
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,
    #[clap(short, long, default_value = "http://localhost:8000")]
    pub host: String,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    GetIndex(GetIndex),
    ListParts(ListParts),
    CreatePart(CreatePart),
    GetPart(GetPart),
    DeletePart(DeletePart),
    GetChildren(GetChildren),
    UpdatePart(UpdatePart),
    GetContained(GetContained),
}

/// Return text from BOM Server root
#[derive(Clap)]
pub struct GetIndex {
}

/// List parts from BOM Server
#[derive(Clap)]
pub struct ListParts {
    /// Filter for listing parts matching the variants listed
    #[clap(short, long, default_value = "ALL", arg_enum, case_insensitive(true))]
    pub filter: FilterOption,
}

/// Creates a part in the BOM Server
#[derive(Clap)]
pub struct CreatePart {
    /// The name of the new part, used for uniqueness
    #[clap(short, long)]
    pub name: String,
}

/// Retrieves a part from the BOM Server
#[derive(Clap)]
pub struct GetPart {
    /// Part id to retrieve, assigned during creation
    #[clap(short, long)]
    pub id: Uuid,
}

/// Deletes a part from the BOM Server
#[derive(Clap)]
pub struct DeletePart {
    /// Part id to delete, assigned during creation
    #[clap(short, long)]
    pub id: Uuid,
}

/// Retrieves a part from the BOM Server
#[derive(Clap)]
pub struct GetChildren {
    /// Part id to retrieve children from
    #[clap(short, long)]
    pub id: Uuid,

    /// Filter for listing children matching the variants listed
    #[clap(short, long, default_value = "ALL", arg_enum, case_insensitive(true))]
    pub filter: FilterOption,
}

/// Updates children of part in the BOM Server
#[derive(Clap)]
pub struct UpdatePart {
    /// Id of part to update
    #[clap(short, long)]
    pub id: Uuid,

    /// The ids of the parts to add/remove/replace
    #[clap(short, long)]
    pub children: Vec<Uuid>,

    /// Action for updating the children of a part
    #[clap(short, long, default_value = "ADD", arg_enum, case_insensitive(true))]
    pub action: ActionOption,
}

/// Finds all assemblies which contain a part
#[derive(Clap)]
pub struct GetContained {
    /// Part id to retrieve contained assemblies from
    #[clap(short, long)]
    pub id: Uuid,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    let base_url = Url::parse(&opts.host)?;
    let context = client::ClientContext::new(base_url);
    match opts.subcmd {
        SubCommand::GetIndex(_) => {
            let response = client::get_index(&context).await?;
            println!("{}", response);
            Ok(())
        }
        SubCommand::ListParts(subopts) => {
            let response = client::list_parts(&context, subopts.filter.into()).await?;
            println!("{}", to_string_pretty(&response)?);
            Ok(())
        }
        SubCommand::CreatePart(subopts) => {
            let response = client::create_part(&context, &subopts.name).await?;
            println!("{}", to_string_pretty(&response)?);
            Ok(())
        }
        SubCommand::GetPart(subopts) => {
            let response = client::get_part(&context, &subopts.id).await?;
            println!("{}", to_string_pretty(&response)?);
            Ok(())
        }
        SubCommand::DeletePart(subopts) => {
            let response = client::delete_part(&context, &subopts.id).await?;
            println!("{}", to_string_pretty(&response)?);
            Ok(())
        }
        SubCommand::GetChildren(subopts) => {
            let response = client::get_children(&context, &subopts.id, subopts.filter.into()).await?;
            println!("{}", to_string_pretty(&response)?);
            Ok(())
        }
        SubCommand::UpdatePart(subopts) => {
            let response = client::update_part(&context, &subopts.id, &subopts.children, subopts.action.into()).await?;
            println!("{}", to_string_pretty(&response)?);
            Ok(())
        }
        SubCommand::GetContained(subopts) => {
            let response = client::get_children(&context, &subopts.id, PartsListFilter::Assembly).await?;
            println!("{}", to_string_pretty(&response)?);
            Ok(())
        }
    }
}
