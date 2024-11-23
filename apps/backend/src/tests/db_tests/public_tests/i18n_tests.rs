#[cfg(test)]
mod tests {
    use crate::{db::public::{delete_i18n, get_all_i18n, get_i18n_by_id, insert_i18n, update_i18n}, tests::db_tests::utils::{cleanup_isolated_db, setup_isolated_db}};

    #[tokio::test]
    async fn test_i18n_crud_operations() {
        let (pool, db_name) = setup_isolated_db().await;

        // Test Create
        let new_i18n = insert_i18n(&pool, "English", "en")
            .await
            .expect("failed create new i18n");
        assert_eq!(new_i18n.written_name, "English");
        assert_eq!(new_i18n.language_code, "en");

        // Test Read
        let fetched_i18n = get_i18n_by_id(&pool, new_i18n.id)
            .await
            .expect("failed to read i18n by id")
            .expect("failed to find an i18n with this id");
        assert_eq!(fetched_i18n.id, new_i18n.id);
        assert_eq!(fetched_i18n.written_name, "English");
        assert_eq!(fetched_i18n.language_code, "en");

        // Test Update
        let updated_i18n = update_i18n(&pool, new_i18n.id, "American English", "en-us")
            .await
            .expect("failed to update i18n")
            .expect("failed to update an i18n with this id");
        assert_eq!(updated_i18n.written_name, "American English");
        assert_eq!(updated_i18n.language_code, "en-us");

        // Test Read All
        let all_i18n = get_all_i18n(&pool)
            .await
            .expect("failed to read all i18n");
        assert_eq!(all_i18n.len(), 1);
        assert_eq!(all_i18n[0].id, updated_i18n.id);

        // Test Delete
        let deleted_i18n = delete_i18n(&pool, updated_i18n.id)
            .await
            .expect("failed to delete i18n")
            .expect("faield to delete an i18n with this id");
        assert_eq!(deleted_i18n.id, updated_i18n.id);

        // Ensure it's deleted
        let should_be_none = get_i18n_by_id(&pool, updated_i18n.id)
            .await
            .expect("failed to read i18n");
        assert!(
            should_be_none.is_none(),
            "should not be able to find an i18n with this id",
        );

        // Cleanup
        cleanup_isolated_db(&db_name).await;
    }
}
