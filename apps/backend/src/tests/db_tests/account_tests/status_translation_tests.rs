#[cfg(test)]
mod tests {
    use crate::{db::{account::{status::insert_status, status_translation::{delete_status_translation, get_all_status_translations, get_status_translation_by_pk, insert_status_translation, update_status_translation}}, public::insert_i18n}, tests::db_tests::utils::{cleanup_isolated_db, setup_isolated_db}};


    #[tokio::test]
    async fn test_status_translation_crud_operations() {
        let (pool, db_name) = setup_isolated_db().await;

        // Create a new i18n to reference
        let i18n = insert_i18n(
            &pool,
            "English",
            "en",
        )
            .await
            .expect("failed to create reference i18n");

        // Create a new status to reference
        let status = insert_status(
            &pool,
            "active",
        )
            .await
            .expect("failed to create reference status");

        // Test Create
        let before_create = chrono::Utc::now().naive_utc();
        let new_translation = insert_status_translation(
            &pool,
            status.id,
            i18n.id,
            "Active",
            "Account is currently active",
        )
            .await
            .expect("failed to create new status translation");
        let after_create = chrono::Utc::now().naive_utc();
        assert!(new_translation.created_at >= before_create);
        assert!(new_translation.created_at <= after_create);
        assert!(new_translation.updated_at >= before_create);
        assert!(new_translation.updated_at <= after_create);
        assert_eq!(new_translation.title, "Active");
        assert_eq!(new_translation.description, "Account is currently active");

        // Test Read
        let fetched_translation = get_status_translation_by_pk(
            &pool,
            status.id,
            i18n.id,
        )
            .await
            .expect("failed to read status translation by pk")
            .expect("failed to find a status translation with this pk");
        assert_eq!(fetched_translation.status_id, status.id);
        assert_eq!(fetched_translation.i18n_id, i18n.id);
        assert_eq!(fetched_translation.title, "Active");
        assert_eq!(fetched_translation.description, "Account is currently active");

        // Test Update
        let before_update = chrono::Utc::now().naive_utc();
        let updated_translation = update_status_translation(
            &pool,
            status.id,
            i18n.id,
            "Pending",
            "Account is pending confirmation",
        )
            .await
            .expect("failed to update status translation")
            .expect("failed to update a status translation with this pk");
        let after_update = chrono::Utc::now().naive_utc();
        assert!(updated_translation.updated_at >= before_update);
        assert!(updated_translation.updated_at <= after_update);
        assert_eq!(updated_translation.title, "Pending");
        assert_eq!(updated_translation.description, "Account is pending confirmation");

        // Test Read All
        let all_translations = get_all_status_translations(&pool)
            .await
            .expect("failed to read all punishment translations");
        assert_eq!(all_translations.len(), 1);
        assert_eq!(all_translations[0].status_id, updated_translation.status_id);
        assert_eq!(all_translations[0].i18n_id, updated_translation.i18n_id);

        // Test Delete
        let deleted_translation = delete_status_translation(
            &pool,
            updated_translation.status_id,
            updated_translation.i18n_id,
        )
            .await
            .expect("failed to delete status translation")
            .expect("failed to delete a status translation with this pk");
        assert_eq!(deleted_translation.status_id, updated_translation.status_id);
        assert_eq!(deleted_translation.i18n_id, updated_translation.i18n_id);

        // Ensure it's deleted
        let should_be_none = get_status_translation_by_pk(
            &pool,
            updated_translation.status_id,
            updated_translation.i18n_id,
        )
            .await
            .expect("failed to read status translation");
        assert!(
            should_be_none.is_none(),
            "should not be able to find a status translation with this pk"
        );

        // Cleanup
        cleanup_isolated_db(&db_name).await;
    }
}
