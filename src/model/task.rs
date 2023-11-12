use crate::{
	ctx::Ctx,
	model::{task, ModelManager, Result},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
	pub id: i64,
	pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
	pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
	pub title: Option<String>,
}

pub struct TaskBmc;

impl TaskBmc {
	pub async fn create(
		_ctx: &Ctx,
		mm: &ModelManager,
		task_c: TaskForCreate,
	) -> Result<i64> {
		let db = mm.db();

		let (id,) = sqlx::query_as::<_, (i64,)>(
			"INSERT INTO task (title) VALUES ($1) RETURNING id",
		)
		.bind(task_c.title)
		.fetch_one(db)
		.await?;

		Ok(id)
	}
}

#[cfg(test)]
mod test {
	use crate::_dev_utils;

	use super::*;
	use anyhow::Result;

	#[tokio::test]
	async fn test_create_ok() -> Result<()> {
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_title = "test_create_ok test";

		let task_c = TaskForCreate {
			title: fx_title.to_string(),
		};

		// -- Execute
		let id = TaskBmc::create(&ctx, &mm, task_c).await?;

		// -- Execute
		let (title,): (String,) = sqlx::query_as("SELECT * FROM task WHERE id = $1")
			.bind(id)
			.fetch_one(mm.db())
			.await?;

		// -- Check
		assert_eq!(title, fx_title);

		// -- Cleanup
		let rows_affected = sqlx::query("DELETE FROM task WHERE id = $1")
			.bind(id)
			.execute(mm.db())
            .await?
            .rows_affected();

        // -- Check
        assert_eq!(rows_affected, 1, "Did not delete 1 row");

		Ok(())
	}
}
