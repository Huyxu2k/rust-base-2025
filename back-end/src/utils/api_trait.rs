use serde::{Deserialize, Serialize};



pub trait PathParamsApi {
    fn get_by_id(id: i32);
    fn delete_by_id(id: i32);
}

pub trait QueryParamsApi<T>
where 
    T: for<'a> Deserialize<'a>+ Serialize
{
    fn search(params: T);
    fn delete_multiple(params: T);
}

pub trait BodyParamsApi<T> 
where 
    T: for<'a> Deserialize<'a>+Serialize,
{
    fn create(data: T);
    fn update(data: T);
}