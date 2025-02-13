use async_trait::async_trait;
use redis::Client;

use crate::config;

use super::Cache;



const DEFAULT_SEQ_STEP: i32 = 5000;

#[derive(Debug)]
pub struct RedisCache{
    client: redis::Client,
    seq_step: i32,
    single_seq_exe_sha: String,
    group_seq_exe_sha: String
}

impl RedisCache {
    #[allow(dead_code)]
    pub fn new(client: redis::Client)-> Self{
        let single_seq_exe_sha= Self::single_script_load(&client);
        let group_seq_exe_sha= Self::group_script_load(&client);
        let seq_step= DEFAULT_SEQ_STEP;
        
        Self { 
            client: client, 
            seq_step, 
            single_seq_exe_sha,
            group_seq_exe_sha
        }
    }
    pub fn from_config(config: &config::Config)-> Self{
        let client= redis::Client::open(config.redis.url()).unwrap();
        let single_seq_exe_sha= Self::single_script_load(&client);
        let group_seq_exe_sha= Self::group_script_load(&client);
        let mut seq_step= DEFAULT_SEQ_STEP;
        
        if config.redis.seq_step !=0{
            seq_step= config.redis.seq_step;
        }
        Self { 
            client: client, 
            seq_step, 
            single_seq_exe_sha,
            group_seq_exe_sha
        }
    }

    
    fn single_script_load(client: &redis::Client) -> String {
        let mut conn = client.get_connection().unwrap();

        let script = r#"
        local cur_seq = redis.call('HINCRBY', KEYS[1], 'cur_seq', 1)
        local max_seq = redis.call('HGET', KEYS[1], 'max_seq')
        local updated = false
        if max_seq == false then
            max_seq = tonumber(ARGV[1])
            redis.call('HSET', KEYS[1], 'max_seq', max_seq)
            end
        if tonumber(cur_seq) > tonumber(max_seq) then
            max_seq = tonumber(max_seq) + ARGV[1]
            redis.call('HSET', KEYS[1], 'max_seq', max_seq)
            updated = true
        end
        return {cur_seq, max_seq, updated}
        "#;
        redis::Script::new(script)
            .prepare_invoke()
            .load(&mut conn)
            .unwrap()
    }

    fn group_script_load(client: &redis::Client) -> String {
        let mut conn = client.get_connection().unwrap();

        let script = r#"
        local seq_step = tonumber(ARGV[1])
        local result = {}

        for i=2,#ARGV do
            local key = "seq:" .. ARGV[i]
            local cur_seq = redis.call('HINCRBY', key, 'cur_seq', 1)
            local max_seq = redis.call('HGET', key, 'max_seq')
            local updated = 0
            if max_seq == false then
                max_seq = seq_step
                redis.call('HSET', key, 'max_seq', max_seq)
            else
                max_seq = tonumber(max_seq)
            end
            if cur_seq > max_seq then
                max_seq = max_seq + seq_step
                redis.call('HSET', key, 'max_seq', max_seq)
                updated = 1
            end
            table.insert(result, {cur_seq, max_seq, updated})
        end

        return result
        "#;
        redis::Script::new(script)
            .prepare_invoke()
            .load(&mut conn)
            .unwrap()
    }

}


#[async_trait]
impl Cache for  RedisCache {
    async fn check_role_permission(&self, permission: i64)-> Result<bool, axum::http::Error> {
        todo!()
    }

    async fn get_role_permission(&self)->Result<Vec<String>, axum::http::Error> {
        todo!()
    }

    async fn save_role_permission(&self, role_id: i64,permissions: Vec<i64>)-> Result<(), axum::http::Error> {
        todo!()
    }

    async fn remove_role_permission(&self, role_id: i64)-> Result<(), axum::http::Error> {
        todo!()
    }
}