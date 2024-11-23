#[cfg(test)]
mod tests {
    use crate::{db::account::status::{delete_status, get_all_status, get_status_by_id, insert_status, update_status}, tests::db_tests::utils::{cleanup_isolated_db, setup_isolated_db}};

    #[tokio::test]
    async fn test_status_crud_operations() {
        let (pool, db_name) = setup_isolated_db().await;

        // Test Create
        let before_create = chrono::Utc::now().naive_utc();
        let new_status = insert_status(
            &pool,
            "pending",
        )
            .await
            .expect("failed to create new status");
        let after_create = chrono::Utc::now().naive_utc();
        assert!(new_status.created_at >= before_create);
        assert!(new_status.created_at <= after_create);
        assert!(new_status.updated_at >= before_create);
        assert!(new_status.updated_at <= after_create);
        assert_eq!(new_status.slug, "pending");

        // Test Read
        let fetched_status = get_status_by_id(
            &pool,
            new_status.id,
        )
            .await
            .expect("failed to read status by id")
            .expect("failed to find a status with this id");
        assert_eq!(fetched_status.id, new_status.id);
        assert_eq!(fetched_status.slug, "pending");

        // Test Update
        let before_update = chrono::Utc::now().naive_utc();
        let updated_status = update_status(
            &pool,
            new_status.id,
            "active",
        )
            .await
            .expect("failed to update status")
            .expect("failed to update a status with this id");
        let after_update = chrono::Utc::now().naive_utc();
        assert!(updated_status.updated_at >= before_update);
        assert!(updated_status.updated_at <= after_update);
        assert_eq!(updated_status.slug, "active");

        // Test Read All
        let all_status = get_all_status(&pool)
            .await
            .expect("failed to read all status");
        assert_eq!(all_status.len(), 1);
        assert_eq!(all_status[0].id, updated_status.id);

        // Test Delete
        let deleted_status = delete_status(
            &pool,
            updated_status.id,
        )
            .await
            .expect("failed to delete status")
            .expect("failed to delete a status with this id");
        assert_eq!(deleted_status.id, updated_status.id);

        // Ensure it's deleted
        let should_be_none = get_status_by_id(
            &pool,
            updated_status.id,
        )
            .await
            .expect("failed to read status");
        assert!(
            should_be_none.is_none(),
            "should not be able to find a status with this id",
        );

        // Cleanup
        cleanup_isolated_db(&db_name).await;
    }
}
