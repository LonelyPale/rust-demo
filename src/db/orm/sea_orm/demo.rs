use sea_orm::sea_query::{Expr, Value};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection,
    DbErr, EntityTrait, ExecResult, IntoActiveModel, ModelTrait, PaginatorTrait, QueryFilter,
    Schema, Set, Statement,
};

// use crate::db::orm::sea_orm::user;
use super::*;

async fn get_db_conn() -> Result<DatabaseConnection, DbErr> {
    // let db = Database::connect("sqlite::memory:").await?;
    // let db = Database::connect("sqlite:data/test.db?mode=rwc").await?;
    let db = Database::connect("sqlite://data/test.db?mode=rwc").await?;
    Ok(db)
}

#[tokio::test]
async fn create_table() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;
    let builder = db.get_database_backend();
    // let schema = Schema::new(demo::DbBackend::Sqlite);
    let schema = Schema::new(builder);
    let stmt = schema.create_table_from_entity(user::Entity);
    let result = db.execute(builder.build(&stmt)).await?;
    println!("stmt.result: {:?}", result);

    Ok(())
}

#[tokio::test]
async fn save() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    use sea_orm::ActiveValue::NotSet;

    let banana = user::ActiveModel {
        id: NotSet, // primary key is NotSet
        name: Set("Banana".to_owned()),
        ..Default::default() // all other attributes are `NotSet`
    };

    // Insert, because primary key `id` is `NotSet`
    let mut banana: user::ActiveModel = banana.save(&db).await?;
    println!("save.1: {:?}", banana);

    banana.name = Set("Banana Mongo".to_owned());

    // Update, because primary key `id` is `Unchanged`
    let banana: user::ActiveModel = banana.save(&db).await?;
    println!("save.2: {:?}", banana);

    Ok(())
}

#[tokio::test]
async fn insert() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let apple = user::ActiveModel {
        name: Set("Apple 1".to_owned()), //sea_orm::entity::Set()
        ..Default::default()             // no need to set primary key
    };

    let insert_result = apple.insert(&db).await?;
    println!("insert_result: {:?}", insert_result);

    Ok(())
}

#[tokio::test]
async fn insert2() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let apple = user::ActiveModel {
        name: Set("Apple 3".to_owned()),
        ..Default::default()
    };
    let orange = user::ActiveModel {
        name: Set("Orange".to_owned()),
        ..Default::default()
    };

    let insert_result = User::insert_many([apple, orange])
        .exec_with_returning(&db)
        .await?;
    println!("insert_result: {:?}", insert_result);

    Ok(())
}

#[tokio::test]
async fn insert_one() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let apple = user::ActiveModel {
        name: Set("Apple 2".to_owned()),
        ..Default::default()
    };

    let insert_result = user::Entity::insert(apple).exec(&db).await?;
    println!("insert_result: {:?}", insert_result);

    Ok(())
}

#[tokio::test]
async fn insert_many() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let apple = user::ActiveModel {
        name: Set("Apple 3".to_owned()),
        ..Default::default()
    };
    let orange = user::ActiveModel {
        name: Set("Orange".to_owned()),
        ..Default::default()
    };

    let insert_result = user::Entity::insert_many([apple, orange]).exec(&db).await?;
    println!("insert_result: {:?}", insert_result);

    Ok(())
}

#[tokio::test]
async fn update_one() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;
    let result = User::find_by_id(21).one(&db).await?;
    println!("update_one: {:?}", result);

    let mut result: user::ActiveModel = result.unwrap_or_default().into();
    result.name = Set("Sweet pear".to_owned());
    let result = result.update(&db).await?;
    println!("update_one: {:?}", result);

    Ok(())
}

#[tokio::test]
async fn find_one() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;
    let result = user::Entity::find().one(&db).await?.unwrap_or_default();
    println!("{:?}", result);

    Ok(())
}

#[tokio::test]
async fn update_many() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let result: Option<user::Model> = User::find_by_id(21).one(&db).await?;
    println!("update_many: {:?}", result);

    // let result: user::ActiveModel = result.unwrap_or_default().into();
    let mut result = result.unwrap_or_default().into_active_model();
    // result.name = Set("Sweet pear--".to_owned());
    // let result = result.reset_all();
    result.reset(user::Column::Name);

    let update_result = User::update_many()
        .set(result)
        .filter(user::Column::Id.eq(22))
        .exec(&db)
        .await?;
    println!("update_many: {:?}", update_result);

    Ok(())
}

#[tokio::test]
async fn update_many2() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let update_result = User::update_many()
        .col_expr(
            user::Column::Name,
            Expr::value(Value::String(Some(Box::new("Apple pear".to_string())))),
        )
        .filter(user::Column::Name.contains("apple"))
        .exec(&db)
        .await?;
    println!("update_many: {:?}", update_result);

    Ok(())
}

#[tokio::test]
async fn delete_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;
    let delete_result = User::delete_by_id(28).exec(&db).await?;
    println!("delete_by_id: {:?}", delete_result);
    Ok(())
}

#[tokio::test]
async fn delete_one() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let result: Option<user::Model> = User::find_by_id(21).one(&db).await?;
    let result: user::Model = result.unwrap_or_default();
    let delete_result = user::Entity::delete(result.into_active_model())
        .exec(&db)
        .await?;
    println!("delete_one: {:?}", delete_result);

    Ok(())
}

#[tokio::test]
async fn delete_one2() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let result: Option<user::Model> = User::find_by_id(22).one(&db).await?;
    let result: user::Model = result.unwrap_or_default();
    let delete_result = result.delete(&db).await?;
    println!("delete_one: {:?}", delete_result);

    Ok(())
}

#[tokio::test]
async fn delete_one3() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let result: Option<user::Model> = User::find_by_id(27).one(&db).await?;
    let result = result.unwrap_or_default().into_active_model();
    let delete_result = result.delete(&db).await?;
    println!("delete_one: {:?}", delete_result);

    Ok(())
}

#[tokio::test]
async fn delete_many() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let delete_result = user::Entity::delete_many()
        .filter(user::Column::Name.contains("Orange"))
        .exec(&db)
        .await?;
    println!("delete_many: {:?}", delete_result);

    Ok(())
}

#[tokio::test]
async fn find_all() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;
    let result = user::Entity::find().all(&db).await?;

    for item in result {
        println!("{:?}", item);
    }

    Ok(())
}

#[tokio::test]
async fn find_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;
    let result = user::Entity::find_by_id(2)
        .one(&db)
        .await?
        .unwrap_or_default();
    println!("{:?}", result);

    Ok(())
}

#[tokio::test]
async fn count() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;
    let result = user::Entity::find().count(&db).await?;
    println!("count: {:?}", result);
    Ok(())
}

#[tokio::test]
async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_conn().await?;

    let sql = r#"
CREATE TABLE "fruit" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT NOT NULL
);
"#;

    let result: ExecResult = db
        .execute(Statement::from_string(DatabaseBackend::Sqlite, sql))
        .await?;

    println!("execute: {:?}", result);
    Ok(())
}
