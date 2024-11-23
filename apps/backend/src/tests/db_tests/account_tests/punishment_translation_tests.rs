#[cfg(test)]
mod tests {
    use crate::{db::{account::{punishment::insert_punishment, punishment_translation::{delete_punishment_translation, get_all_punishment_translations, get_punishment_translation_by_pk, insert_punishment_translation, update_punishment_translation}}, public::insert_i18n}, tests::db_tests::utils::{cleanup_isolated_db, setup_isolated_db}};

    #[tokio::test]
    async fn test_punishment_translation_crud_operations() {
        let (pool, db_name) = setup_isolated_db().await;

        // Create a new i18n to reference
        let i18n = insert_i18n(
            &pool,
            "English",
            "en",
        )
            .await
            .expect("failed to create reference i18n");

        // Create a new punishment to reference
        let punishment = insert_punishment(
            &pool,
            "ban",
            3600,
        )
            .await
            .expect("failed to create reference punishment");

        // Test Create
        let before_create = chrono::Utc::now().naive_utc();
        let new_translation = insert_punishment_translation(
            &pool,
            punishment.id,
            i18n.id,
            "Ban",
            "User is banned for 1 hour",
        )
            .await
            .expect("failed to create new punishment translation");
        let after_create = chrono::Utc::now().naive_utc();
        assert!(new_translation.created_at >= before_create);
        assert!(new_translation.created_at <= after_create);
        assert!(new_translation.updated_at >= before_create);
        assert!(new_translation.updated_at <= after_create);
        assert_eq!(new_translation.title, "Ban");
        assert_eq!(new_translation.description, "User is banned for 1 hour");

        // Test Read
        let fetched_translation = get_punishment_translation_by_pk(
            &pool,
            punishment.id,
            i18n.id,
        )
            .await
            .expect("failed to read punishment translation by pk")
            .expect("failed to find a punishment translation with this pk");
        assert_eq!(fetched_translation.punishment_id, punishment.id);
        assert_eq!(fetched_translation.i18n_id, i18n.id);
        assert_eq!(fetched_translation.title, "Ban");
        assert_eq!(fetched_translation.description, "User is banned for 1 hour");

        // Test Update
        let before_update = chrono::Utc::now().naive_utc();
        let updated_translation = update_punishment_translation(
            &pool,
            punishment.id,
            i18n.id,
            "Banned",
            "User is currently banned. This punishment expires in 1 hour",
        )
            .await
            .expect("failed to update punishment translation")
            .expect("failed to update a punishment translation with this pk");
        let after_update = chrono::Utc::now().naive_utc();
        assert!(updated_translation.updated_at >= before_update);
        assert!(updated_translation.updated_at <= after_update);
        assert_eq!(updated_translation.title, "Banned");
        assert_eq!(updated_translation.description, "User is currently banned. This punishment expires in 1 hour");

        // Test Read All
        let all_translations = get_all_punishment_translations(&pool)
            .await
            .expect("failed to read all punishment translations");
        assert_eq!(all_translations.len(), 1);
        assert_eq!(all_translations[0].punishment_id, updated_translation.punishment_id);
        assert_eq!(all_translations[0].i18n_id, updated_translation.i18n_id);

        // Test Delete
        let deleted_translation = delete_punishment_translation(
            &pool,
            updated_translation.punishment_id,
            updated_translation.i18n_id,
        )
            .await
            .expect("failed to delete punishment translation")
            .expect("failed to delete a punishment translation with this pk");
        assert_eq!(deleted_translation.punishment_id, updated_translation.punishment_id);
        assert_eq!(deleted_translation.i18n_id, updated_translation.i18n_id);

        // Ensure it's deleted
        let should_be_none = get_punishment_translation_by_pk(
            &pool,
            updated_translation.punishment_id,
            updated_translation.i18n_id,
        )
            .await
            .expect("failed to read punishment translation");
        assert!(
            should_be_none.is_none(),
            "should not be able to find a punishment translation with this pk"
        );

        // Cleanup
        cleanup_isolated_db(&db_name).await;
    }
}
