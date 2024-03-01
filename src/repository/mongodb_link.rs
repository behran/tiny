use futures::stream::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId},
    Client, Collection,
};
use mongodb::bson::doc;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};


use crate::config::Config;
use crate::models::link::Link;

#[derive(Clone, Debug)]
pub struct MongoRepo {
    col: Collection<Link>,
}


impl MongoRepo {
    pub async fn new(config: Config) -> Self {
        let dsn = &config.mongo.dsn;
        let db = Client::with_uri_str(dsn)
            .await
            .expect(format!("Failed to connect mongodb {dsn}").as_str())
            .database(&config.mongo.database);

        let col: Collection<Link> = db.collection::<Link>("links");

        MongoRepo { col }
    }

    pub async fn create_link(&self, data: Link) -> Result<InsertOneResult, Error> {
        let doc = Link {
            id: None,
            link: data.link,
            hash: data.hash,
        };
        let link = self
            .col
            .insert_one(doc, None)
            .await
            .ok()
            .expect("Error creating link");

        Ok(link)
    }

    pub async fn delete_link(&self, id: String) -> Result<DeleteResult, Error> {
        let object_id = ObjectId::parse_str(id).ok().expect("is nor mongoID");
        let delete_result = self
            .col
            .delete_one(doc! {"_id":object_id}, None)
            .await
            .ok()
            .expect("error deleting link");

        Ok(delete_result)
    }

    pub async fn update_link(&self, id: String, payload: Link) -> Result<UpdateResult, Error> {
        let object_id = ObjectId::parse_str(id).ok().expect("is nor mongoID");
        let filter = doc! {"_id":object_id};
        let update = doc! {
            "$set":{
                "link":payload.link
            }
        };
        let update_result = self
            .col
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("err update link");

        Ok(update_result)
    }

    pub async fn find_link(&self, id: String) -> Result<Link, Error> {
        let object_id = ObjectId::parse_str(id).ok().expect("is nor mongoID");
        let link = self
            .col
            .find_one(doc! {"_id":object_id}, None)
            .await
            .ok()
            .expect("Error get link detail");

        Ok(link.unwrap())
    }

    pub async fn get_all_links(&self) -> Result<Vec<Link>, Error> {
        let mut cursor = self
            .col
            .find(doc! {}, None)
            .await
            .ok()
            .expect("Error getting list of links");


        let mut links: Vec<Link> = Vec::new();
        while let Some(link) = cursor
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            links.push(link)
        }

        Ok(links)
    }
}
