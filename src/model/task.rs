use crate::{
	ctx::Ctx,
	model::{task, Error, ModelManager, Result},
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

	pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
		let db = mm.db();

		let task = sqlx::query_as::<_, Task>("SELECT * FROM task WHERE id = $1")
			.bind(id)
			.fetch_optional(db)
			.await?
			.ok_or(Error::EntityNotFound { entity: "task", id })?;

		Ok(task)
	}

	pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
		let db = mm.db();

		let tasks = sqlx::query_as::<_, Task>("SELECT * FROM task ORDER BY id")
			.fetch_all(db)
			.await?;

		Ok(tasks)
	}

	pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		let db = mm.db();

		let rows_affected = sqlx::query("DELETE FROM task WHERE id = $1")
			.bind(id)
			.execute(db)
			.await?
			.rows_affected();

		if rows_affected == 0 {
			return Err(Error::EntityNotFound { entity: "task", id });
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::_dev_utils;
	use super::*;
	use anyhow::Result;
	use serial_test::serial;

	#[serial]
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
		let task: Task = TaskBmc::get(&ctx, &mm, id).await?;

		// -- Check
		assert_eq!(task.title, fx_title);

		// -- Cleanup
		TaskBmc::delete(&ctx, &mm, id).await?;

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_get_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Execute
		let res = TaskBmc::get(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound { entity, id })
					if entity == "task" && id == fx_id
			),
			"Entity not found matching",
		);

		Ok(())
	}

	#[serial]
	#[tokio::test]
	async fn test_list_ok() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_titles = &["test_list_ok-task 01", "test_list_ok-task 02"];
		_dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

		// -- Execute
		let tasks = TaskBmc::list(&ctx, &mm).await?;

		// -- Check
		let tasks: Vec<Task> = tasks
			.into_iter()
			.filter(|t| t.title.starts_with("test_list_ok-task"))
			.collect();
		assert_eq!(tasks.len(), 2, "number of tasks");

		// -- Cleanup
		for task in tasks {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }

	#[serial]
	#[tokio::test]
	async fn test_delete_err_not_found() -> Result<()> {
		// -- Setup & Fixtures
		let mm = _dev_utils::init_test().await;
		let ctx = Ctx::root_ctx();
		let fx_id = 100;

		// -- Execute
		let res = TaskBmc::delete(&ctx, &mm, fx_id).await;

		// -- Check
		assert!(
			matches!(
				res,
				Err(Error::EntityNotFound { entity, id })
					if entity == "task" && id == fx_id
			),
			"Entity not found matching",
		);

		Ok(())
	}
}
