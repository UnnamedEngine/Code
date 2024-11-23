#[cfg(test)]
mod tests {
    use crate::{db::account::punishment::{delete_punishment, get_all_punishments, get_punishment_by_id, insert_punishment, update_punishment}, tests::db_tests::utils::{cleanup_isolated_db, setup_isolated_db}};

    #[tokio::test]
    async fn test_punishment_crud_operations() {
        let (pool, db_name) = setup_isolated_db().await;

        // Test Create
        let before_create = chrono::Utc::now().naive_utc();
        let new_punishment = insert_punishment(
            &pool,
            "ban",
            3154000000,
        )
            .await
            .expect("failed to create new punishment");
        let after_create = chrono::Utc::now().naive_utc();
        assert!(new_punishment.created_at >= before_create);
        assert!(new_punishment.created_at <= after_create);
        assert!(new_punishment.updated_at >= before_create);
        assert!(new_punishment.updated_at <= after_create);
        assert_eq!(new_punishment.slug, "ban");
        assert_eq!(new_punishment.default_duration, 3154000000);

        // Test Read
        let fetched_punishment = get_punishment_by_id(
            &pool,
            new_punishment.id,
        )
            .await
            .expect("failed to read punishment by id")
            .expect("failed to find a punishment with this id");
        assert_eq!(fetched_punishment.id, new_punishment.id);
        assert_eq!(fetched_punishment.slug, "ban");
        assert_eq!(fetched_punishment.default_duration, 3154000000);

        // Test Update
        let before_update = chrono::Utc::now().naive_utc();
        let updated_punishment = update_punishment(
            &pool,
            new_punishment.id,
            "mute",
            259200,
        )
            .await
            .expect("failed to update punishment")
            .expect("failed to update a punishment with this id");
        let after_update = chrono::Utc::now().naive_local();
        assert!(updated_punishment.updated_at >= before_update);
        assert!(updated_punishment.updated_at <= after_update);
        assert_eq!(updated_punishment.slug, "mute");
        assert_eq!(updated_punishment.default_duration, 259200);

        // Test Read All
        let all_punishments = get_all_punishments(&pool)
            .await
            .expect("failed to read all punishments");
        assert_eq!(all_punishments.len(), 1);
        assert_eq!(all_punishments[0].id, updated_punishment.id);

        // Test Delete
        let deleted_punishment = delete_punishment(
            &pool,
            updated_punishment.id,
        )
            .await
            .expect("failed to delete punishment")
            .expect("failed to delete a punishment with this id");
        assert_eq!(deleted_punishment.id, updated_punishment.id);

        // Ensure it's deleted
        let should_be_none = get_punishment_by_id(
            &pool,
            updated_punishment.id,
        )
            .await
            .expect("failed to read punishment");
        assert!(
            should_be_none.is_none(),
            "should not be able to find a punishment with this id",
        );

        // Cleanup
        cleanup_isolated_db(&db_name).await;
    }
}
