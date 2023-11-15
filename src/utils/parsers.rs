use serenity::model::prelude::{Guild, ChannelCategory};

pub fn parse_category_from_name(guild: &Guild, name: String) -> Option<ChannelCategory> {
    for (_channel_id, channel) in &guild.channels {
        if let Some(category) = channel.clone().category() {
            if category.name.to_lowercase().starts_with(&name) {
                return Some(category)
            }
        }
    }

    None
}
