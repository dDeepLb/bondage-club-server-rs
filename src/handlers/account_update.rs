use mongodb::{
    Collection,
    bson::{self, Bson, Document, doc},
};
use socketioxide::extract::SocketRef;

use crate::{
    common::protocol::{Account, AccountUpdateRequest},
    server::BCServer,
};

impl BCServer {
    pub async fn on_account_update(&self, socket: SocketRef, request: AccountUpdateRequest) {
        let mut update: Document = doc! {};

        let mut accounts = self.accounts.lock().await;
        let account = accounts
            .iter_mut()
            .find(|a| a.id == Some(socket.id.to_string()));
        if account.is_none() {
            return;
        }
        let account = account.unwrap();
        if let Some(log) = request.log.clone() {
            update.insert("Log", bson::to_bson(&log).unwrap());
            account.log = Some(log);
        }
        if let Some(inventory_data) = request.inventory_data {
            update.insert("InventoryData", bson::to_bson(&inventory_data).unwrap());
            account.inventory_data = Some(inventory_data);
        }
        if let Some(item_permission) = request.item_permission {
            account.item_permission = item_permission;
            update.insert("ItemPermission", Bson::from(item_permission as i32));
        }
        if let Some(arousal_settings) = request.arousal_settings {
            update.insert("ArousalSettings", bson::to_bson(&arousal_settings).unwrap());
            account.arousal_settings = Some(arousal_settings);
        }
        if let Some(online_shared_settings) = request.online_shared_settings {
            update.insert(
                "OnlineSharedSettings",
                bson::to_bson(&online_shared_settings).unwrap(),
            );
            account.online_shared_settings = Some(online_shared_settings);
        }
        if let Some(game) = request.game.clone() {
            update.insert("Game", bson::to_bson(&game).unwrap());
            account.game = Some(game);
        }
        if let Some(map_data) = request.map_data {
            account.map_data = Some(map_data);
        }
        if let Some(label_color) = request.label_color {
            update.insert("LabelColor", bson::to_bson(&label_color).unwrap());
            account.label_color = Some(label_color);
        }
        if let Some(appearance) = request.appearance.clone() {
            update.insert("Appearance", bson::to_bson(&appearance).unwrap());
            account.appearance = Some(appearance);
        }
        if let Some(reputation) = request.reputation {
            update.insert("Reputation", bson::to_bson(&reputation).unwrap());
            account.reputation = Some(reputation);
        }
        if let Some(description) = request.description {
            update.insert("Description", bson::to_bson(&description).unwrap());
            account.description = Some(description);
        }
        if let Some(block_items) = request.block_items {
            update.insert("BlockItems", bson::to_bson(&block_items).unwrap());
            account.block_items = Some(block_items);
        }
        if let Some(limited_items) = request.limited_items {
            update.insert("LimitedItems", bson::to_bson(&limited_items).unwrap());
            account.limited_items = Some(limited_items);
        }
        if let Some(favorite_items) = request.favorite_items {
            update.insert("FavoriteItems", bson::to_bson(&favorite_items).unwrap());
            account.favorite_items = Some(favorite_items);
        }
        if let Some(white_list) = request.white_list {
            update.insert("WhiteList", bson::to_bson(&white_list).unwrap());
            account.white_list = white_list
        }
        if let Some(black_list) = request.black_list {
            update.insert("BlackList", bson::to_bson(&black_list).unwrap());
            account.black_list = black_list;
        }
        if let Some(friend_list) = request.friend_list {
            update.insert("FriendList", bson::to_bson(&friend_list).unwrap());
            account.friend_list = friend_list;
        }
        // TODO: Lovership
        if let Some(skill) = request.skill.clone() {
            update.insert("Skill", bson::to_bson(&skill).unwrap());
            account.skill = Some(skill);
        }
        if let Some(title) = request.title {
            update.insert("Title", bson::to_bson(&title).unwrap());
            account.title = Some(title);
        }
        if let Some(nickname) = request.nickname {
            update.insert("Nickname", bson::to_bson(&nickname).unwrap());
            account.nickname = Some(nickname);
        }
        if let Some(crafting) = request.crafting {
            update.insert("Crafting", bson::to_bson(&crafting).unwrap());
            account.crafting = Some(crafting);
        }

        // Some changes should be synched to other players in chatroom
        if let Some(_chat_room) = request.chat_room {
            // TODO: Chatroom
        }

        // If only the appearance is updated, we keep the change in memory and do not update the database right away
        /*     if let Some(appearance) = parsed.appearance.clone() {
            update.insert("DelayedSkillUpdate", bson::to_bson(&appearance).unwrap());
            account.delayed_appearance_update = Some(appearance).clone();
            //console.log("TO REMOVE - Keeping Appearance in memory for account: " + Acc.AccountName);
            return;
        }

        // If only the skill is updated, we keep the change in memory and do not update the database right away
        if let Some(skill) = parsed.skill {
            update.insert("DelayedSkillUpdate", bson::to_bson(&skill).unwrap());
            account.delayed_skill_update = Some(skill).clone();
            //console.log("TO REMOVE - Keeping Skill in memory for account: " + Acc.AccountName);
            return;
        }

        // If only the game is updated, we keep the change in memory and do not update the database right away
        if let Some(game) = parsed.game {
            update.insert("DelayedGameUpdate", bson::to_bson(&game).unwrap());
            account.delayed_game_update = Some(game).clone();
            //console.log("TO REMOVE - Keeping Game in memory for account: " + Acc.AccountName);
            return;
        } */

        // Removes the delayed data to update if we update that property right now
        /*   if let Some(appearance) = parsed.appearance.clone() {
        if let Some(delayed_appearance_update) = account.delayed_appearance_update.clone() {
            if delayed_appearance_update == appearance {
                account.delayed_appearance_update = None;
            }
        } */

        /*  if parsed.appearance.is_some() && account.delayed_appearance_update.is_some() {
                update.remove("DelayedAppearanceUpdate");
            }
            if parsed.skill.is_some() && account.delayed_skill_update.is_some() {
                update.remove("DelayedSkillUpdate");
            }
            if parsed.game.is_some() && account.delayed_game_update.is_some() {
                update.remove("DelayedGameUpdate");
            }
        }*/
        let users: Collection<Account> = self.db.collection(&self.config.db_accounts);
        let _ = users
            .update_one(
                doc! { "AccountName": &account.account_name },
                doc! { "$set": update },
                None,
            )
            .await;
    }
}
