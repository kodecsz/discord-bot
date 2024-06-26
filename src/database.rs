use crate::models::{
    AnimalModel, BangPointModel, InfractionModel, Punishment, PunishmentModel, Severity, TagModel,
    UserInfractionModel,
};
use serenity::all::UserId;
use sqlx::{
    postgres::{PgPoolOptions, PgQueryResult},
    Error, Pool, Postgres,
};

pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(database_url: String) -> Result<Self, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await?;

        Ok(Self { pool })
    }

    pub async fn add_animal(
        &self,
        animal: &str,
        emoji: &str,
        points: i32,
    ) -> Result<AnimalModel, Error> {
        sqlx::query_as!(
            AnimalModel,
            r#"INSERT INTO animals (animal, emoji, points) VALUES ($1, $2, $3) RETURNING id, animal, emoji, points"#,
            animal,
            emoji,
            points
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn remove_animal(&self, animal: &str) -> Result<PgQueryResult, Error> {
        sqlx::query!("DELETE FROM animals WHERE animal = $1", animal)
            .execute(&self.pool)
            .await
    }

    pub async fn get_animal(&self, animal: &str) -> Result<AnimalModel, Error> {
        sqlx::query_as!(
            AnimalModel,
            r#"SELECT * FROM animals WHERE animal = $1"#,
            animal
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_animals(&self) -> Result<Vec<AnimalModel>, Error> {
        sqlx::query_as!(AnimalModel, r#"SELECT * FROM animals"#)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_bang_ranking(&self) -> Result<Vec<BangPointModel>, Error> {
        sqlx::query_as!(
            BangPointModel,
            r#"SELECT * FROM bang_points ORDER BY points DESC LIMIT 10"#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_user_bang_points(&self, user_id: String) -> Result<BangPointModel, Error> {
        sqlx::query_as!(
            BangPointModel,
            r#"SELECT * FROM bang_points WHERE user_id = $1"#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn add_user_bang_points(
        &self,
        user_id: String,
        points: i32,
    ) -> Result<BangPointModel, Error> {
        sqlx::query_as!(
            BangPointModel,
            r#"UPDATE bang_points SET points = points + $1 WHERE user_id = $2 RETURNING id, user_id, points"#,
            points,
            user_id
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create_user_bang_points(
        &self,
        user_id: String,
        points: i32,
    ) -> Result<BangPointModel, Error> {
        sqlx::query_as!(
            BangPointModel,
            r#"INSERT INTO bang_points (user_id, points) VALUES ($1, $2) RETURNING id, user_id, points"#,
            user_id,
            points
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn create_or_add_user_bang_points(
        &self,
        user_id: String,
        points: i32,
    ) -> Result<BangPointModel, Error> {
        if let Ok(_) = self.get_user_bang_points(user_id.clone()).await {
            return self.add_user_bang_points(user_id, points).await;
        }

        self.create_user_bang_points(user_id, points).await
    }

    pub async fn log_user_punishment(
        &self,
        user_id: &UserId,
        punishment: Punishment,
        duration: i64,
    ) -> Result<PunishmentModel, Error> {
        sqlx::query_as!(
            PunishmentModel,
            r#"INSERT INTO punishments (user_id, punishment, duration) VALUES ($1, $2, $3) RETURNING id, user_id, punishment AS "punishment!: Punishment", duration"#,
            user_id.get().to_string(),
            punishment as Punishment,
            duration
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn log_user_infraction(
        &self,
        user_id: &UserId,
        infraction_id: i32,
    ) -> Result<UserInfractionModel, Error> {
        sqlx::query_as!(
            UserInfractionModel,
            r#"INSERT INTO user_infractions (user_id, infraction_id) VALUES ($1, $2) RETURNING id, user_id, infraction_id, created_at"#,
            user_id.get().to_string(),
            infraction_id
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_infraction(&self, id: i32) -> Result<InfractionModel, Error> {
        sqlx::query_as!(
            InfractionModel,
            r#"SELECT id, severity AS "severity!: Severity", punishment AS "punishment!: Punishment", duration FROM infractions WHERE id = $1"#,
            id
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_infractions(&self) -> Result<Vec<InfractionModel>, Error> {
        sqlx::query_as!(
            InfractionModel,
            r#"SELECT id, severity AS "severity!: Severity", punishment AS "punishment!: Punishment", duration FROM infractions"#
        )
            .fetch_all(&self.pool)
            .await
    }

    pub async fn add_infraction(
        &self,
        id: i32,
        severity: Severity,
        punishment: Punishment,
        duration: i64,
    ) -> Result<InfractionModel, Error> {
        sqlx::query_as!(
            InfractionModel,
            r#"INSERT INTO infractions (id, severity, punishment, duration) VALUES ($1, $2, $3, $4) RETURNING id, severity AS "severity!: Severity", punishment AS "punishment!: Punishment", duration"#,
            id,
            severity as Severity,
            punishment as Punishment,
            duration
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_infraction(
        &self,
        id: i32,
        severity: Severity,
        punishment: Punishment,
        duration: i64,
    ) -> Result<InfractionModel, Error> {
        sqlx::query_as!(
            InfractionModel,
            r#"UPDATE infractions SET severity = $1, punishment = $2, duration = $3 WHERE id = $4 RETURNING id, severity AS "severity!: Severity", punishment AS "punishment!: Punishment", duration"#,
            severity as Severity,
            punishment as Punishment,
            duration,
            id
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn remove_infraction(&self, id: i32) -> Result<PgQueryResult, Error> {
        sqlx::query!("DELETE FROM infractions WHERE id = $1", id)
            .execute(&self.pool)
            .await
    }

    pub async fn get_user_infractions(
        &self,
        user_id: UserId,
    ) -> Result<Vec<UserInfractionModel>, Error> {
        sqlx::query_as!(
            UserInfractionModel,
            r#"SELECT * FROM user_infractions WHERE user_id = $1"#,
            user_id.get().to_string()
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_tag(&self, name: &str) -> Result<TagModel, Error> {
        sqlx::query_as!(TagModel, r#"SELECT * FROM tags WHERE name = $1"#, name)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn add_tag(
        &self,
        name: &str,
        content: &str,
        user_id: UserId,
    ) -> Result<TagModel, Error> {
        sqlx::query_as!(
            TagModel,
            r#"INSERT INTO tags (user_id, name, content) VALUES ($1, $2, $3) RETURNING id, user_id, name, content"#,
            user_id.to_string(),
            name,
            content
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_tag(
        &self,
        name: &str,
        content: &str,
        user_id: UserId,
    ) -> Result<TagModel, Error> {
        sqlx::query_as!(
            TagModel,
            r#"UPDATE tags SET content = $1 WHERE user_id = $2 AND name = $3 RETURNING id, user_id, name, content"#,
            content,
            user_id.to_string(),
            name
        )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_all_tags(&self) -> Result<Vec<TagModel>, Error> {
        sqlx::query_as!(TagModel, r#"SELECT * FROM tags"#)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_user_tags(&self, user_id: UserId) -> Result<Vec<TagModel>, Error> {
        sqlx::query_as!(
            TagModel,
            r#"SELECT * FROM tags WHERE user_id = $1"#,
            user_id.to_string()
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn remove_tag(&self, name: &str, user_id: UserId) -> Result<PgQueryResult, Error> {
        sqlx::query!(
            "DELETE FROM tags WHERE name = $1 AND user_id = $2",
            name,
            user_id.to_string()
        )
        .execute(&self.pool)
        .await
    }
}
