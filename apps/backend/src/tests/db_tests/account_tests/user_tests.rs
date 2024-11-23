#[cfg(test)]
mod tests {
    use crate::{db::account::{status::insert_status, user::{delete_user, get_all_users, get_user_by_id, insert_user, update_user}}, tests::db_tests::utils::{cleanup_isolated_db, setup_isolated_db}};

    #[tokio::test]
    async fn test_user_crud_operations() {
        let (pool, db_name) = setup_isolated_db().await;

        // Create a new status to reference
        let status = insert_status(
            &pool,
            "active",
        )
            .await
            .expect("failed to create reference status");

        // Create a second status to reference
        let second_status = insert_status(
            &pool,
            "inactive",
        )
            .await
            .expect("failed to create a second reference status");

        // Test Create
        let before_create = chrono::Utc::now().naive_utc();
        let new_user = insert_user(
            &pool,
            status.id,
            "test@test.com",
            "test",
            "Test Master",
        )
            .await
            .expect("failed to create new user");
        let after_create = chrono::Utc::now().naive_utc();
        assert!(new_user.created_at >= before_create);
        assert!(new_user.created_at <= after_create);
        assert!(new_user.updated_at >= before_create);
        assert!(new_user.updated_at <= after_create);
        assert_eq!(new_user.status_id, status.id);
        assert_eq!(new_user.email, "test@test.com");
        assert_eq!(new_user.username, "test");
        assert_eq!(new_user.display_name, "Test Master");
        assert_eq!(new_user.last_login, None);

        // Test Read
        let fetched_user = get_user_by_id(
            &pool,
            new_user.id,
        )
            .await
            .expect("failed to find user by id")
            .expect("failed to find an user with this id");
        assert_eq!(fetched_user.status_id, status.id);
        assert_eq!(new_user.email, "test@test.com");
        assert_eq!(new_user.username, "test");
        assert_eq!(new_user.display_name, "Test Master");
        assert_eq!(new_user.last_login, None);

        // Test Update
        let before_update = chrono::Utc::now().naive_utc();
        let updated_user = update_user(
            &pool,
            new_user.id,
            second_status.id,
            "admin@admin.com",
            "admin",
            "Administrator",
            Some(chrono::Utc::now().naive_utc()),
        )
            .await
            .expect("failed to update user")
            .expect("failed to update an user with this id");
        let after_update = chrono::Utc::now().naive_utc();
        assert!(updated_user.updated_at >= before_update);
        assert!(updated_user.updated_at <= after_update);
        assert_eq!(updated_user.status_id, second_status.id);
        assert_eq!(updated_user.email, "admin@admin.com");
        assert_eq!(updated_user.username, "admin");
        assert_eq!(updated_user.display_name, "Administrator");
        assert!(updated_user.last_login.unwrap() >= before_update);
        assert!(updated_user.last_login.unwrap() <= after_update);

        // Test Read All
        let all_users = get_all_users(&pool)
            .await
            .expect("failed to read all users");
        assert_eq!(all_users.len(), 1);
        assert_eq!(all_users[0].id, updated_user.id);

        // Test Delete
        let deleted_user = delete_user(
            &pool,
            updated_user.id,
        )
            .await
            .expect("failed to delete user")
            .expect("failed to delete an user with this id");
        assert_eq!(deleted_user.status_id, updated_user.status_id);
        assert_eq!(deleted_user.email, updated_user.email);
        assert_eq!(deleted_user.username, updated_user.username);
        assert_eq!(deleted_user.display_name, updated_user.display_name);
        assert_eq!(deleted_user.last_login, updated_user.last_login);

        // Ensure it's deleted
        let should_be_none = get_user_by_id(
            &pool,
            updated_user.id,
        )
            .await
            .expect("failed to read user");
        assert!(
            should_be_none.is_none(),
            "should not be able to find an user with this id",
        );

        // Cleanup
        cleanup_isolated_db(&db_name).await;
    }
}
