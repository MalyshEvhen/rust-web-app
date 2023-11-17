use crate::{
  ctx::Ctx,
  model::{ModelManager, Result},
};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

use super::base::{self, DbBmc};

#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Task {
  pub id: i64,
  pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForCreate {
  pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForUpdate {
  pub title: Option<String>,
}

pub struct TaskBmc;

impl DbBmc for TaskBmc {
  const TABLE: &'static str = "task";
}

impl TaskBmc {
  pub async fn create(
    ctx: &Ctx,
    mm: &ModelManager,
    task_c: TaskForCreate,
  ) -> Result<i64> {
    base::create::<Self, _>(ctx, mm, task_c).await
  }

  pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
    base::get::<Self, _>(ctx, mm, id).await
  }

  pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
    base::list::<Self, _>(ctx, mm).await
  }

  pub async fn update(
    ctx: &Ctx,
    mm: &ModelManager,
    id: i64,
    task_u: TaskForUpdate,
  ) -> Result<()> {
    base::update::<Self, _>(ctx, mm, id, task_u).await
  }

  pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
    base::delete::<Self>(ctx, mm, id).await
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::_dev_utils;
  use anyhow::Result;
  use crate::model::Error;
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
  async fn test_update_ok() -> Result<()> {
    // -- Setup & Fixtures
    let mm = _dev_utils::init_test().await;
    let ctx = Ctx::root_ctx();
    let fx_title = "test_update_ok - task 01";
    let fx_new_title = "test_update_ok - task 01 - new";
    let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
      .await?
      .remove(0);

    // -- Execute
    TaskBmc::update(
      &ctx,
      &mm,
      fx_task.id,
      TaskForUpdate {
        title: Some(fx_new_title.to_string()),
      },
    )
    .await?;

    // -- Execute
    let task: Task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;

    // -- Check
    assert_eq!(task.title, fx_new_title);

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
