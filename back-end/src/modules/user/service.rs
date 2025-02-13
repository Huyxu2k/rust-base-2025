use diesel::{query_dsl::methods::{FindDsl, OrderDsl, SelectDsl}, ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper};

use super::model::{User, UserRegister, NewUser};
use crate::{db_pool::{get_conn, DbPool}, schema::_users::dsl::*, to_hash};
use crate::schema::_users;

pub async fn create_user(cre_user: UserRegister, pool: &DbPool,cre_id:i32)->Result<User,String>{
    let mut conn=get_conn(&pool);

    let new_user= NewUser{
        EmployeeId: None,
        Username: cre_user.user_name,
        PasswordHash: to_hash(cre_user.password),
        Email: cre_user.email,
        EmailVerified: Some(true),
        IsActive: Some(true),
        CreatedBy: cre_id,
        UpdatedBy: cre_id,
        CreatedAt: Some(chrono::Utc::now().naive_utc()),
        UpdatedAt: Some(chrono::Utc::now().naive_utc()),
    };

    let result= diesel::insert_into(_users)
                                                    .values(&new_user)
                                                    .execute(&mut conn);
    
    
                                                    
   match result {
    Ok(_) => {
        let user=_users::table
            .order(_users::ID.desc())
            .select(User::as_select())
            .first::<User>(&mut conn).unwrap();
            Ok(user)

    },
    Err(e) => Err(format!("Insert error: {}",e)),
   }
    
}

//TODO 
pub async fn update_user(user:User,pool: &DbPool)->Result<User,String>{
    // let mut conn=get_conn(&pool);
    // let result=diesel::update(_users.find(user.ID))
    //                                                     .set((Username.eq(user.Username),Em;ail))

    // Ok(())
    todo!()
}
pub async fn delete_user(id: i32,pool: &DbPool)->Result<i32,String>{
    let mut conn=get_conn(&pool);
    let mut user=get_user_by_id(id,pool).await.unwrap();
    user.IsActive=Some(false);

    let result= diesel::update(_users.find(id))
                        .set(IsActive.eq(false))
                        .execute(&mut conn);

    match result {
        Ok(_) => Ok(id),
        Err(e) => Err(format!("Delete error: {}",e)) ,
    }
}
pub async fn modify_password(id: i32, old_password: String, new_password:String,is_admin: bool,pool: &DbPool)->Result<User,String>{
    let mut conn=get_conn(&pool);
   if is_admin{
        todo!()
   }else {
    let mut user=get_user_by_id(id,pool).await.unwrap();
    let pwd_hash=to_hash(old_password);
    if pwd_hash==user.PasswordHash{
        user.PasswordHash=pwd_hash;
        let result=diesel::update(_users.find(id))
                        .set(PasswordHash.eq(to_hash(new_password)))
                        .execute(&mut conn);
        match result {
            Ok(_) => Ok(user),
            Err(e) => Err(format!("Update password error: {}",e)),
        }
    }else {
        Err("Old password is incorrect".to_string())
    }
   }
}

pub async fn get_user_by_id(id:i32,pool: &DbPool)->Result<User,String>{
    let mut conn=get_conn(&pool);
    let result= _users.find(id)
                                                            .select(User::as_select())
                                                            .first(&mut conn)
                                                            .optional();
    match result {
        Ok(Some(user)) => Ok(user),
        Ok(None)=>Err(format!("Not found User by Id: {}",id)),
        Err(e) => Err(format!("Get by id error: {}",e)),
    }
}
pub async fn get_all_user(pool: &DbPool)->Result<Vec<User>,String>{
    let mut conn=get_conn(&pool);
    let result= _users.select(User::as_select())
                                                        .load::<User>(&mut conn);
    match result {
        Ok(users) => Ok(users),
        Err(e) => Err(format!("Get all error: {}",e)),
    }
}