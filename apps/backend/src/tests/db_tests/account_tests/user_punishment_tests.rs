#[cfg(test)]
mod tests {
    use crate::{db::account::{punishment::insert_punishment, status::insert_status, user::insert_user, user_punishment::{delete_user_punishment, get_all_user_punishments, get_user_punishment_by_id, insert_user_punishment, update_user_punishment}}, tests::db_tests::utils::{cleanup_isolated_db, setup_isolated_db}};

    #[tokio::test]
    async fn test_user_punishment_crud_operations() {
        let (pool, db_name) = setup_isolated_db().await;

        // Create a new punishment to reference
        let punishment = insert_punishment(
            &pool,
            "mute",
            3600,
        )
            .await
            .expect("failed to create reference punishment");

        // Create a second punishment to reference
        let second_punishment = insert_punishment(
            &pool,
            "ban",
            4800,
        )
            .await
            .expect("failed to create second reference punishment");

        // Create a new status to reference
        let status = insert_status(
            &pool,
            "active",
        )
            .await
            .expect("failed to create reference status");

        // Create a new user to reference
        let user = insert_user(
            &pool,
            status.id,
            "test@test.com",
            "test",
            "Test Master",
        )
            .await
            .expect("failed to create reference user");

        // Test Create
        let before_create = chrono::Utc::now().naive_utc();
        let new_user_punishment = insert_user_punishment(
            &pool,
            user.id,
            punishment.id,
            punishment.default_duration + 3600,
        )
            .await
            .expect("failed to create new user punishment");
        let after_create = chrono::Utc::now().naive_utc();
        assert!(new_user_punishment.applied_at >= before_create);
        assert!(new_user_punishment.applied_at <= after_create);
        assert_eq!(new_user_punishment.user_id, user.id);
        assert_eq!(new_user_punishment.punishment_id, punishment.id);
        assert_eq!(new_user_punishment.duration, punishment.default_duration + 3600);

        // Test Read
        let fetched_user_punishment = get_user_punishment_by_id(
            &pool,
            new_user_punishment.id,
        )
            .await
            .expect("failed to read user punishment by id")
            .expect("failed to find an user punishment with this id");
        assert_eq!(fetched_user_punishment.user_id, user.id);
        assert_eq!(fetched_user_punishment.punishment_id, punishment.id);
        assert_eq!(fetched_user_punishment.duration, punishment.default_duration + 3600);

        // Test Update
        let updated_user_punishment = update_user_punishment(
            &pool,
            new_user_punishment.id,
            user.id,
            second_punishment.id,
            second_punishment.default_duration,
        )
            .await
            .expect("failed to update user punishment")
            .expect("failed to update an user punishment with this id");
        assert_eq!(updated_user_punishment.user_id, user.id);
        assert_eq!(updated_user_punishment.punishment_id, second_punishment.id);
        assert_eq!(updated_user_punishment.duration, second_punishment.default_duration);

        // Test Read All
        let all_user_punishments = get_all_user_punishments(&pool)
            .await
            .expect("failed to read all user punishments");
        assert_eq!(all_user_punishments.len(), 1);
        assert_eq!(all_user_punishments[0].id, updated_user_punishment.id);

        // Test Delete
        let deleted_user_punishment = delete_user_punishment(
            &pool,
            updated_user_punishment.id,
        )
            .await
            .expect("failed to delete user punishment")
            .expect("failed to delete an user punishment with this id");
        assert_eq!(deleted_user_punishment.id, updated_user_punishment.id);

        // Ensure it's deleted
        let should_be_none = get_user_punishment_by_id(
            &pool,
            updated_user_punishment.id,
        )
            .await
            .expect("failed to read user punishment");
        assert!(
            should_be_none.is_none(),
            "should not be able to find an user punishment with this id",
        );

        // Cleanup
        cleanup_isolated_db(&db_name).await;
    }
}
