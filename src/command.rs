use crate::prelude::*;

pub mod data;
pub mod help;
pub mod ping;

macro_rules! getter {
    ($id:ident($inner:path) -> $output:ty) => {
        #[allow(dead_code)]
        pub fn $id<'c>(options: &'c [ResolvedOption<'c>], name: &'c str) -> Result<$output> {
            let resolved = options.iter().find(|r| r.name == name).map_or_else(
                || Err(anyhow!("missing data for \"{name}\"")),
                |resolved| Ok(&resolved.value),
            )?;

            match resolved {
                $inner(v) => Ok(*v),
                _ => Err(anyhow!("invalid data type for \"{name}\"")),
            }
        }
    };
    ($id:ident($inner:path) -> ref $output:ty) => {
        #[allow(dead_code)]
        pub fn $id<'c>(options: &'c [ResolvedOption<'c>], name: &'c str) -> Result<&'c $output> {
            let resolved = options.iter().find(|r| r.name == name).map_or_else(
                || Err(anyhow!("missing data for \"{name}\"")),
                |resolved| Ok(&resolved.value),
            )?;

            match resolved {
                $inner(v) => Ok(v),
                _ => Err(anyhow!("invalid data type for \"{name}\"")),
            }
        }
    };
}

getter!(get_bool(ResolvedValue::Boolean) -> bool);
getter!(get_i64(ResolvedValue::Integer) -> i64);
getter!(get_f64(ResolvedValue::Number) -> f64);
getter!(get_partial_channel(ResolvedValue::Channel) -> ref PartialChannel);
getter!(get_role(ResolvedValue::Role) -> ref Role);
getter!(get_str(ResolvedValue::String) -> ref str);
getter!(get_subcommand(ResolvedValue::SubCommand) -> ref [ResolvedOption<'c>]);
getter!(get_subcommand_group(ResolvedValue::SubCommandGroup) -> ref [ResolvedOption<'c>]);

#[allow(dead_code)]
pub fn get_user<'c>(
    options: &'c [ResolvedOption<'c>],
    name: &'c str,
) -> Result<(&'c User, Option<&'c PartialMember>)> {
    let resolved = options.iter().find(|r| r.name == name).map_or_else(
        || Err(anyhow!("missing data for \"{name}\"")),
        |resolved| Ok(&resolved.value),
    )?;

    match resolved {
        ResolvedValue::User(user, member) => Ok((user, *member)),
        _ => Err(anyhow!("invalid data type for \"{name}\"")),
    }
}

pub fn get_input_text<'c>(options: &'c [ActionRow], name: &'c str) -> Result<&'c str> {
    for row in options {
        let Some(ActionRowComponent::InputText(input)) = row.components.first() else {
            continue;
        };

        if input.custom_id == name && !input.value.is_empty() {
            return Ok(&input.value);
        }
    }

    Err(anyhow!("missing data for \"{name}\""))
}
