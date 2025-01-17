
use crate::configs::reader_cfg::RedisConfig;
use crate::adapters::repository::{RepoHash, RepoClient};
use std::collections::BTreeMap;
use crate::domain::request::{Message};
use redis::{ RedisError};

pub fn get_hash(settings: &RedisConfig, req: RepoHash) -> BTreeMap<String, String>{
    let client = RepoClient::new(settings);
    let repo = RepoClient{
        db: client.unwrap(),

    };
    let resp = RepoHash::get(req.key,repo).unwrap();

    resp.value
}

pub fn set_hash(settings: &RedisConfig, req: RepoHash) -> Result<(), RedisError> {
    let client = RepoClient::new(settings);
    let repo = RepoClient{
        db: client.unwrap(),

    };
    let resp = RepoHash::set(req,repo);
    resp

}
pub fn map_repo_hash( k:String) ->RepoHash{
    RepoHash{
        value: Default::default(),
        key: k,
        ttl: 0
    }
}
pub fn map_payload_to_repo_hash(m: &Message, k:String) ->RepoHash{
    let v:BTreeMap<String,String>= m.m_hash.clone().unwrap();

    RepoHash{
        value: v,
        key: k,
        ttl: m.ttl.clone()
    }
}